//! This module defines the Material trait and its associated functions.

#![warn(missing_docs, missing_debug_implementations)]

pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::materials::{dielectric::DielectricMat, lambertian::LambertianMat, metal::MetalMat};
use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub enum Materials {
    Metal(MetalMat),
    Dielectric(DielectricMat),
    Lambertian(LambertianMat),
}

/// The Material trait.
pub trait Scatterable {
    /// Function that returns true if the material produces a scattered Ray. If scattered then, it
    /// indicates how much attenuation via mutation.
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

impl Scatterable for Materials {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Materials::Lambertian(lamb) => lamb.scatter(r_in, rec, attenuation, scattered),
            Materials::Metal(metal) => metal.scatter(r_in, rec, attenuation, scattered),
            Materials::Dielectric(die) => die.scatter(r_in, rec, attenuation, scattered),
        }
    }
}
