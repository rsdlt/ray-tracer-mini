//! This module defines the Material trait and its associated functions.

#![warn(missing_docs, missing_debug_implementations)]

pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

/// The Material trait.
pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    /// This is a helper function to the implementation of the Clone trait for the Material trait objects..
    fn clone_box(&self) -> Box<dyn Material>;
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}
