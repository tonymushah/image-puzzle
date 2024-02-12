use std::{ops::Deref, path::PathBuf};

use crate::{
    error::Error,
    game::{party::GameParty, save::GameSave},
    plugin::state::GameStates,
    Result,
};
use tauri::{async_runtime::block_on, command, Runtime, State, Window, WindowEvent};

#[command(rename_all = "snake_case")]
pub async fn load<R: Runtime>(
    window: Window<R>,
    state: State<'_, GameStates>,
    path: PathBuf,
) -> Result<()> {
    let mut state_write = state.write().await;
    if state_write.get(window.label()).is_some() {
        Err(Error::GameAlreadyLoaded)
    } else {
        let game = GameParty::new(GameSave::load(path)?);
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

#[command(rename_all = "snake_case")]
pub async fn load_party<R: Runtime>(
    window: Window<R>,
    state: State<'_, GameStates>,
    path: PathBuf,
) -> Result<()> {
    let mut state_write = state.write().await;
    if state_write.get(window.label()).is_some() {
        Err(Error::GameAlreadyLoaded)
    } else {
        let game = GameParty::load_party(GameSave::load(path)?)?;
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
