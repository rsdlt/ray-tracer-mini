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
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }
}
