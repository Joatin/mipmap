

pub struct MipmapLayer<'a> {
    pub data: &'a [u8],
    pub level: usize
}

impl<'a> MipmapLayer<'a> {
    pub const fn new(level: usize, data: &'a [u8]) -> Self {
        Self {
            level,
            data
        }
    }
}