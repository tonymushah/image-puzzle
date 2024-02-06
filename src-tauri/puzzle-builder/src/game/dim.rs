use nalgebra::{DMatrix, Scalar};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum GameDimension {
    Square(usize),
    Rectangle{
        lines: usize,
        columns: usize
    }
}

impl GameDimension {
    pub fn new_unit(size: usize) -> Self {
        Self::Square(size)
    }
    pub fn new_rec(lines: usize, columns: usize) -> Self {
        Self::Rectangle { lines, columns }
    }
}

impl From<GameDimension> for (usize, usize) {
    fn from(value: GameDimension) -> Self {
        match value {
            GameDimension::Square(size) => (size, size),
            GameDimension::Rectangle { lines, columns } => (lines, columns),
        }
    }
}

impl<T: Scalar + Default> From<GameDimension> for DMatrix<T> {
    fn from(value: GameDimension) -> Self {
        let (ncols, nrows) = From::from(value);
        DMatrix::<T>::from_element(nrows, ncols, Default::default())
    }
}