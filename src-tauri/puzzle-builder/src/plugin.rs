pub mod commands;
pub mod state;

use commands::{
    get_current::{__cmd__get_current, get_current},
    get_image::{__cmd__get_image, __cmd__get_image_by_path, get_image, get_image_by_path},
    get_moves::{__cmd__get_moves, get_moves},
    have_won::{__cmd__have_won, have_won},
    init::{__cmd__init, init},
    load::{__cmd__load, load},
    reset::{__cmd__reset, reset},
    swap::{
        __cmd__swap,
        columns::{__cmd__swap_columns, swap_columns},
        rows::{__cmd__swap_rows, swap_rows},
        swap,
    },
    transpose::{__cmd__transpose, transpose},
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
            get_image_by_path,
            get_moves,
            swap_rows,
            swap_columns,
            load,
            reset,
            transpose
        ])
        .setup(|app| {
            app.manage(GameStates::default());
            Ok(())
        })
        .build()
}
