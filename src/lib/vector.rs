const EPSILON: f64 = 0.001;

use derive_more::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Add, Div, Mul, Sub, Neg)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        if (self.x - other.x).abs() > EPSILON
            || (self.y - other.y).abs() > EPSILON
            || (self.y - other.y).abs() > EPSILON
        {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    #[rustfmt::skip]
    fn derive_add() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0, };
        let v2 = Vec3 { x: 1.0, y: 2.0, z: 3.0, };
        
        assert_eq!( v1 + v2, Vec3 { x: 2.0, y: 4.0, z: 6.0 } )
    }

    #[test]
    #[rustfmt::skip]
    fn derive_sub() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0, };
        let v2 = Vec3 { x: 1.0, y: 2.0, z: 3.0, };

        assert_eq!( v1 - v2, Vec3 { x: 0.0, y: 0.0, z: 0.0 } )
    }

    #[test]
    #[rustfmt::skip]
    fn derive_mul() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0, };
        let val = 2.0;

        assert_eq!( v1 * val, Vec3 { x: 2.0, y: 4.0, z: 6.0 } )
    }

    #[test]
    #[rustfmt::skip]
    fn derive_div() {
        let v1 = Vec3 { x: 2.0, y: 4.0, z: 6.0, };
        let val = 2.0;

        assert_eq!( v1 / val, Vec3 { x: 1.0, y: 2.0, z: 3.0 } )
    }

    #[test]
    #[rustfmt::skip]
    fn derive_neg() {
        let v1 = Vec3 { x: 2.0, y: 4.0, z: 6.0, };

        assert_eq!( -v1, Vec3 { x: -2.0, y: -4.0, z: -6.0 } )
    }
}
