use crate::image::imageops::FilterType;
use crate::generate_mipmaps::generate_mipmaps;
use crate::image::ImageFormat;
use std::path::Path;

pub fn generate_mipmaps_and_save(original_image: &[u8], filter_type: FilterType, image_name: &str, save_path: Path) {
    let images = generate_mipmaps(original_image, filter_type);

    for (index, image) in images.into_iter().enumerate() {
        let path = save_path.join(format!("{}_{}.png", image_name, index));
        image.save_with_format(path, ImageFormat::Png);
    }
}