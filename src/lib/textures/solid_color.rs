//! The solid color texture.

#![warn(missing_docs, missing_debug_implementations)]

use crate::color::Color;
use crate::textures::Textures;
use crate::vector::Point3;

/// A solid color texture.
#[derive(Debug, Clone, Copy)]
pub struct SolidColor {
    /// Color for the solid color texture.
    color_value: Color,
}

impl SolidColor {
    /// Creates and returns an owned Solid Color texture.
    pub fn new(color: Color) -> Self {
        Self { color_value: color }
    }
}

impl Textures for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.color_value
    }
}
