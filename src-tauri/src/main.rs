// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use futures_util::{SinkExt, StreamExt};
use log::info;
use tauri::Manager;
use tauri_plugin_sql::{Migration, MigrationKind};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Result};
use window_shadows::set_shadow;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

async fn start_server() {
    let addr = "127.0.0.1:9002".to_string();

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }
}

async fn accept_connection(stream: TcpStream) -> Result<()> {
    let mut ws_stream = accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            info!("Received a message: {}", msg.to_text().unwrap());
            ws_stream.send(msg).await?;
        }
    }
    // let (mut write, read) = ws_stream.split();

    // if let Err(e) = read.forward(write).await {
    //     eprintln!("Error: {}", e);
    // }

    Ok(())
}

fn main() {
    env_logger::init();

    tauri::async_runtime::spawn(start_server());


    // run tauri
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            set_shadow(&window, true).expect("Unsupported platform!");
            Ok(())
        })
        .plugin(tauri_plugin_sql::Builder::default()
        .add_migrations("sqlite:database.db", vec![
            // Define your migrations here
            Migration {
                version: 1,
                description: "create_initial_tables",
                sql: "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);",
                kind: MigrationKind::Up,
            }
        ])
        .build())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
