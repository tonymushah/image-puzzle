use tauri::{Runtime, Window};

use crate::{error::Error, game::party::GameParty, plugin::state::GameStates, Result};

use super::SwapColArgArgs;

impl SwapColArgArgs {
    pub fn swap_columns(self, party: &mut GameParty) {
        party.swap_columns(self.current, self.target)
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn swap_columns<R: Runtime>(
    window: Window<R>,
    state: tauri::State<'_, GameStates>,
    args: SwapColArgArgs,
) -> Result<()> {
    let mut state_write = state.write().await;
    state_write
        .get_mut(window.label())
        .map(|party| {
            args.swap_columns(party);
        })
        .ok_or(Error::GameNotLoaded)?;
    Ok(())
}
