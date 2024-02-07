use std::{collections::HashMap, ops::Deref, sync::Arc};

use tauri::async_runtime::RwLock;

use crate::game::party::GameParty;

pub type GameStateInner = Arc<RwLock<HashMap<String, GameParty>>>;

#[derive(Debug, Clone)]
pub struct GameStates(GameStateInner);

impl Default for GameStates {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(HashMap::new())))
    }
}

impl Deref for GameStates {
    type Target = GameStateInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
