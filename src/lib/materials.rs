pub mod lambertian;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub trait Material {
    fn scatter(r_in: &Ray, rec: &HitRecord, attenuation: Color, scattered: &Ray) -> bool;
}
