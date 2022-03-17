

pub struct MipmapLayer {
    pub data: &'static [u8],
    pub level: usize
}

impl MipmapLayer {
    pub fn new(level: usize, data: &'static [u8]) -> Self {
        Self {
            level,
            data
        }
    }
}