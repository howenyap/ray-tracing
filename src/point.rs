use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Point(f64, f64, f64);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point(x, y, z)
    }

    pub const fn zero() -> Self {
        Point(0., 0., 0.)
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
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}
