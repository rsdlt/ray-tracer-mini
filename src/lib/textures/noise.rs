//! The noise texture.

use crate::color::Color;
use crate::textures::perlin::Perlin;
use crate::textures::Textures;
use crate::vector::Point3;

/// Noise texture.
#[derive(Debug, Clone)]
pub struct Noise {
    noise: Perlin,
    scale: f64,
}

impl Noise {
    /// Creates and returns a new owned noise texture.
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Textures for Noise {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        // Color::white() * 0.5 * (1.0 + self.noise.noise(p * self.scale))
        Color::white() * self.noise.turbulence(p * self.scale, 7)
    }
}
