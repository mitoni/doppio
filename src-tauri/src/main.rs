// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;

mod commands;
mod server;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let boxed_handle = Box::new(handle);

            thread::spawn(move || {
                server::init(*boxed_handle).expect("failed to start the thread");
            });

            return Ok(());
        })
        .invoke_handler(tauri::generate_handler![commands::find_my_ip])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
