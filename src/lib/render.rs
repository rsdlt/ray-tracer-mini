#![allow(unused_assignments, clippy::write_with_newline)]

const IMG_WIDTH: usize = 256;
const IMG_HEIGHT: usize = 256;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::color::Color;
use crate::ray::Ray;
use crate::vector::{Point3, Vec3};

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(oc, r.direction());
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(r: Ray) -> Color {
    let mut t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &r);

    if t > 0.0 {
        let n = Vec3::unit(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }

    let unit_direction = r.direction().to_unit();
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
