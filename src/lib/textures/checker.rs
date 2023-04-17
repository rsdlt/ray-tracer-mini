//! The checker texture.

#![warn(missing_docs, missing_debug_implementations)]

use crate::color::Color;
use crate::textures::solid_color::SolidColor;
use crate::textures::{Texture, Textures};
use crate::vector::Point3;
use std::sync::Arc;

/// Checker texture type.
#[derive(Debug, Clone)]
pub struct Checker {
    /// Texture on the 'odd' checker.
    pub odd: Arc<Texture>,
    /// Texture on the 'even' checker.
    pub even: Arc<Texture>,
}

impl Checker {
    /// Creates and returns a new owned checker texture.
    pub fn new(color_even: Color, color_odd: Color) -> Self {
        Self {
            even: Arc::new(Texture::SolidColor(SolidColor::new(color_even))),
            odd: Arc::new(Texture::SolidColor(SolidColor::new(color_odd))),
        }
    }
}
impl Textures for Checker {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
