use super::vector::{Point3, Vec3};

#[derive(Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
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
