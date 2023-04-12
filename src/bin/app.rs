use raytracer::render::render;
use std::time::SystemTime;

fn main() {
    println!("Rendering image now!");
    let start = SystemTime::now();

    match render() {
        Ok(img_file) => println!(
            "image created {:?}",
            img_file.metadata().expect("metadata error")
        ),
        Err(err) => println!("error: {:?}", err),
    }

    println!("End! Now: {:?}", SystemTime::now());

    let elapsed = start
        .elapsed()
        .expect("Error calculating SystemTime::elapsed");

    println!("Total rendering time: {:?}", elapsed);
}
