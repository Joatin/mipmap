use std::env;
use std::fs::{read, read_dir};
use std::path::Path;
use crate::image::imageops::FilterType;
use std::error::Error;
use crate::generate_mipmaps_and_save;

/// Generates mipmap for all png files within a folder
///
/// Meant to be used within a buildscript to process an entire folder
///
/// # Examples
///
/// ```rust
/// use std::fs::read_dir;
/// use mipmap::generate_mipmap_for_path_and_save_to_out_dir;
/// use mipmap::image::imageops::FilterType;
///
/// fn main() {
///     println!("cargo:rerun-if-changed=assets/");
///     println!("cargo:rerun-if-changed=build.rs");
///
///     generate_mipmap_for_path_and_save_to_out_dir(paths, FilterType::Gaussian).unwrap();
/// }
/// ```
pub fn generate_mipmap_for_path_and_save_to_out_dir(input_dir: &str, filter_type: FilterType) -> Result<(), Box<dyn Error + Send + Sync>> {
    let out_dir = env::var("OUT_DIR")?;
    let in_dir = env::var("CARGO_MANIFEST_DIR")?;
    let paths = read_dir(Path::new(&in_dir).join(input_dir))?;

    for path in paths {
        let path = path?;
        if path.file_name().into_string().unwrap().ends_with(".png") {
            let image_name = path.file_name().into_string().unwrap();
            println!("Generating mip maps for {}", image_name);
            let image = read(path.path())?;
            let dest_path = Path::new(&out_dir);
            let total = generate_mipmaps_and_save(&image, filter_type, &image_name, dest_path)?;
            println!("Created {} layers for {}", total, image_name);
        }
    }
    Ok(())
}