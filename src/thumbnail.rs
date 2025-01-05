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

    /// Generates a thumbnail for the given video file and saves it to the specified output path.
    ///
    /// # Arguments
    ///
    /// * `input_path` - The path to the input video file.
    /// * `output_path` - The path to save the generated thumbnail.
    ///
    /// # Returns
    ///
    /// This function returns `Ok(())` if the thumbnail is generated successfully or
    /// an `Err(String)` describing the error if the operation fails.
    pub fn generate_thumbnail<P: AsRef<Path>>(
        &self,
        input_path: P,
        output_path: P,
    ) -> Result<(), String> {
        let input_path = input_path.as_ref();
        let output_path = output_path.as_ref();

        if !input_path.exists() {
            return Err(format!("Input file does not exist: {:?}", input_path));
        }

        let resolution = format!("{}x{}", self.resolution.0, self.resolution.1);

        // Construct and run the FFmpeg command.
        let status = Command::new("ffmpeg")
            .args([
                "-i",
                input_path.to_str().unwrap(),
                "-vf",
                &format!("scale={}", resolution),
                "-q:v",
                &self.quality.to_string(),
                "-frames:v",
                "1",
                output_path.to_str().unwrap(),
            ])
            .status();

        match status {
            Ok(status) if status.success() => Ok(()),
            Ok(status) => Err(format!(
                "FFmpeg failed with exit code: {}",
                status.code().unwrap_or(-1)
            )),
            Err(err) => Err(format!("Failed to execute FFmpeg: {}", err)),
        }
    }
}
