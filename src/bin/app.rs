#![warn(missing_docs, missing_debug_implementations)]

/*!
 Ray Tracer Mini is a raytracer built following the  [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
 book by Peter Shirley.
*/

use raytracer::render::render;
use std::io::stdin;
use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();

    match render() {
        Ok(img_file) => {
            println!(
                "Total rendering time: {:5.2?}\nCheck your image at: ./image.ppm",
                start.elapsed().expect("Error calculating SystemTime")
            );
        }
        Err(err) => println!("Error: {:?}", err),
    }
    println!("\nPress ENTER to exit...");
    get_user_input();
}

fn get_user_input() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Unexpected error");
}
