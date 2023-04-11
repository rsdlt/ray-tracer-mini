use raytracer::render::render;

fn main() {
    match render() {
        Ok(img_file) => println!(
            "image created {:?}",
            img_file.metadata().expect("metadata error")
        ),
        Err(err) => println!("error: {:?}", err),
    }
}
