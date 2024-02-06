use std::num::TryFromIntError;
use image::error::ImageError;

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
    TomlDeError(#[from] toml::de::Error)
}

