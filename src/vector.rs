use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

#[derive(Debug, Clone, Copy)]
struct Vector(f64, f64, f64);

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
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
}

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

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.x();
        self.1 += rhs.y();
        self.2 += rhs.z();
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

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}
