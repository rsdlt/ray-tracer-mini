//! The perlin texture.

use crate::utilities::{random_float, random_usize_range};
use crate::vector::{Point3, Vec3};
use rand::Rng;

#[derive(Debug, Clone, Default)]
/// Perlin noise texture
pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn turbulence(&self, p: Point3, depth: usize) -> f64 {
        let mut accum = 0.0f64;
        let mut temp_p = p;
        let mut weight = 1.0;

        for i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }
        accum.abs()
    }

    /// New Perlin texture
    pub fn new() -> Self {
        Self {
            ranvec: Self::perlin_generate(),
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    /// Noise function
    pub fn noise(&self, p: Point3) -> f64 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();

        // u = u * u * (3.0 - 2.0 * u);
        // v = v * v * (3.0 - 2.0 * v);
        // w = w * w * (3.0 - 2.0 * w);

        let i = p.x.floor() as usize;
        let j = p.y.floor() as usize;
        let k = p.z.floor() as usize;

        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]]
                }
            }
        }
        Self::trilinear_interp(&c, u, v, w)
    }

    fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * Vec3::dot(c[i][j][k], weight);
                }
            }
        }

        accum
    }

    fn perlin_generate() -> Vec<Vec3> {
        let mut rng = rand::thread_rng();
        let mut p = Vec::with_capacity(Self::POINT_COUNT);
        for _ in 0..Self::POINT_COUNT {
            p.push(Vec3::new(
                -1.0 + 2.0 * rng.gen::<f64>(),
                -1.0 + 2.0 * rng.gen::<f64>(),
                -1.0 + 2.0 * rng.gen::<f64>(),
            ));
        }
        p
    }
    fn perlin_generate_perm() -> Vec<usize> {
        let mut p = Vec::with_capacity(Self::POINT_COUNT);
        for i in 0..Self::POINT_COUNT {
            p.push(i);
        }
        Self::permute(&mut p, Self::POINT_COUNT);
        p
    }

    fn permute(p: &mut [usize], n: usize) -> Vec<usize> {
        let mut rng = rand::thread_rng();
        rng.gen_range(0.0..=1.0);
        for i in (1..n).rev() {
            let target = rng.gen_range(0..=(i + 1));
            p.swap(i, target);
        }
        p.to_vec()
    }
}
