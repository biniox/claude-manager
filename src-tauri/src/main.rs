// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod process_manager;
mod terminal_io;
mod types;

use process_manager::{get_process_manager, init_process_manager};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // Initialize the process manager
            init_process_manager(app.handle().clone());

            // Start autostart instances
            tauri::async_runtime::spawn(async move {
                if let Err(e) = commands::start_autostart_instances().await {
                    eprintln!("Failed to start autostart instances: {}", e);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            // Instance management
            commands::create_instance,
            commands::get_instances,
            commands::delete_instance_command,
            commands::update_instance_command,
            commands::get_autostart_instances_command,
            // Process control
            commands::start_instance,
            commands::stop_instance,
            commands::restart_instance,
            // Terminal I/O
            commands::write_to_terminal,
            commands::resize_terminal,
            commands::get_instance_status,
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                // Stop all running instances when window closes
                let process_manager = get_process_manager();
                tauri::async_runtime::spawn(async move {
                    process_manager.stop_all().await;
                });
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run()
}
