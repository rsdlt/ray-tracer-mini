//! The Textures module that contains the Texture trait and the list of textures supported..

pub mod checker;
pub mod noise;
pub mod perlin;
pub mod solid_color;

use crate::color::Color;
use crate::textures::checker::Checker;
use crate::textures::noise::Noise;
use crate::textures::solid_color::SolidColor;
use crate::vector::Point3;

/// Texture
pub trait Textures {
    /// Function that maps a Color based on u,v coordinates
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

/// List of textures
#[derive(Debug, Clone)]
pub enum Texture {
    /// The solid color texture variant.
    SolidColor(SolidColor),
    /// The checker texture variant.
    Checker(Checker),
    /// The noise texture variant.
    Noise(Noise),
}

impl Textures for Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        match self {
            Texture::SolidColor(solid_color) => solid_color.value(u, v, p),
            Texture::Checker(checker) => checker.value(u, v, p),
            Texture::Noise(noise) => noise.value(u, v, p),
        }
    }
}
