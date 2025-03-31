/**
 * This Angle struct is a wrapper around an f32 that represents an angle in radians. RADIANS!
 * The angle is always in the range (-π, π].
 * 0 radians point to the right! Why? Because everyone does it. 
 * 
 *             0.5π radians
 *                  ^
 *                  |
 *   π radians < -- O -- > 0 radians
 *                  |
 *                  v
 *            -0.5π radians
 * 
 **/

use std::f32::consts::PI;
use derive_more::{Add, Sub};

use super::vector2::Vector2;

#[derive(Debug, Clone, Copy, Add)]
pub struct Angle(pub f32);

/* Constructor methods */
impl Angle {
    pub const TWO_PI: Angle = Angle(2.0 * PI);
    pub const PI: Angle = Angle(PI);
    pub const PI_2: Angle = Angle(PI / 2.0);
    pub const PI_4: Angle = Angle(PI / 4.0);

    /// Wrap the angle to the range (-π, π]
    pub fn wrap(radians: f32) -> Self {
        let mut radians = radians % (2.0 * PI);
        if radians <= -PI {
            radians += 2.0 * PI;
        } else if radians > PI {
            radians -= 2.0 * PI;
        }
        Angle(radians)
    }

    pub fn from_radians(radians: f32) -> Self {
        Self::wrap(radians)
    }

    pub fn from_degrees(degrees: f32) -> Self {
        Self::wrap(degrees.to_radians())
    }
}

impl Default for Angle {
    fn default() -> Self {
        Self::from_radians(0.0)
    }
}


/* Mathematical methods */
impl Angle {

    /// Compute the smallest signed counter-clockwise angle from point a to point b.
    pub fn between_points(a: Vector2, b: Vector2) -> Self {
        let angle = (b.y() - a.y()).atan2(b.x() - a.x());
        Self::from_radians(angle)
    }

    /// Rotate a vector by this angle.
    // pub fn rotate_vector(&self, v: &Vector2) -> Vector2 {
    //     let rot = nalgebra::Rotation2::new(self.0);
    //     rot * v
    // }

    pub fn inv(&self) -> Self {
        Angle::wrap(-self.0)
    }

    pub fn abs(&self) -> Angle {
        Angle::wrap(self.0.abs())
    }

    pub fn to_vector(&self) -> Vector2 {
        Vector2::from_xy(self.0.cos(), self.0.sin())
    }
}

/* Operator methods */

// impl std::ops::Add for Angle {
//     type Output = Self;

//     fn add(self, other: Self) -> Self {
//         Angle::from_radians(self.0 + other.0)
//     }
// }

impl std::ops::Sub for Angle {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Angle::from_radians(self.0 - other.0)
    }
}

impl std::ops::Neg for Angle {
    type Output = Self;

    fn neg(self) -> Self {
        Angle::from_radians(-self.0)
    }
}

impl std::ops::AddAssign for Angle {
    fn add_assign(&mut self, other: Self) {
        self.0 = Angle::wrap(self.0 + other.0).0;
    }
}

impl std::ops::SubAssign for Angle {
    fn sub_assign(&mut self, other: Self) {
        self.0 = Angle::wrap(self.0 - other.0).0;
    }
}

// impl std::ops::Mul<Vector2> for Angle {
//     type Output = Vector2;

//     fn mul(self, v: Vector2) -> Vector2 {
//         self.rotate_vector(&v)
//     }
// }

impl std::ops::Mul<f32> for Angle {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Angle::from_radians(self.0 * scalar)
    }
}

impl std::ops::Div<f32> for Angle {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Angle::from_radians(self.0 / scalar)
    }
}

impl std::fmt::Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} rad", self.0)
    }
}



impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        let diff: f32 = (self.radians() - other.radians()).abs();
        const TOLERANCE: f32 = 1e-5; // about sqrt of f32 precision
        !(TOLERANCE..=(2.0 * PI - TOLERANCE)).contains(&diff)
    }
}

/* Approximation methods */

/* Indexing methods */
impl Angle {
    pub fn radians(&self) -> f32 {
        self.0
    }

    pub fn degrees(&self) -> f32 {
        self.0.to_degrees()
    }
}


#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_wrap_angle() {
        assert_eq!(Angle::wrap(0.0), Angle(0.0));
        assert_eq!(Angle::wrap(PI), Angle(PI));
        assert_eq!(Angle::wrap(-PI), Angle(PI));
        assert_eq!(Angle::wrap(3.0 * PI), Angle(PI));
        assert_eq!(Angle::wrap(-3.0 * PI), Angle(PI));
    }

    #[test]
    fn between_points() {
        let a = Vector2::from_xy(0.0, 0.0);
        let b = Vector2::from_xy(1.0, 1.0);
        let angle = Angle::between_points(a, b);
        assert_eq!(angle.degrees(), 45.0);
    }

    #[test]
    fn test_angle_add() {
        let a = Angle::from_degrees(90.0);
        let b = Angle::from_degrees(45.0);
        let c = a + b;
        assert_eq!(c.degrees(), 135.0);
    }

    #[test]
    fn test_angle_sub() {
        let a = Angle::from_degrees(90.0);
        let b = Angle::from_degrees(45.0);
        let c = a - b;
        assert_eq!(c.degrees(), 45.0);

        let a = Angle::from_degrees(-180.0);
        let b = Angle::from_degrees(180.0);
        assert_eq!((a - b).degrees(), 0.0);

        let a = Angle::from_degrees(180.0);
        let b = Angle::from_degrees(-179.0);
        assert_relative_eq!((a - b).degrees(), -1.0, epsilon = 1e-5);
    }

    #[test]
    fn test_angle_neg() {
        let a = Angle::from_degrees(90.0);
        let b = -a;
        assert_eq!(b.degrees(), -90.0);
    }

    #[test]
    fn test_angle_add_assign() {
        let mut a = Angle::from_degrees(90.0);
        let b = Angle::from_degrees(45.0);
        a += b;
        assert_eq!(a.degrees(), 135.0);
    }

    #[test]
    fn test_angle_sub_assign() {
        let mut a = Angle::from_degrees(90.0);
        let b = Angle::from_degrees(45.0);
        a -= b;
        assert_eq!(a.degrees(), 45.0);
    }

    // #[test]
    // fn test_angle_mul() {
    //     let a = Angle::from_degrees(90.0);
    //     let v = Vector2::from_xy(1.0, 0.0);
    //     let r = a * v;
    //     assert_eq!(r.y(), 1.0);
    // }

    #[test]
    fn test_flip_around_y_axis() {
        fn flip(angle: Angle) -> Angle {
            Angle::PI + angle
        }

        let a = Angle::from_degrees(90.0);
        assert_relative_eq!(flip(a).degrees(), -90.0, epsilon = 1e-5);

        let b = Angle::from_degrees(-90.0);
        assert_relative_eq!(flip(b).degrees(), 90.0, epsilon = 1e-5);

        let c = Angle::from_degrees(0.0);
        assert_relative_eq!(flip(c).degrees(), 180.0, epsilon = 1e-5);

        let c = Angle::from_degrees(180.0);
        assert_relative_eq!(flip(c).degrees(), 0.0, epsilon = 1e-5);
        let c = Angle::from_degrees(-180.0);
        assert_relative_eq!(flip(c).degrees(), 0.0, epsilon = 1e-5);

        let c = Angle::from_degrees(45.0);
        assert_relative_eq!(flip(c).degrees(), -135.0, epsilon = 1e-5);
    }

    #[test]
    fn test_angle_between_points() {
        let a = Vector2::from_xy(0.0, 0.0);
        let b = Vector2::from_xy(1.0, 1.0);
        let angle1 = Angle::between_points(a, b);
        assert_eq!(angle1.degrees(), 45.0);

        let angle2 = Angle::between_points(b, a);
        assert_eq!(angle2.degrees(), -135.0);
    }

    #[test]
    fn test_from_vector() {
        let v = Vector2::from_xy(-1.0, -1.0);
        let angle = v.to_angle();
        assert_relative_eq!(angle.degrees(), -135.0, epsilon = 1e-5);
    }
}