use crate::color::Color;
use crate::hittable::HitRecord;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(Vec3::unit(r_in.direction()), rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;

        Vec3::dot(scattered.direction(), rec.normal) > 0.0
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(*self)
    }
}
