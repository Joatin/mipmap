use crate::mipmap_layer::MipmapLayer;

#[derive(Debug)]
pub struct Mipmap<'a, const LAYERS: usize> {
    layers: &'a [MipmapLayer<'a>; LAYERS]
}

impl<'a, const LAYERS: usize> Mipmap<'a, LAYERS> {
    pub const fn new(layers: &'a [MipmapLayer; LAYERS]) -> Self {
        Self {
            layers
        }
    }

    /// Returns the raw png data
    pub fn get_raw(&'a self, level: usize) -> Option<&'a [u8]> {
        self.layers.iter().find(|i| i.level == level).map(|i| i.data)
    }

    /// Returns the unpacked image data as rgba8
    pub fn get(&'a self, level: usize) -> Option<Vec<u8>> {
        self.get_raw(level).map(|data| {
            let image = image::load_from_memory(data).unwrap();
            image.into_rgba8().into_vec()
        })
    }

    pub const fn num_layers(&'a self) -> usize {
        LAYERS
    }

    pub fn get_all(&'a self) -> Vec<u8> {
        self.layers.iter().map(|layer| {
            let image = image::load_from_memory(layer.data).unwrap();
            image.into_rgba8().into_vec()
        }).flatten().collect()
    }
}


