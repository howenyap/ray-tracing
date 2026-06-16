use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

use crate::point::Point;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vector(f64, f64, f64);

impl Vector {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        let x = x.into();
        let y = y.into();
        let z = z.into();

        Vector(x, y, z)
    }

    pub const fn zero() -> Self {
        Vector(0., 0., 0.)
    }

    pub const fn x(&self) -> f64 {
        self.0
    }

    pub const fn y(&self) -> f64 {
        self.1
    }

    pub const fn z(&self) -> f64 {
        self.2
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: &Self) -> Self {
        let x = self.y() * other.z() - self.z() * other.y();
        let y = self.z() * other.x() - self.x() * other.z();
        let z = self.x() * other.y() - self.y() * other.x();

        Vector(x, y, z)
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.len()
    }

    pub fn random() -> Self {
        Vector(rand::random(), rand::random(), rand::random())
    }

    pub fn random_with_range(min: impl Into<f64>, max: impl Into<f64>) -> Self {
        let min = min.into();
        let max = max.into();

        let random_in_range = |min: f64, max: f64| min + (max - min) * rand::random::<f64>();

        Vector(
            random_in_range(min, max),
            random_in_range(min, max),
            random_in_range(min, max),
        )
    }

    pub fn random_on_hemisphere(normal: &Vector) -> Vector {
        let on_unit_sphere = Self::random_unit_vector();

        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn random_unit_vector() -> Vector {
        loop {
            let vector = Vector::random_with_range(-1, 1);
            let len_squared = vector.len_squared();

            if (1e-160..=1.).contains(&len_squared) {
                // todo: lazy computation of length and length squared
                return vector / len_squared.sqrt();
            }
        }
    }

    pub fn random_in_unit_disk() -> Vector {
        loop {
            let vector = Vector::new(
                rand::random::<f64>() * 2. - 1.,
                rand::random::<f64>() * 2. - 1.,
                0.,
            );

            if vector.len_squared() < 1. {
                return vector;
            }
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;

        [self.x(), self.y(), self.z()].iter().all(|&x| x.abs() < s)
    }

    pub fn reflect(self, normal: Vector) -> Vector {
        self - 2. * self.dot(&normal) * normal
    }

    pub fn refract(self, normal: Vector, etai_over_etat: f64) -> Vector {
        let cos_theta = (-self.dot(&normal)).min(1.0);

        let r_out_perpendicular = etai_over_etat * (self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perpendicular.len_squared()).abs().sqrt() * normal;

        r_out_perpendicular + r_out_parallel
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

// Vector ops

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        let x = -self.x();
        let y = -self.y();
        let z = -self.z();

        Vector(x, y, z)
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x() + rhs.x();
        let y = self.y() + rhs.y();
        let z = self.z() + rhs.z();

        Vector(x, y, z)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul for Vector {
    type Output = Vector;

    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.x() * rhs.x();
        let y = self.y() * rhs.y();
        let z = self.z() * rhs.z();

        Vector(x, y, z)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.x();
        self.1 += rhs.y();
        self.2 += rhs.z();
    }
}

// f64 ops

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        let x = self.x() * rhs;
        let y = self.y() * rhs;
        let z = self.z() * rhs;

        Vector(x, y, z)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1. / rhs)
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

// i32 ops

impl Mul<i32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: i32) -> Self::Output {
        self * rhs as f64
    }
}

impl Mul<Vector> for i32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self as f64
    }
}

impl Div<i32> for Vector {
    type Output = Vector;

    fn div(self, rhs: i32) -> Self::Output {
        self / rhs as f64
    }
}

// u32 ops

impl Mul<u32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: u32) -> Self::Output {
        self * rhs as f64
    }
}

impl Mul<Vector> for u32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self as f64
    }
}

impl Div<u32> for Vector {
    type Output = Vector;

    fn div(self, rhs: u32) -> Self::Output {
        self / rhs as f64
    }
}

// Point ops

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        let x = self.x() + rhs.x();
        let y = self.y() + rhs.y();
        let z = self.z() + rhs.z();

        Point::new(x, y, z)
    }
}
