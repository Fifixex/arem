// Copyright 2024 the Arem Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Arem is a Rust crate designed for generating thumbnails from multimedia files.
//!
//! ## Overview
//!
//! Arem simplifies the process of extracting thumbnails from video and audio files,
//! supporting a wide range of formats. Its primary use case is media processing pipelines,
//! where generating quick previews is essential.
//!
//! The crate leverages efficient media decoding libraries and is designed to be
//! lightweight and fast. Arem aims to provide both high-level utilities for quick
//! integration and low-level APIs for fine-grained control.
//!
//! ## Key Features
//!
//! - Support for popular video and audio formats (e.g., MP4, MKV, MP3, etc.).
//! - Customizable thumbnail extraction parameters such as frame position and resolution.
//! - Built-in resizing and format conversion.
//! - Asynchronous API for non-blocking operations.
//! - Support for embedded frames (album art in audio files).
//!
//! ## Scope and Goals
//!
//! Arem focuses on simplifying media thumbnail generation, emphasizing:
//!
//! - Efficiency: Minimized resource usage for fast thumbnail extraction.
//! - Compatibility: Broad format support with modern multimedia standards.
//! - Ease of Use: A clear and simple API for quick integration into projects.
//!
//! This crate is not intended for full-fledged media editing or complex processing.
//! It specializes in extracting representative frames or images for preview purposes.
//!
//! ## Example Usage
//!
//! ```rust
//! use arem::ThumbnailGenerator;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let generator = ThumbnailGenerator::new();
//!     let thumbnail = generator
//!         .generate_thumbnail("example.mp4")
//!         .resolution(320, 240)
//!         .quality(80)
//!         .extract()
//!         .await?;
//!
//!     thumbnail.save("thumbnail.webp")?;
//!     Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! - `ffmpeg-sys`: Enables support for FFmpeg-based decoding (enabled by default).
//! - `image`: Provides utilities for image manipulation.
//! - `tokio`: Adds async support for seamless integration in async environments.
//!
//! ## Integration with Ecosystem
//!
//! Arem works seamlessly with crates like `image` for further image processing
//! and `tokio` for asynchronous workflows. It is designed to be extendable,
//! allowing developers to plug in their own processing steps or integrate with
//! other media handling libraries.
//!
//! ## Contribution
//!
//! Contributions are welcome! Whether you find a bug, want to add a new feature,
//! or improve the documentation, feel free to open an issue or submit a pull request.
//!
//! ## Licensing
//!
//! Arem is dual-licensed under Apache 2.0 and MIT. You can choose the license
//! that best suits your needs.

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
