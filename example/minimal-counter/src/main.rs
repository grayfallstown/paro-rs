#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, RwLock};
use std::net::SocketAddr;

use tauri;
use tauri_plugin_websocket::TauriWebsocket;
use tokio::net::{TcpListener, TcpStream};
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{accept_async};
use tungstenite::{Result, Message};

use maud::{html};

use paro_rs::{ParoApp, event};

/**
 * State of your pâro application.
 * Can contain anything you need to hold as state and is
 * available in all callbacks.
 * No need to be serializable, clonable or anything. You can
 * have network connections and open file handles in here.
 */
pub struct ApplicationState {
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

    let paro_app = Arc::new(RwLock::new(ParoApp::<ApplicationState>::new(ApplicationState {
        current_count: 0,
    })));

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        tokio::spawn(accept_connection(paro_app.clone(), peer, stream));
    }
}

/**
 * Accept a connection and forward to handle_connection
 */
async fn accept_connection(paro_app: Arc<RwLock<ParoApp<ApplicationState>>>, peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(paro_app, peer, stream).await {
        match e {
            err => println!("Error processing connection: {}", err),
        }
    }
}

/**
 * This is where we do the server side work for your application
 */
async fn handle_connection(paro_app: Arc<RwLock<ParoApp<ApplicationState>>>, peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    println!("New WebSocket connection: {}", peer);

    // initial html
    let rendered_html = render_with_format(&mut paro_app.clone());
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
                
                loop {
                    match paro_app.try_write() {
                        Ok(ref mut data) => {
                            data.call(event_id.to_owned())
                                .expect(&format!("could not call paro callback for id '{}'", event_id));
        
                            // clean up old callbacks to free memory
                            data.iterate();
                            
                            break;
                        }
                        _ => {
                            std::thread::sleep(std::time::Duration::from_millis(1));
                        },
                    }
                }
                // render updated html and fill callbackstore with current callbacks
                let rendered_html = render_with_maud(&mut paro_app.clone());
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
fn render_with_format(paro_app: &mut Arc<RwLock<ParoApp<ApplicationState>>>) -> String {
    // for the button
    let increase_counter = event!(paro_app, (move |state: &mut ApplicationState, _| {
        // this is executed here in tauri and not in the gui client application
        state.current_count += 1;
        println!("state.current_count updated to: {}", state.current_count);
    }));

    // for the input field
    let set_counter = event!(paro_app, (move |state: &mut ApplicationState, value: Option<String>| {
        match value {
            None => state.current_count = 0,
            Some(val) => {
                match val.parse::<u64>() {
                    Err(_) => state.current_count = 0,
                    Ok(number) => state.current_count = number,
                }
            }
        }
        state.current_count += 1;
        println!("state.current_count updated to: {}", state.current_count);
    }));
    let html = format!(
        r#"<button onclick="{}">
            counter: {}
        </button>
        <input type="number" onchange="{}" value="{}" />"#,
            increase_counter,
            paro_app.read().unwrap().state.current_count,
            set_counter,
            paro_app.read().unwrap().state.current_count,
        );
    println!("format! generated html:\n{}", html);
    return html;
}

/**
 * Html rendering with a template engine. We are using maud here, as it is easy, has compile time checks
 * on the generated html and is very performant, but you can use whatever you prefer.
 */
fn render_with_maud(paro_app: &mut Arc<RwLock<ParoApp<ApplicationState>>>) -> String {
    // for the button
    let increase_counter = event!(paro_app, (move |state: &mut ApplicationState, _| {
        // this is executed here in tauri and not in the gui client application
        state.current_count += 1;
        println!("state.current_count updated to: {}", state.current_count);
    }));

    // for the input field
    let set_counter = event!(paro_app, (move |state: &mut ApplicationState, value: Option<String>| {
        match value {
            None => state.current_count = 0,
            Some(val) => {
                match val.parse::<u64>() {
                    Err(_) => state.current_count = 0,
                    Ok(number) => state.current_count = number,
                }
            }
        }
        state.current_count += 1;
        println!("state.current_count updated to: {}", state.current_count);
    }));
    let maud_template = html! {
        button onclick=(increase_counter) {
            "counter: " (paro_app.read().unwrap().state.current_count)
        }
        input type="number" value=(paro_app.read().unwrap().state.current_count) onchange=({set_counter}) {

        }
    };
    let html = maud_template.into_string();
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
