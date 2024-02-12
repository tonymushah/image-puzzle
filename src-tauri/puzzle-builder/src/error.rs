use image::error::ImageError;
use serde::Serialize;
use std::num::TryFromIntError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    TryFromIntError(#[from] TryFromIntError),
    #[error(transparent)]
    ImageError(#[from] ImageError),
    #[error("Error on extracting in extension on a path")]
    ExtractPathExtensionError,
    #[error("Error on transfroming `OSStr` or a `Path` to `str`")]
    TostrError,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    TomlSerError(#[from] toml::ser::Error),
    #[error(transparent)]
    TomlDeError(#[from] toml::de::Error),
    #[error("An game is already loaded")]
    GameAlreadyLoaded,
    #[error("Any game is not loaded for this window")]
    GameNotLoaded,
    #[error("The given matrix is not square")]
    NotSquareMatrix,
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}
