#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, RwLock};
use std::net::SocketAddr;

use tauri::*;
use tokio::net::{TcpListener, TcpStream};
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{accept_async};
use tungstenite::{Result, Message};

use paro_rs::{ParoApp};

mod state;
mod router;
mod pages;
use state::*;
use router::*;

/**
 * Start a websocket server for pâro to connect to
 */
async fn start_server() {
    let addr = "127.0.0.1:36432".to_string();
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    let paro_app = Arc::new(RwLock::new(ParoApp::<ApplicationState>::new(ApplicationState::default())));

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
    let rendered_html = render_page(&mut paro_app.clone());
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
                let rendered_html = render_page(&mut paro_app.clone());
                // send updated html to the client, so it can be shown to the user
                ws_stream.send(Message::Text(rendered_html)).await?;
            }
        }
    }

    Ok(())
}

/**
 * normal tauri main with TauriWebsocket added
 */
fn main() {
    tauri::async_runtime::spawn(start_server());
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.set_focus().unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
