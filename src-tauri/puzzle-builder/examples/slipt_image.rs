use std::{fs::create_dir_all, path::Path};

use anyhow::{Error, Result};
use image::open;
use puzzle_builder::{game::dim::GameDimension, image_splitter::split_image};

fn main() -> Result<()> {
    let path = Path::new("./test-assets/marguerite-729510_640.jpg");
    let image_ = open(path)?;
    let images_ = split_image(image_, GameDimension::new_rec(3, 4))?;
    create_dir_all("./games/test-1")?;
    for (y, row) in images_.row_iter().enumerate() {
        for (x, colums) in row.column_iter().enumerate() {
            colums[(0, 0)].save(format!("./games/test-1/{x}-{y}.{}", path.extension().and_then(|d| d.to_str()).ok_or(Error::msg("Missing extension"))?))?;
        }
    }
    Ok(())
}