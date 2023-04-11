#![allow(unused_assignments, clippy::write_with_newline)]

const IMG_WIDTH: usize = 256;
const IMG_HEIGHT: usize = 256;

use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn render() -> Result<File, std::io::Error> {
    let path = Path::new("../image.ppm");
    let mut img_file = File::create(path)?;

    let line = format!("P3\n{} {} \n255\n", IMG_WIDTH, IMG_HEIGHT);
    write!(img_file, "{}", line)?;

    let (mut rf, mut gf, mut bf): (f64, f64, f64) = (0.0, 0.0, 0.0);
    let (mut ri, mut gi, mut bi): (usize, usize, usize) = (0, 0, 0);
    for j in 0..IMG_HEIGHT {
        for i in 0..IMG_WIDTH {
            (rf, gf, bf) = (
                i as f64 / (IMG_WIDTH - 1) as f64,
                j as f64 / (IMG_HEIGHT - 1) as f64,
                0.25,
            );
            (ri, gi, bi) = (
                (255.99 * rf) as usize,
                (255.99 * gf) as usize,
                (255.99 * bf) as usize,
            );
            write!(img_file, "{} {} {}\n", ri, gi, bi)?;
        }
    }

    Ok(img_file)
}
