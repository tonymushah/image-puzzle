use image::DynamicImage;
use nalgebra::DMatrix;
use crate::Result;

use crate::game::dim::GameDimension;

pub fn split_image(image: DynamicImage, size: GameDimension) -> Result<DMatrix<DynamicImage>> {
    let mut res: DMatrix<DynamicImage> = size.into();
    let (row, column) = size.into();
    let x_size = <u32 as TryInto<usize>>::try_into(image.width())? / column;
    let y_size = <u32 as TryInto<usize>>::try_into(image.height())? / row;
    for x in 0..column {
        for y in 0..row {
            res[(x, y)] = image.crop_imm((x * x_size).try_into()?, (y * y_size).try_into()?, x_size.try_into()?, y_size.try_into()?);
        }
    }
    Ok(res)
}