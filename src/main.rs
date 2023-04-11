const IMG_WIDTH: usize = 256;
const IMG_HEIGHT: usize = 256;

use std::fs::File;
use std::io::Write;
use std::path::Path;

fn render() -> Result<File, std::io::Error> {
    let path = Path::new("image.ppm");
    let mut img_file = File::create(path)?;

    let line = format!("P3\n {} {} \n255\n", IMG_WIDTH, IMG_HEIGHT);
    write!(img_file, "{}", line)?;

    Ok(img_file)
}

fn main() {
    render();
}
