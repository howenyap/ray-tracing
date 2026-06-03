use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Colour(f64, f64, f64);

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Colour(r, g, b)
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
