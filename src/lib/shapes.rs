//! The Shapes module provides types and functionality for different shapes; for example, Spheres.

#![warn(missing_docs, missing_debug_implementations)]

use crate::aabb::AaBb;
use crate::bvh::BhvNode;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::shapes::moving_sphere::MovingSphere;
use crate::shapes::sphere::Sphere;

/// Module that contains the functionality of the Sphere shape.
pub mod sphere;

/// Module that contains the functionality of a Moving sphere shape.
pub mod moving_sphere;

/// Shape types that can be rendered.
#[derive(Debug, Clone)]
pub enum HittableObjects {
    /// Shapes of type Sphere.
    Sphere(Sphere),
    /// Shapes of type Sphere.
    MovingSphere(MovingSphere),
    /// BhvNode
    BhvNode(BhvNode),
}


impl Hittable for HittableObjects {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            HittableObjects::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
            HittableObjects::MovingSphere(sphere) => sphere.hit(ray, t_min, t_max),
            HittableObjects::BhvNode(node) => node.hit(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AaBb> {
        match self {
            HittableObjects::Sphere(sphere) => sphere.bounding_box(time0, time1),
            HittableObjects::MovingSphere(sphere) => sphere.bounding_box(time0, time1),
            HittableObjects::BhvNode(node) => node.bounding_box(time0, time1),
        }
    }
}
