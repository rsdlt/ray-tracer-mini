//! This module defines the implementation of Axis-Aligned Bounding Boxes (AABBs)

#![warn(missing_docs, missing_debug_implementations)]

use crate::ray::Ray;
use crate::vector::Point3;

/// Type for AABBs
#[derive(Debug, Clone, Copy, Default)]
pub struct AaBb {
    /// Minimum 'slab' boundary.
    pub minimum: Point3,
    /// Maximum 'slab' boundary.
    pub maximum: Point3,
}

impl AaBb {
    /// Function that returns true if ray hits an AABB or false otherwise.
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let t0 = f64::min(
                (self.minimum[a] - ray.origin()[a]) / ray.direction()[a],
                (self.maximum[a] - ray.origin()[a]) / ray.direction()[a],
            );
            let t1 = f64::max(
                (self.minimum[a] - ray.origin()[a]) / ray.direction()[a],
                (self.maximum[a] - ray.origin()[a]) / ray.direction()[a],
            );
            let t_min = f64::max(t0, t_min);
            let t_max = f64::min(t1, t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    /// Creates and returns an owned AABB.
    pub fn new(a: Point3, b: Point3) -> Self {
        Self {
            minimum: a,
            maximum: b,
        }
    }
    /// Returns the minimum 'slab' boundary.
    pub fn min(&self) -> Point3 {
        self.minimum
    }

    /// Returns the maximum 'slab' boundary.
    pub fn max(&self) -> Point3 {
        self.maximum
    }

    /// Computes the bounding box for two boxes.
    pub fn surrounding_box(box0: AaBb, box1: AaBb) -> Self {
        let small = Point3::new(
            f64::min(box0.min().x, box1.min().x),
            f64::min(box0.min().y, box1.min().y),
            f64::min(box0.min().z, box1.min().z),
        );

        let big = Point3::new(
            f64::max(box0.max().x, box1.max().x),
            f64::max(box0.max().y, box1.max().y),
            f64::max(box0.max().z, box1.max().z),
        );

        AaBb::new(small, big)
    }
}
