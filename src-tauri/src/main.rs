#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]   //esto para que pueda usarlo en javascript
fn hello_world(message: String) -> String {
    println!("me llamaron desde JS!!! Este es el mensaje {}", message);
    return "vengo desde rust!".into();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hello_world, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
