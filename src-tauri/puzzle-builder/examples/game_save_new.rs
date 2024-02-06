use anyhow::Result;
use puzzle_builder::game::{dim::GameDimension, save::GameSave};

fn main() -> Result<()> {
    let save = GameSave::new("./test-assets/marguerite-729510_640.jpg", "./games/test-3", GameDimension::new_rec(3, 4))?;
    let load = GameSave::load("./games/test-3")?;
    assert_eq!(save, load);
    Ok(())
}