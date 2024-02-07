pub mod columns;
pub mod rows;

use serde::Deserialize;
use tauri::{Runtime, Window};

use crate::{
    error::Error,
    game::{frame::GameFrame, party::GameParty},
    plugin::state::GameStates,
    Result,
};

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct SwapArgs {
    frame: GameFrame,
    target: GameFrame,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct SwapColArgArgs {
    pub(crate) current: usize,
    pub(crate) target: usize,
}

impl SwapArgs {
    pub fn swap(&self, party: &mut GameParty) {
        party.swap(self.frame, self.target);
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn swap<R: Runtime>(
    window: Window<R>,
    state: tauri::State<'_, GameStates>,
    args: SwapArgs,
) -> Result<()> {
    let mut state_write = state.write().await;
    state_write
        .get_mut(window.label())
        .map(|party| {
            args.swap(party);
        })
        .ok_or(Error::GameNotLoaded)?;
    Ok(())
}
