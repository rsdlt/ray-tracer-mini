use crate::color::Color;
use crate::hittable::HitRecord;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Dielectric {
    // index of refraction
    pub ir: f64,
}
impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::white();

        let refraction_ratio = match rec.front_face {
            true => 1.0 / self.ir,
            false => self.ir,
        };
        let unit_direction = r_in.direction().to_unit();
        let refracted = Vec3::refract(unit_direction, rec.normal, refraction_ratio);

        *scattered = Ray::new(rec.p, refracted);
        true
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(*self)
    }
}
