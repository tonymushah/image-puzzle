use tauri::{Runtime, Window};

use crate::{error::Error, plugin::state::GameStates, Result};

// remember to call `.manage(MyState::default())`
#[tauri::command(rename_all = "snake_case")]
pub async fn have_won<R: Runtime>(
    window: Window<R>,
    state: tauri::State<'_, GameStates>,
) -> Result<bool> {
    let state_read = state.read().await;
    state_read
        .get(window.label())
        .map(|party| party.have_won())
        .ok_or(Error::GameNotLoaded)
}
