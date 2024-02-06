use std::{
    fs::{create_dir_all, File}, io::{BufReader, BufWriter, Read, Write}, ops::{Deref, DerefMut}, path::{Path, PathBuf}
};

use image::open;
use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::Error, image_splitter::split_image, Result};

use super::dim::GameDimension;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Deserialize, Serialize)]
pub struct GameSave {
    pub(crate) images: DMatrix<String>,
    #[serde(skip)]
    pub game_path: PathBuf,
}

impl Deref for GameSave {
    type Target = DMatrix<String>;
    fn deref(&self) -> &Self::Target {
        &self.images
    }
}

impl DerefMut for GameSave {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.images
    }
}

impl GameSave {
    pub fn save_file() -> &'static Path {
        Path::new("game.toml")
    }
    pub fn save(&self) -> Result<()> {
        let file = File::create(Path::new(&self.game_path).join(Self::save_file()))?;
        let fut_file_data = toml::to_string_pretty(&self)?;
        let mut buf_write_file = BufWriter::new(file);
        buf_write_file.write_all(fut_file_data.as_bytes())?;
        buf_write_file.flush()?;
        Ok(())
    }
    pub fn new<I: AsRef<Path>, G: AsRef<Path>>(
        image_path: I,
        game_path: G,
        size: GameDimension,
    ) -> Result<Self> {
        let extension = image_path
            .as_ref()
            .extension()
            .and_then(|d| d.to_str())
            .ok_or(Error::ExtractPathExtensionError)?;
        create_dir_all(&game_path)?;
        let image = open(&image_path)?;
        let images = split_image(image, size)?;
        let images_path = images.map_with_location(|_, _, _| format!("{}.{}", Uuid::new_v4(), extension));
        for (y, row) in images.row_iter().enumerate() {
            for (x, colums) in row.column_iter().enumerate() {
                println!("{x}-{y}");
                colums[(0, 0)].save(game_path.as_ref().to_path_buf().join(images_path[(y, x)].as_str()))?;
            }
        }
        let save = Self {
            images: images_path,
            game_path: game_path.as_ref().to_path_buf(),
        };
        save.save()?;
        Ok(save)
    }
    pub fn load<G: AsRef<Path>>(game_path: G) -> Result<Self> {
        let file = File::open(game_path.as_ref().join(Self::save_file()))?;
        let mut buf_reader = BufReader::new(file);
        let mut buf = String::new();
        buf_reader.read_to_string(&mut buf)?;
        let mut save: Self = toml::from_str(&buf)?;
        save.game_path = game_path.as_ref().to_path_buf();
        Ok(save)
    }
}
