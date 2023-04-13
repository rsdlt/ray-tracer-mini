#![allow(unused_assignments, clippy::write_with_newline)]

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::HittableList;
use crate::materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use crate::ray::Ray;
use crate::shapes::sphere::Sphere;
use crate::utilities::{random_float, INFINITY};
use crate::vector::{Point3, Vec3};

fn ray_color(ray: &Ray, world: &HittableList, depth: usize) -> Color {
    // Recursion base case: if exceeded the ray bounce limit, no more light is gathered
    if depth == 0 {
        return Color::black();
    }

    if let Some(hit) = world.hit(&ray, 0.001, INFINITY) {
        // diffuse render 1: let target = hit.p + hit.normal + Point3::random_in_unit_sphere();
        // diffuse render 2: let target = hit.p + hit.normal + Point3::random_unit_vector();
        // diffuse render 3:  let target = hit.p + Point3::random_in_hemisphere(&hit.normal);
        // return 0.5 * ray_color(&Ray::new(hit.p, target - hit.p), world, depth - 1);

        let mut scattered = Ray::new(Point3::default(), Vec3::default());
        let mut attenuation = Color::black();
        if hit
            .material
            .scatter(&ray, &hit, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::black();
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
    let material_ground = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    // let material_center = Box::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    // let material_left = Box::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_center = Box::new(Dielectric::new(1.5));
    let material_left = Box::new(Dielectric::new(1.5));
    let material_right = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let sphere_1 = Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    let sphere_2 = Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    ));
    let sphere_3 = Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    let sphere_4 = Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));
    let mut world = HittableList::new(sphere_1);
    world.add(sphere_2);
    world.add(sphere_3);
    world.add(sphere_4);

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
