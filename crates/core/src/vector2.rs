use approx::{AbsDiffEq, RelativeEq};
use derive_more::{Add, AddAssign, Sub, SubAssign, Index};

use super::angle::Angle;

#[derive(Clone, Copy, Debug, PartialEq, Add, AddAssign, Sub, SubAssign, Index)]
pub struct Vector2(pub nalgebra::Vector2<f32>);

/* Constructor methods */
impl Vector2 {
    pub fn from_xy(x: f32, y:f32) -> Self {
        Self(nalgebra::Vector2::new(x, y))
    }

    pub fn from_polar(magnitude: f32, angle: f32) -> Self {
        Self(nalgebra::Vector2::new(magnitude * angle.cos(), magnitude * angle.sin()))
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Self(nalgebra::Vector2::new(0., 0.))
    }
}

/* Mathematical methods */
impl Vector2 {
    pub fn dot(&self, other: &Vector2) -> f32 {
        self.0.x * other.0.x + self.0.y * other.0.y
    }

    pub fn length2(&self) -> f32 {
        self.0.x.powi(2) + self.0.y.powi(2)
    }

    pub fn length(&self) -> f32 {
        self.length2().sqrt()
    }

    pub fn dist2(&self, other: &Vector2) -> f32 {
        (self.0.x - other.0.x).powi(2) + (self.0.y - other.0.y).powi(2)
    }

    pub fn dist(&self, other: &Vector2) -> f32 {
        self.dist2(other).sqrt()
    }

    pub fn scale(&self, scalar: f32) -> Vector2 {
        Vector2::from_xy(self.0.x * scalar, self.0.y * scalar)
    }

    pub fn normalize(&self) -> Vector2 {
        if self.length2() == 0. {
            Vector2::from_xy(0., 0.)
        }else{
            let d = 1.0 / self.length();
            Vector2::from_xy(self.0.x * d, self.0.y * d)
        }
    }

    pub fn to_angle(&self) -> Angle {
        Angle(f32::atan2(self.0.y, self.0.x))
    }

    pub fn angle(&self) -> f32 {
        f32::atan2(self.0.y, self.0.x)
    }

    pub fn lerp(&self, other: &Vector2, factor: f32) -> Vector2 {
        self.scale(factor) + other.scale(1.0 - factor)
    }

    pub fn rotate<T: Into<f32>>(&self, radians: T) -> Vector2 {
        let radians = radians.into();
        let c:f32 = f32::cos(radians);
        let s:f32 = f32::sin(radians);
        Vector2::from_xy(self.0.x * c - self.0.y * s, self.0.x * s + self.0.y * c)
    }

    pub fn rotate_around_point<T: Into<f32>>(&self, point: &Vector2, radians: T) -> Vector2 {
        let radians = radians.into();
        let c:f32 = f32::cos(radians);
        let s:f32 = f32::sin(radians);
        let x = self.0.x - point.0.x;
        let y = self.0.y - point.0.y;
        Vector2::from_xy(x * c - y * s + point.0.x, x * s + y * c + point.0.y)
    }

    pub fn project_line_segment(&self, a: &Vector2, b: &Vector2) -> Vector2 {
        let a = a.clone();
        let b = b.clone();
        let ab = b - a;         // ab represents the vector from point a to point b
        let ap = *self - a;     // ap represents the vector from point a to the point self
        let t = ap.dot(&ab) / ab.dot(&ab);
        if t < 0. {
            a
        }else
        if 1. < t {
            b
        }else{
            a + ab.scale(t)
        }
    }

    pub fn project_line_infinite(&self, ab: &Vector2) -> Vector2 {
        ab.scale(
            self.dot(ab) / ab.length2()
        )
    }

    pub fn is_not_nan(&self) -> bool {
        self.0.x.is_normal() && self.0.y.is_normal()
    }

    pub fn stretch_to_length(&self, length: f32) -> Vector2 {
        if self.length2() == 0. {
            Vector2::from_xy(length, 0.)
        }else {
            let d = length / self.length();
            Vector2::from_xy(self.0.x * d, self.0.y * d)
        }
    }

    pub fn cross(&self, other: &Vector2) -> f32 {
        self.0.x * other.0.y - self.0.y * other.0.x
    }

}

/* Operator methods */
impl PartialOrd for Vector2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.length2().partial_cmp(&other.length2())
    }
}

/* Approximation methods */
impl AbsDiffEq for Vector2 {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        core::f32::EPSILON * 3.0
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.x.abs_diff_eq(&other.0.x, epsilon) && self.0.y.abs_diff_eq(&other.0.y, epsilon)
    }
}

impl RelativeEq for Vector2 {
    fn default_max_relative() -> Self::Epsilon {
        f32::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
        self.0.x.relative_eq(&other.0.x, epsilon, max_relative) && self.0.y.relative_eq(&other.0.y, epsilon, max_relative)
    }
}

/* Indexing methods */
impl Vector2 {
    fn x(&self) -> f32 {
        self.0.x
    }

    fn y(&self) -> f32 {
        self.0.y
    }
}

/* Conversion methods */
impl Into<f32> for Angle {
    fn into(self) -> f32 {
        self.0
    }
}

/* Test methods */
#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    #[test]
    fn test_dot() {
        let a = super::Vector2::from_xy(1., 0.);
        let b = super::Vector2::from_xy(0., 1.);
        assert_eq!(a.dot(&b), 0.);

        let a = super::Vector2::from_xy(1., 2.);
        let b = super::Vector2::from_xy(3., 4.);
        assert_eq!(a.dot(&b), 11.);
    }

    #[test]
    fn test_length2() {
        let a = super::Vector2::from_xy(-3., 4.);
        assert_eq!(a.length2(), 25.);

        let b = super::Vector2::from_xy(0., 0.);
        assert_eq!(b.length2(), 0.);
    }

    #[test]
    fn test_length() {
        let a = super::Vector2::from_xy(-3., 4.);
        assert_eq!(a.length(), 5.);

        let b = super::Vector2::from_xy(0., 0.);
        assert_eq!(b.length(), 0.);
    }

    #[test]
    fn test_dist2() {
        let a = super::Vector2::from_xy(-1., -2.);
        let b = super::Vector2::from_xy(2., 2.);
        assert_eq!(a.dist2(&b), 25.);

        let a = super::Vector2::from_xy(1., 2.);
        let b = super::Vector2::from_xy(1., 2.);
        assert_eq!(a.dist2(&b), 0.);
    }

    #[test]
    fn test_dist() {
        let a = super::Vector2::from_xy(-1., -2.);
        let b = super::Vector2::from_xy(2., 2.);
        assert_eq!(a.dist(&b), 5.);

        let a = super::Vector2::from_xy(1., 2.);
        let b = super::Vector2::from_xy(1., 2.);
        assert_eq!(a.dist(&b), 0.);
    }

    #[test]
    fn test_scale() {
        let a = super::Vector2::from_xy(-1., 2.);
        assert_eq!(a.scale(-2.), super::Vector2::from_xy(2., -4.));

        let b = super::Vector2::from_xy(0., 0.);
        assert_eq!(b.scale(2.), super::Vector2::from_xy(0., 0.));
    }

    #[test]
    fn test_normalize() {
        let a = super::Vector2::from_xy(3., 4.);
        assert_eq!(a.normalize(), super::Vector2::from_xy(3. / 5., 4. / 5.));
        assert_eq!(a.normalize().length(), 1.);

        let b = super::Vector2::from_xy(0., 0.);
        assert_eq!(b.normalize(), super::Vector2::from_xy(0., 0.));
        assert_eq!(b.normalize().length(), 0.);

    }

    #[test]
    fn test_to_angle() {
        let a = super::Vector2::from_xy(1., 1.);
        assert_eq!(a.to_angle().0, std::f32::consts::FRAC_PI_4);

        let b = super::Vector2::from_xy(0., 0.);
        assert_eq!(b.to_angle().0, 0.);
    }

    #[test]
    fn test_angle() {
        let a = super::Vector2::from_xy(1., 1.);
        assert_eq!(a.angle(), std::f32::consts::FRAC_PI_4);

        let b = super::Vector2::from_xy(0., 0.);
        assert_eq!(b.angle(), 0.);
    }

    #[test]
    fn test_lerp() {
        let a = super::Vector2::from_xy(1., 1.);
        let b = super::Vector2::from_xy(3., 3.);
        assert_eq!(a.lerp(&b, 0.), super::Vector2::from_xy(3., 3.));
        assert_eq!(a.lerp(&b, 0.5), super::Vector2::from_xy(2., 2.));
        assert_eq!(a.lerp(&b, 1.), super::Vector2::from_xy(1., 1.));
        assert_eq!(a.lerp(&b, 2.), super::Vector2::from_xy(-1., -1.));
    }

    #[test]
    fn test_rotate() {
        let a = super::Vector2::from_xy(1., 0.);
        assert_relative_eq!(a.rotate(std::f32::consts::FRAC_PI_2), super::Vector2::from_xy(0., 1.));
        assert_relative_eq!(a.rotate(std::f32::consts::PI), super::Vector2::from_xy(-1., 0.));
        assert_relative_eq!(a.rotate(std::f32::consts::PI * 1.5), super::Vector2::from_xy(0., -1.));
        assert_relative_eq!(a.rotate(std::f32::consts::PI * 2.), super::Vector2::from_xy(1., 0.));
    }

    #[test]
    fn test_rotate_around_point() {
        let a = super::Vector2::from_xy(1., 0.);
        let b = super::Vector2::from_xy(2., 2.);
        assert_relative_eq!(a.rotate_around_point(&b, std::f32::consts::PI * 0.5), super::Vector2::from_xy(4., 1.));
        assert_relative_eq!(a.rotate_around_point(&b, std::f32::consts::PI), super::Vector2::from_xy(3., 4.));
        assert_relative_eq!(a.rotate_around_point(&b, std::f32::consts::PI * 1.5), super::Vector2::from_xy(0., 3.));
        assert_relative_eq!(a.rotate_around_point(&b, std::f32::consts::PI * 2.), super::Vector2::from_xy(1., 0.));
    }

    #[test]
    fn test_project_line_segment() {
        let x = super::Vector2::from_xy(1., 3.);
        let y = super::Vector2::from_xy(10., 12.);
        let z = super::Vector2::from_xy(-9., -6.);

        let a = super::Vector2::from_xy(1., 1.);
        let b = super::Vector2::from_xy(3., 3.);

        assert_eq!(x.project_line_segment(&a, &b), super::Vector2::from_xy(2., 2.));
        assert_eq!(y.project_line_segment(&a, &b), b);
        assert_eq!(z.project_line_segment(&a, &b), a);
    }

    #[test]
    fn test_project_line_infinite() {
        let x = super::Vector2::from_xy(1., 3.);
        let y = super::Vector2::from_xy(10., 12.);
        let z = super::Vector2::from_xy(-9., -7.);

        let line = super::Vector2::from_xy(1., 1.);

        assert_eq!(x.project_line_infinite(&line), super::Vector2::from_xy(2., 2.));
        assert_eq!(y.project_line_infinite(&line), super::Vector2::from_xy(11., 11.));
        assert_eq!(z.project_line_infinite(&line), super::Vector2::from_xy(-8., -8.));
    }

    #[test]
    fn test_is_not_nan() {
        let a = super::Vector2::from_xy(1., 3.);
        let b = super::Vector2::from_xy(10., f32::NAN);
        let c = super::Vector2::from_xy(f32::NAN, f32::NAN);

        assert!(a.is_not_nan());
        assert!(!b.is_not_nan());
        assert!(!c.is_not_nan());
    }

    #[test]
    fn test_stretch_to_length() {
        let a = super::Vector2::from_xy(10., 0.);
        let b = super::Vector2::from_xy(0., 0.);
        let c = super::Vector2::from_xy(1., 1.);

        assert_eq!(a.stretch_to_length(5.), super::Vector2::from_xy(5., 0.));
        assert_eq!(b.stretch_to_length(5.), super::Vector2::from_xy(5., 0.));
        assert_eq!(c.stretch_to_length(1.), super::Vector2::from_xy(f32::sqrt(2.)/2., f32::sqrt(2.)/2.));
    }
    
    #[test]
    fn test_cross() {
        let a = super::Vector2::from_xy(1., 0.);
        let b = super::Vector2::from_xy(0., 1.);
        assert_eq!(a.cross(&b), 1.);

        let a = super::Vector2::from_xy(1., 2.);
        let b = super::Vector2::from_xy(3., 4.);
        assert_eq!(a.cross(&b), -2.);
    }

    #[test]
    fn test_addition() {
        let a = super::Vector2::from_xy(1., 2.);
        let b = super::Vector2::from_xy(3., 4.);
        assert_eq!(a + b, super::Vector2::from_xy(4., 6.));
    }

    #[test]
    fn test_subtraction() {
        let a = super::Vector2::from_xy(1., 2.);
        let b = super::Vector2::from_xy(3., 4.);
        assert_eq!(a - b, super::Vector2::from_xy(-2., -2.));
    }

    #[test]
    fn test_indexing() {
        let a = super::Vector2::from_xy(1., 2.);
        assert_eq!(a[0], 1.);
        assert_eq!(a[1], 2.);

        assert_eq!(a.x(), 1.);
        assert_eq!(a.y(), 2.);
    }
}