use crate::image::imageops::FilterType;
use crate::generate_mipmaps::generate_mipmaps;
use crate::image::ImageFormat;
use std::path::Path;
use std::fs::create_dir;
use std::error::Error;

pub fn generate_mipmaps_and_save(original_image: &[u8], filter_type: FilterType, image_name: &str, save_path: &Path) -> Result<usize, Box<dyn Error + Send + Sync>> {
    let images = generate_mipmaps(original_image, filter_type)?;
    let total = images.len();

    let _ = create_dir(save_path.join(format!("{}/", image_name)));
    for (index, image) in images.into_iter().enumerate() {
        let path = save_path.join(format!("{}/mip_{}.png", image_name, index));
        image.save_with_format(&path, ImageFormat::Png)?;
    }

    Ok(total)
}