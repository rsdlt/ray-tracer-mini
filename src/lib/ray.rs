//! This module defines the Ray type, its functions and methods.

#![warn(missing_docs, missing_debug_implementations)]

use crate::vector::{Point3, Vec3};

/// The Ray type that contains a Point3 Origin and a Vec3 Direction.
#[derive(Debug)]
pub struct Ray {
    pub(crate) orig: Point3,
    pub(crate) dir: Vec3,
}

impl Ray {
    /// Function that calculates and return a Point with the position of the Ray
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }

    /// Function that creates and return an owned Ray.
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    /// Function that returns the direction component of a Ray.
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    /// Function that returns the origin component of a Ray.
    pub fn origin(&self) -> Point3 {
        self.orig
    }
}
