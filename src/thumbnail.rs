#[derive(Debug, Clone)]
pub struct ThumbnailGenerator {
    quality: u8,
    resolution: (u32, u32),
}

impl Default for ThumbnailGenerator {
    fn default() -> Self {
        Self {
            quality: 80,
            resolution: (320, 240),
        }
    }
}

impl ThumbnailGenerator {
    /// Creates a new `ThumbnailGenerator` with default options.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quality(mut self, quality: u8) -> Self {
        if !(0..=100).contains(&quality) {
            panic!("Quality must be between 0 and 100, got {}", quality);
        }
        self.quality = quality;
        self
    }

    pub fn resolution(mut self, width: u32, height: u32) -> Self {
        if width == 0 || height == 0 {
            panic!("Resolution must be greater than 0, got {}x{}", width, height);
        }
        self.resolution = (width, height);
        self
    }
}
