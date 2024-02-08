use tauri::{Runtime, Window};

use crate::{error::Error, plugin::state::GameStates, Result};

// remember to call `.manage(MyState::default())`
#[tauri::command(rename_all = "snake_case")]
pub async fn reset<R: Runtime>(
    window: Window<R>,
    state: tauri::State<'_, GameStates>,
) -> Result<()> {
    let mut state_write = state.write().await;
    state_write
        .remove(window.label())
        .ok_or(Error::GameNotLoaded)?;
    Ok(())
}
