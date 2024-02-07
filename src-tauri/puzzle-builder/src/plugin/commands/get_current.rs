use nalgebra::DMatrix;
use tauri::{Runtime, Window};

use crate::{plugin::state::GameStates, Result};

#[tauri::command(rename_all = "snake_case")]
pub async fn get_current<R: Runtime>(
    window: Window<R>,
    state: tauri::State<'_, GameStates>,
) -> Result<Option<DMatrix<String>>> {
    let state_read = state.read().await;
    Ok(state_read
        .get(window.label())
        .map(|inner| inner.get_current().clone()))
}
