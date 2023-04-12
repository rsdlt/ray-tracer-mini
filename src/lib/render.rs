#![allow(unused_assignments, clippy::write_with_newline)]

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::HittableList;
use crate::ray::Ray;
use crate::shapes::sphere::Sphere;
use crate::utilities::{random_float, INFINITY};
use crate::vector::Point3;

fn ray_color(ray: &Ray, world: &HittableList, depth: usize) -> Color {
    // Recursion base case: if exceeded the ray bounce limit, no more light is gathered
    if depth == 0 {
        return Color::black();
    }

    if let Some(hit) = world.hit(&ray, 0.0, INFINITY) {
        let target = hit.p + hit.normal + Point3::random_int_unit_sphere();
        return 0.5 * ray_color(&Ray::new(hit.p, target - hit.p), world, depth - 1);

        // return 0.5 * (Color::from(hit.normal) + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction().to_unit();
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

pub fn render() -> Result<File, std::io::Error> {
    // Camera
    let camera = Camera::new();

    // Image
    let aspect_ratio = camera.aspect_ratio();
    let image_width = 400_usize;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100_usize;
    let max_depth = 50_usize;

    // World
    let sph1 = Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    let sph2 = Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));
    let mut world = HittableList::new(sph1);
    world.add(sph2);

    // Render
    let path = Path::new("image.ppm");
    let mut img_file = File::create(path)?;
    let mut line = format!("P3\n{} {} \n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::black();

            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_float()) / (image_width as f64 - 1.0);
                let v = (j as f64 + random_float()) / (image_height as f64 - 1.0);
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &world, max_depth);
            }
            Color::write_color(&mut line, &pixel_color, samples_per_pixel);
        }
    }
    write!(img_file, "{}", line)?;

    Ok(img_file)
}
