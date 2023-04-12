#![allow(unused_assignments, clippy::write_with_newline)]

const IMG_WIDTH: usize = 256;
const IMG_HEIGHT: usize = 256;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::color::Color;
use crate::ray::Ray;
use crate::vector::Vec3;

fn ray_color(r: &Ray) -> Color {
    let unit_direction = Vec3::unit(r.dir);
    let t = 0.5 * (unit_direction.y + 1.0);

    Color::white() * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

pub fn render() -> Result<File, std::io::Error> {
    let path = Path::new("image.ppm");
    let mut img_file = File::create(path)?;

    let mut line = format!("P3\n{} {} \n255\n", IMG_WIDTH, IMG_HEIGHT);

    for j in (0..IMG_HEIGHT).rev() {
        for i in 0..IMG_WIDTH {
            let color = Color {
                r: (i as f64) / (IMG_WIDTH - 1) as f64,
                g: (j as f64) / (IMG_HEIGHT - 1) as f64,
                b: 0.25,
            };

            Color::write_color(&mut line, &color);
        }
    }
    write!(img_file, "{}", line)?;

    Ok(img_file)
}
