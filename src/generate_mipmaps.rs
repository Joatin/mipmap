use image::imageops::FilterType;
use image::DynamicImage;


pub fn generate_mipmaps(original_image: &[u8], filter_type: FilterType) -> Vec<DynamicImage> {
    let mut mipmaps = vec![image::load_from_memory(original_image).unwrap()];

    let mut new_width = mipmaps[0].width() / 2;
    let mut new_height = mipmaps[0].height() / 2;

    loop {
        let new_image = mipmaps[0].resize(new_width, new_height, filter_type);
        mipmaps.push(new_image);
        if new_height == 1 || new_width == 1 {
            break
        }
        new_width = new_width / 2;
        new_height = new_height / 2;
    }

    mipmaps
}

#[cfg(test)]
mod tests {
    use crate::generate_mipmaps;
    use crate::image::imageops::FilterType;

    #[test]
    fn it_should_return_correct_amount_of_mipmaps_and_the_right_sizes() {
        let result = generate_mipmaps(include_bytes!("rainbow_block.png"), FilterType::Gaussian);

        assert_eq!(6, result.len());
        assert_eq!(32, result[0].width());
        assert_eq!(16, result[1].width());
        assert_eq!(8, result[2].width());
        assert_eq!(4, result[3].width());
        assert_eq!(2, result[4].width());
        assert_eq!(1, result[5].width());
    }
}