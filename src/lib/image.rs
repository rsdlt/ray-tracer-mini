//! This module defines an Image type and its properties

#![warn(missing_docs, missing_debug_implementations)]

/// Aspect ratio Width of the Image and the Camera.
pub const ASPECT_RATIO_WIDTH_DEFAULT: f64 = 1.0;
/// Aspect ratio Height of the Image and the Camera.
pub const ASPECT_RATIO_HEIGHT_DEFAULT: f64 = 1.0;
/// Aspect ratio of the Image and the Camera.
pub const ASPECT_RATIO_DEFAULT: f64 = ASPECT_RATIO_WIDTH_DEFAULT / ASPECT_RATIO_HEIGHT_DEFAULT;

/// Default Image width.
pub const IMAGE_WIDTH_DEFAULT: usize = 256;
/// Default Image height.
pub const IMAGE_HEIGHT_DEFAULT: usize = 256;

/// Default samples-per-pixel.
pub const SAMPLES_PER_PIXEL_DEFAULT: usize = 50;

/// Default max recursion depth to limit recursive ray tracing.
pub const MAX_DEPTH_DEFAULT: usize = 50;

/// The Image type.
#[derive(Debug, Copy, Clone)]
pub struct Image {
    /// Aspect ratio of an Image.
    pub aspect_ratio: f64,
    /// Width of an Image in pixels.
    pub width: usize,
    /// Height of an Image in pixels.
    pub height: usize,
    /// Number of color sample computations per pixel.
    pub samples_per_pixel: usize,
    /// Maximum recursion depth to generate the Image.
    pub max_depth: usize,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            aspect_ratio: ASPECT_RATIO_DEFAULT,
            width: IMAGE_WIDTH_DEFAULT,
            height: IMAGE_HEIGHT_DEFAULT,
            samples_per_pixel: SAMPLES_PER_PIXEL_DEFAULT,
            max_depth: MAX_DEPTH_DEFAULT,
        }
    }
}
impl Image {
    /// Creates and returns an owned Image.
    pub fn new(width: usize, height: usize, samples_per_pixel: usize, max_depth: usize) -> Self {
        Self {
            aspect_ratio: width as f64 / height as f64,
            width,
            height,
            samples_per_pixel,
            max_depth,
        }
    }
}
