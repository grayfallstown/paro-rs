#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

use tauri;
use tauri_plugin_websocket::TauriWebsocket;
use tokio::net::{TcpListener, TcpStream};
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{accept_async};
use tungstenite::{Result, Message};

use maud::{html};

use paro_rs::{CallbackStore, event};

/**
 * State of your pâro application.
 * Can contain anything you need to hold as state and is
 * available in all callbacks as Arc<Mutex<ApplicationState>>.
 * No need to be serializable, clonable or anything. You can
 * have network connections and open file handles in here.
 */
pub struct ApplicationState {
    /**
     * required.
     * Please note that this variable name is fixed and this api will
     * change soon to be a Trait that needs to be implemented for
     * ApplicationState
     */
    pub callback_store: CallbackStore,

    // whatever you need
    pub current_count: u64,
}


/**
 * Start a websocket server for pâro to connect to
 */
async fn start_server() {
    let addr = "127.0.0.1:1234".to_string();
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    let state = Arc::new(Mutex::new(ApplicationState {
        callback_store: CallbackStore::new(),
        current_count: 0,
    }));

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        tokio::spawn(accept_connection(state.clone(), peer, stream));
    }
}

/**
 * Accept a connection and forward to handle_connection
 */
async fn accept_connection(state: Arc<Mutex<ApplicationState>>, peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(state, peer, stream).await {
        match e {
            err => println!("Error processing connection: {}", err),
        }
    }
}

/**
 * This is where we do the server side work for your application
 */
async fn handle_connection(state: Arc<Mutex<ApplicationState>>, peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    println!("New WebSocket connection: {}", peer);

    // initial html
    let rendered_html = render_with_format(&mut state.clone());
    ws_stream.send(Message::Text(rendered_html)).await?;
    
    // You can have an eventloop here to match pâro message input, database returns result,
    // async api calls, etc

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        println!("got message from pâro");
        if msg.is_text() || msg.is_binary() {
            if msg.is_text() && msg.to_text().unwrap().eq("ping") {
                // ping / pong to keep the websocket alive while the user if afk
                ws_stream.send(Message::Text("pong".to_owned())).await?;
            } else {
                let event_id = msg.to_text().unwrap();
                println!("calling pâro event id '{}'", &event_id);
                let mut callback_store = state.lock().unwrap().callback_store.clone();
                callback_store.call(event_id.to_owned())
                    .expect(&format!("could not call paro callback for id '{}'", event_id));

                // clean up old callbacks to free memory
                state.lock().unwrap().callback_store.clear();
                // render updated html and fill callbackstore with current callbacks
                let rendered_html = render_with_maud(&mut state.clone());
                // send updated html to the client, so it can be shown to the user
                ws_stream.send(Message::Text(rendered_html)).await?;
            }
        }
    }

    Ok(())
}

/**
 * Pure html rendering without template engine. Has no compile time checks on
 * the generated html.
 */
fn render_with_format(state: &mut Arc<Mutex<ApplicationState>>) -> String {
     let html = format!(
        r#"<button onclick='{}'>
            counter: {}
        </button>"#, // we use single quotes on onclick, as event! returns a string with double quotes. maud handles that iself
            event!(state, (move |state| {
                // this is executed here in tauri and not in the gui client application
                let mut callback_state = state.lock().unwrap();
                callback_state.current_count += 1;
                println!("first number of state.numbers updated to: {}", callback_state.current_count);
            })),
            state.lock().unwrap().current_count
        );
    println!("format! generated html:\n{}", html);
    return html;
}

/**
 * Html rendering with a template engine. We are using maud here, as it has compile time checks
 * on the generated html, but you can use whatever you prefer.
 */
fn render_with_maud(state: &mut Arc<Mutex<ApplicationState>>) -> String {
    let maud_template = html! {
        button onclick=({
            event!(state, (move |state| {
                // this is executed here in tauri and not in the gui client application
                let mut callback_state = state.lock().unwrap();
                callback_state.current_count += 1;
                println!("first number of state.numbers updated to: {}", callback_state.current_count);
            }))
        }) { "counter:" (state.lock().unwrap().current_count) }
    };
    let html = maud_template.into_string();
    println!("maud generated html:\n{}", html);
    return html;
}


/**
 * normal tauri main with TauriWebsocket added
 */
fn main() {
    tauri::async_runtime::spawn(start_server());
    tauri::Builder::default()
        .plugin(TauriWebsocket::default()) // this was added
        .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
