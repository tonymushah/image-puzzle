use tauri::{Runtime, Window};

use crate::{error::Error, plugin::state::GameStates, Result};

#[tauri::command(rename_all = "snake_case")]
pub async fn save_party<R: Runtime>(
    window: Window<R>,
    state: tauri::State<'_, GameStates>,
) -> Result<()> {
    let state_read = state.read().await;
    let party = state_read.get(window.label()).ok_or(Error::GameNotLoaded)?;
    party.save_party()?;
    Ok(())
}
