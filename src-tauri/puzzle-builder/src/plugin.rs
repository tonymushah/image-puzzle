pub mod commands;
pub mod state;

use commands::{
    get_current::{__cmd__get_current, get_current},
    get_image::{__cmd__get_image, get_image},
    get_moves::{__cmd__get_moves, get_moves},
    have_won::{__cmd__have_won, have_won},
    init::{__cmd__init, init},
    swap::{__cmd__swap, swap},
};
use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

use self::state::GameStates;

pub fn get_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("image-puzzle")
        .invoke_handler(generate_handler![
            init,
            get_current,
            swap,
            have_won,
            get_image,
            get_moves
        ])
        .setup(|app| {
            app.manage(GameStates::default());
            Ok(())
        })
        .build()
}
