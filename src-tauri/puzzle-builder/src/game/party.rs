use std::ops::{Deref, DerefMut};

use nalgebra::DMatrix;

use crate::utils::rand_matrix;

use super::{frame::GameFrame, save::GameSave};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct GameParty{
    game_save: GameSave,
    current: DMatrix<String>,
    moves: usize
}

impl Deref for GameParty {
    type Target = GameSave;
    fn deref(&self) -> &Self::Target {
        &self.game_save
    }
}

impl DerefMut for GameParty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.game_save
    }
}

impl GameParty {
    pub fn new(game_save: GameSave) -> GameParty {
        GameParty {
            current: rand_matrix(&game_save),
            game_save,
            moves: 0
        }
    }
    pub fn get_current(&self) -> &DMatrix<String> {
        &self.current
    }
    pub fn get_moves_cound(&self) -> usize {
        self.moves
    }
    pub fn swap(&mut self, frame: GameFrame, target: GameFrame){
        self.current.swap((frame.y, frame.x), (target.y, target.x));
        self.moves += 1;
    }
    pub fn have_won(&self) -> bool {
        self.images == self.current
    }
}