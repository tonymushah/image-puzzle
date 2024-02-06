use std::ops::{Deref, DerefMut};

use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};

use super::save::GameSave;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct GameParty{
    game_save: GameSave,
    current: DMatrix<String>,
    moves: u32
}

impl Deref for GameParty {
    type Target = GameSave;
    fn deref(&self) -> &Self::Target {
        &self.game_save
    }
}

impl GameParty {
    pub fn new(game_save: GameSave) -> GameParty {
        todo!()
    }
}