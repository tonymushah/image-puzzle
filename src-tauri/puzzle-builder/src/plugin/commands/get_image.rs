use tauri::{Runtime, Window};

use crate::{error::Error, game::frame::GameFrame, plugin::state::GameStates, Result};

#[tauri::command(rename_all = "snake_case")]
pub async fn get_image<R: Runtime>(
    window: Window<R>,
    state: tauri::State<'_, GameStates>,
    frame: GameFrame,
) -> Result<Vec<u8>> {
    let state_read = state.read().await;
    let party = state_read.get(window.label()).ok_or(Error::GameNotLoaded)?;
    party.get_image_buf_by_frame(frame)
}
