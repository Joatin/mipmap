use crate::mipmap_layer::MipmapLayer;

pub struct Mipmap {
    layers: &'static [MipmapLayer]
}

impl Mipmap {
    pub fn new(layers: &'static [MipmapLayer]) -> Self {
        Self {
            layers
        }
    }

    /// Returns the raw png data
    pub fn get_raw(&self, level: usize) -> Option<&'static [u8]> {
        self.layers.iter().find(|i| i.level == level).map(|i| i.data)
    }

    /// Returns the unpacked image data as rgba8
    pub fn get(&self, level: usize) -> Option<Vec<u8>> {
        self.get_raw(level).map(|data| {
            let image = image::load_from_memory(data).unwrap();
            image.into_rgba8().into_vec()
        })
    }
}

