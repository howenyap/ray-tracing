use std::{
    fmt::Display,
    ops::{Add, Mul},
};

use crate::Interval;

#[derive(Debug, Clone, Copy)]
pub struct Colour(f64, f64, f64);

impl Colour {
    pub fn new(r: impl Into<f64>, g: impl Into<f64>, b: impl Into<f64>) -> Self {
        let r = r.into();
        let g = g.into();
        let b = b.into();

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
    // maps colour in range of (0..=1) to (0..=255)
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let intensity = Interval::range(0., 0.999);
        let [rbyte, gbyte, bbyte] = [self.r(), self.g(), self.b()].map(|c| {
            // gamma transformation
            let c = c.max(0.).sqrt();

            (256. * intensity.clamp(&c)) as u8
        });

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
