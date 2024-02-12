use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    ops::{Deref, DerefMut},
    path::Path,
};

use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};

use crate::{error::Error, utils::rand_matrix, Result};

use super::{frame::GameFrame, save::GameSave};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Deserialize, Serialize)]
pub struct GameParty {
    #[serde(skip)]
    game_save: GameSave,
    current: DMatrix<String>,
    moves: usize,
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
            moves: 0,
        }
    }
    pub fn get_current(&self) -> &DMatrix<String> {
        &self.current
    }
    pub fn get_moves_count(&self) -> usize {
        self.moves
    }
    pub fn swap(&mut self, frame: GameFrame, target: GameFrame) {
        self.current.swap((frame.y, frame.x), (target.y, target.x));
        self.moves += 1;
    }
    pub fn have_won(&self) -> bool {
        self.images == self.current
    }
    pub fn get_image_buf_by_frame(&self, frame: GameFrame) -> Result<Vec<u8>> {
        self.get_image_buf(&self.current[(frame.y, frame.x)])
    }
    pub fn swap_rows(&mut self, current: usize, target: usize) {
        self.current.swap_rows(current, target);
        self.moves += 1;
    }
    pub fn swap_columns(&mut self, current: usize, target: usize) {
        self.current.swap_columns(current, target);
        self.moves += 1;
    }
    pub fn transpose(&mut self) -> Result<()> {
        if self.current.nrows() != self.current.ncols() {
            Err(Error::NotSquareMatrix)
        } else {
            self.current.transpose_mut();
            self.moves += 1;
            Ok(())
        }
    }
    pub fn invert_rows(&mut self) {
        let nrows = self.current.nrows();
        for row in 0..(nrows / 2) {
            self.current.swap_rows(row, nrows - row - 1);
        }
        self.moves += 1;
    }
    pub fn invert_columns(&mut self) {
        let ncols = self.current.ncols();
        for col in 0..(ncols / 2) {
            self.current.swap_columns(col, ncols - col - 1);
        }
        self.moves += 1;
    }
    pub fn party_file() -> &'static Path {
        Path::new("party.toml")
    }
    pub fn save_party(&self) -> Result<()> {
        let file = File::create(self.game_path.join(Self::party_file()))?;
        let buf = toml::to_string(self)?;
        let mut buf_writer = BufWriter::new(file);
        buf_writer.write_all(buf.as_bytes())?;
        buf_writer.flush()?;
        Ok(())
    }
    pub fn load_party(save: GameSave) -> Result<Self> {
        let file = File::open(save.game_path.join(Self::party_file()))?;
        let mut buf = String::new();
        let mut buf_reader = BufReader::new(file);
        buf_reader.read_to_string(&mut buf)?;
        let mut party: Self = toml::from_str(&buf)?;
        party.game_save = save;
        Ok(party)
    }
}
