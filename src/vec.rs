use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! vec3 {
    () => {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    };
    ($x: expr, $y: expr, $z: expr) => {
        Vec3 {
            x: $x,
            y: $y,
            z: $z,
        }
    };
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, v: Vec3) -> Self::Output {
        Self {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Self::Output {
        Vec3 {
            x: vec.x * self,
            y: vec.y * self,
            z: vec.z * self,
        }
    }
}

impl Vec3 {
    pub fn norm_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn to_unit_vec(&self) -> Vec3 {
        let norm: f64 = self.norm();
        assert_ne!(
            norm, 0.0,
            "Zero vector cannot be converted to a unique unit vector"
        );
        return vec3!(self.x, self.y, self.z) * (1.0 / norm);
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: &self.y * &other.z - &self.z * &other.y,
            y: &self.z * &other.x - &self.x * &other.z,
            z: &self.x * &other.y - &self.y * &other.x,
        }
    }

    pub fn rotate(&self, unit_vec: &Vec3, theta_rad: f64) -> Vec3 {
        assert_eq!(unit_vec.norm(), 1.0);
        let v_rot = theta_rad.cos() * *self
            + (1.0 - theta_rad.cos()) * (unit_vec.dot(self) * *unit_vec)
            + theta_rad.sin() * (unit_vec.cross(self));
        return v_rot;
    }

    pub fn almost_zero(&self) -> bool {
        return self.x.abs() < f64::EPSILON
            && self.y.abs() < f64::EPSILON
            && self.z.abs() < f64::EPSILON;
    }
}

pub const ZERO: Vec3 = vec3!();

#[test]
fn test_vec3_macro_without_args() {
    let actual: Vec3 = vec3!();
    let expected: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    assert_eq!(actual, expected);
}

#[test]
fn test_vec3_macro_with_args() {
    let actual: Vec3 = vec3!(42.1, 11.3, 0.11);
    let expected: Vec3 = Vec3 {
        x: 42.1,
        y: 11.3,
        z: 0.11,
    };
    assert_eq!(actual, expected);
}

#[test]
fn test_vec3_add() {
    let v1: Vec3 = vec3!(1.0, 0.5, 2.0);
    let v2: Vec3 = vec3!(0.0, 1.5, 1.0);
    let actual = v1 + v2;
    let expected: Vec3 = vec3!(1.0, 2.0, 3.0);
    assert_eq!(actual, expected);
}

#[test]
fn test_vec3_sub() {
    let v1: Vec3 = vec3!(1.0, -0.5, 2.0);
    let v2: Vec3 = vec3!(2.0, 1.5, 1.0);
    let actual = v1 - v2;
    let expected: Vec3 = vec3!(-1.0, -2.0, 1.0);
    assert_eq!(actual, expected);
}

#[test]
fn test_vec3_neg() {
    let v: Vec3 = vec3!(1.0, -0.5, 2.0);
    let actual = -v;
    let expected: Vec3 = vec3!(-1.0, 0.5, -2.0);
    assert_eq!(actual, expected);
}

#[test]
fn test_vec3_scalar_mul() {
    let v: Vec3 = vec3!(1.0, -0.5, 2.0);
    let t: f64 = 8.0;
    let actual = v * t;
    let expected: Vec3 = vec3!(8.0, -4.0, 16.0);
    assert_eq!(actual, expected);
}

#[test]
fn test_vec3_vec3_mul() {
    let v: Vec3 = vec3!(1.0, -0.5, 2.0);
    let w: Vec3 = vec3!(2.0, -0.5, -1.0);
    let actual = v * w;
    let expected: Vec3 = vec3!(2.0, 0.25, -2.0);
    assert_eq!(actual, expected);
}

#[test]
fn test_norm_squared() {
    let v = vec3!(5.0f64.sqrt(), 2.0, 4.0);
    let actual = v.norm_squared();
    let expected = 25f64;
    assert_eq!(actual, expected);
}

#[test]
fn test_norm() {
    let v = vec3!(5.0f64.sqrt(), 2.0, 4.0);
    let actual = v.norm();
    let expected = 5f64;
    assert_eq!(actual, expected);
}

#[test]
fn test_dot() {
    let v1 = vec3!(1.0, 2.0, 3.0);
    let v2 = vec3!(-2.0, 0.0, 3.0);
    let actual = v1.dot(&v2);
    let expected = -2.0 + 0.0 + 9.0;
    assert_eq!(actual, expected);
}

#[test]
fn test_cross() {
    let v1 = vec3!(2.0, 3.0, 4.0);
    let v2 = vec3!(5.0, 6.0, 7.0);
    let actual = v1.cross(&v2);
    let expected = vec3!(-3.0, 6.0, -3.0);
    assert_eq!(actual, expected);
}

#[test]
fn test_rotate_60_degrees() {
    let v = vec3!(0.5, 0.5, -1.0);
    let rotation_axis = vec3!(0.0, 0.0, -1.0);
    let actual = v.rotate(&rotation_axis, 60f64.to_radians());
    let expected = vec3!(0.68301, -0.18301, -1.0);

    let distance = (actual - expected).norm();
    assert!(
        distance < 0.001,
        "||actual - expected ||= ||{:?} - {:?} || > EPSILON",
        actual,
        expected
    );
}

#[test]
fn test_rotate_30_degrees() {
    let v = vec3!(0.5, 0.5, -1.0);
    let rotation_axis = vec3!(0.0, 0.0, 1.0);
    let actual = v.rotate(&rotation_axis, 30f64.to_radians());
    let expected = vec3!(0.18301, 0.68301, -1.0);

    let distance = (actual - expected).norm();
    assert!(
        distance < 0.001,
        "||actual - expected ||= ||{:?} - {:?} || > EPSILON",
        actual,
        expected
    );
}

#[test]
fn test_unit_vec() {
    let v = vec3!(1.0, 1.0, 0.0);
    let actual = v.to_unit_vec();
    let expected = vec3!(1f64 / 2f64.sqrt(), 1f64 / 2f64.sqrt(), 0.0);
    assert_eq!(actual, expected);
}

pub type Color = Vec3;
