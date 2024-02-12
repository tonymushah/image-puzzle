use tauri::{Runtime, Window};

use crate::{error::Error, plugin::state::GameStates, Result};

#[tauri::command(rename_all = "snake_case")]
pub async fn invert_rows<R: Runtime>(
    window: Window<R>,
    state: tauri::State<'_, GameStates>,
) -> Result<()> {
    let mut state_write = state.write().await;
    let party = state_write
        .get_mut(window.label())
        .ok_or(Error::GameNotLoaded)?;
    party.invert_rows();
    Ok(())
}
