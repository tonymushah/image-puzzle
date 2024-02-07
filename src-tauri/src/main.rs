// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .plugin(puzzle_builder::get_plugin())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
