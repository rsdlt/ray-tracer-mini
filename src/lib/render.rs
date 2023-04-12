#![allow(unused_assignments, clippy::write_with_newline)]

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::color::Color;
use crate::ray::Ray;
use crate::vector::{Point3, Vec3};

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let origin_center = ray.origin() - center;
    let a = ray.direction().length_squared();
    let half_b = Vec3::dot(origin_center, ray.direction());
    let c = origin_center.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / (a)
    }
}

fn ray_color(ray: Ray) -> Color {
    let mut t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &ray);

    if t > 0.0 {
        let normal = Vec3::unit(ray.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
    }

    let unit_direction = ray.direction().to_unit();
    t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

pub fn render() -> Result<File, std::io::Error> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400_usize;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::zeroes();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render

    let path = Path::new("image.ppm");
    let mut img_file = File::create(path)?;
    let mut line = format!("P3\n{} {} \n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u: f64 = (i as f64) / (image_width as f64 - 1.0);
            let v: f64 = (j as f64) / (image_height as f64 - 1.0);
            let r = Ray::new(
                origin,
                lower_left_corner + (u * horizontal) + (v * vertical) - origin,
            );

            let color = ray_color(r);

            Color::write_color(&mut line, &color);
        }
    }
    write!(img_file, "{}", line)?;

    Ok(img_file)
}
