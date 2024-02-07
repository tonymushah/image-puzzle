use std::{ops::Deref, path::PathBuf};

use serde::Deserialize;
use tauri::{async_runtime::block_on, command, Runtime, State, Window, WindowEvent};

use crate::{
    error::Error,
    game::{dim::GameDimension, party::GameParty, save::GameSave},
    plugin::state::GameStates,
    Result,
};

#[derive(Debug, Clone, Deserialize)]
pub struct GameInitArgs {
    image_path: PathBuf,
    game_path: PathBuf,
    size: GameDimension,
}

impl GameInitArgs {
    pub fn init_game(self) -> Result<GameParty> {
        Ok(GameParty::new(GameSave::new(
            self.image_path,
            self.game_path,
            self.size,
        )?))
    }
}

#[command(rename_all = "snake_case")]
pub async fn init<R: Runtime>(
    window: Window<R>,
    state: State<'_, GameStates>,
    args: GameInitArgs,
) -> Result<()> {
    let mut state_write = state.write().await;
    if state_write.get(window.label()).is_some() {
        Err(Error::GameAlreadyLoaded)
    } else {
        let game = args.init_game()?;
        state_write.insert(window.label().into(), game);
        let re_state = state.deref().clone();
        let window_label = String::from(window.label());
        window.on_window_event(move |e| {
            let re_state = re_state.clone();
            let window_label = window_label.clone();
            if let WindowEvent::Destroyed = e {
                block_on(async move {
                    let mut re_state_write = re_state.write().await;
                    let _ = re_state_write.remove(&window_label);
                })
            }
        });
        Ok(())
    }
}
