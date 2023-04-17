//! This module defines the Material trait and its associated functions.

#![warn(missing_docs, missing_debug_implementations)]

pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use crate::ray::Ray;

/// Materials that can be applied to a Shape.
#[derive(Debug, Clone)]
pub enum Materials {
    /// Metal materials.
    Metals(Metal),
    /// Dielectric materials.
    Dielectrics(Dielectric),
    /// Lambertian materials.
    Lambertians(Lambertian),
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
            Materials::Lambertians(lamb) => lamb.scatter(r_in, rec, attenuation, scattered),
            Materials::Metals(metal) => metal.scatter(r_in, rec, attenuation, scattered),
            Materials::Dielectrics(die) => die.scatter(r_in, rec, attenuation, scattered),
        }
    }
}
