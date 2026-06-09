use std::{
    fmt::Display,
    ops::{Add, Mul},
};

#[derive(Debug, Clone, Copy)]
pub struct Colour(f64, f64, f64);

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Colour(r, g, b)
    }

    pub const fn black() -> Self {
        Colour(0., 0., 0.)
    }

    pub const fn red() -> Self {
        Colour(1., 0., 0.)
    }

    pub const fn r(&self) -> f64 {
        self.0
    }

    pub const fn g(&self) -> f64 {
        self.1
    }

    pub const fn b(&self) -> f64 {
        self.2
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = self.r().clamp(0., 1.);
        let g = self.g().clamp(0., 1.);
        let b = self.b().clamp(0., 1.);

        let rbyte = (255.999 * r) as u8;
        let gbyte = (255.999 * g) as u8;
        let bbyte = (255.999 * b) as u8;

        write!(f, "{rbyte} {gbyte} {bbyte}")
    }
}

impl Add for Colour {
    type Output = Colour;

    fn add(self, rhs: Self) -> Self::Output {
        let r = self.r() + rhs.r();
        let g = self.g() + rhs.g();
        let b = self.b() + rhs.b();

        Colour(r, g, b)
    }
}

impl Mul<f64> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f64) -> Self::Output {
        let r = self.r() * rhs;
        let g = self.g() * rhs;
        let b = self.b() * rhs;

        Colour(r, g, b)
    }
}

impl Mul<Colour> for f64 {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        rhs * self
    }
}
