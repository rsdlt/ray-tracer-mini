//! This module defines the Lambertian Material and its implementation of the Material trait.

#![warn(missing_docs)]
#![allow(unused_variables)]

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::materials::Scatterable;
use crate::ray::Ray;
use crate::textures::{Texture, Textures};
use crate::vector::Vec3;

/// The Lambertian type with the albedo property.
#[derive(Clone, Debug)]
pub struct Lambertian {
    /// Proportion of incident light that is reflected away from the surface.
    pub albedo: Texture,
}
impl Lambertian {
    /// Function creates and returns an owned Lambertian material.
    pub fn new(texture: Texture) -> Self {
        Self { albedo: texture }
    }
}

impl Scatterable for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        // let mut scatter_direction = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        // Catch degenerate scatter direction
        // if scatter_direction.near_zero() {
        //     scatter_direction = rec.normal;
        // }

        *scattered = Ray::new(rec.p, target - rec.p, r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        true
    }
}
