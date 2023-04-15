//! This module defines the Material trait and its associated functions.

#![warn(missing_docs, missing_debug_implementations)]

pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utilities::SharedMaterial;
use parking_lot::Mutex;
use std::sync::Arc;

/// The Material trait.
pub trait Material {
    /// Function that returns true if the material produces a scattered Ray. If scattered then, it
    /// indicates how much attenuation via mutation.
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    /// Helper function to the implementation of the Clone trait for the Material trait objects..
    fn clone_box(&self) -> Arc<Mutex<Box<dyn Material + Send>>>;
}

// impl Clone for Arc<Mutex<Box<dyn Material>>> {
//     fn clone(&self) -> Arc<Mutex<Box<dyn Material>>> {
//         Arc::new(Mutex::new(self.clone_box()))
//     }
// }
