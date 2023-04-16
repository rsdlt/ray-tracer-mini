# Ray Tracer Mini
A simple ray tracer written in [Rust](https://www.rust-lang.org/) following the [_Ray Tracing in One Weekend_](https://github.com/rsdlt/ray-tracer-mini) book series by Peter Shirley.

The ray tracer generates a random scene filled with spheres and leverages [Rayon](https://github.com/rayon-rs/rayon) to deliver parallel computations. 

An image with the following configurations renders in ~382 seconds in a Core i7-12700H spawning ~20 threads:

- **W x H:** 1200 x 800
- **Samples per pixel:** 500
- **Recursion depth:** 50
- **Shapes:** 486

![Final Ray Traced Image](/img_history/image29.png)

Motion Blur:

![Ray Trace Motion Blur](/img_history/image30.png)

## Usage:

1. Build project: `cargo build --release`.
2. Configure ray tracer:
   - Edit `config.toml` to set image width, image height, recursion depth and samples per pixel.
   - Copy `config.toml` to target directory `target/release`.
3. Render image: `./target/release/app`.
4. View image: ` ./target/release/image.ppm`.




