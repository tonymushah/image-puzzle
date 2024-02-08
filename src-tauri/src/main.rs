// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;

use tauri::{CustomMenuItem, Menu, Window, WindowUrl};
use uuid::Uuid;

fn main() {
    let menu = Menu::new().add_item(CustomMenuItem::new("new-window", "New window"));
    tauri::Builder::default()
        .plugin(puzzle_builder::get_plugin())
        .menu(menu)
        .on_menu_event(|e| {
            if e.menu_item_id() == "new-window" {
                let _ = Window::builder(
                    e.window(),
                    Uuid::new_v4().to_string(),
                    WindowUrl::App(Path::new("/").to_path_buf()),
                )
                .title(String::from("Image Puzzle"))
                .build();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
