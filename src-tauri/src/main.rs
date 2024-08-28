#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mp3::get_music_cover;
mod mp3;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    get_music_cover("きのこ帝国 - School／Fiction.mp3".to_string());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
