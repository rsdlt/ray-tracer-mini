//! The Shapes module provides types and functionality for different shapes; for example, Spheres.

#![warn(missing_docs, missing_debug_implementations)]

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::shapes::sphere::Sphere;

/// Module that contains the functionality of the Sphere shape.
pub mod sphere;

#[derive(Debug, Clone, Copy)]
pub enum Shapes {
    Spheroids(Sphere),
}

impl Hittable for Shapes {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Shapes::Spheroids(sphere) => sphere.hit(ray, t_min, t_max),
        }
    }
}
