use crate::Point;

pub enum Shape {
    Sphere { center: Point, radius: f64 },
}

impl Shape {
    pub fn sphere(center: Point, radius: f64) -> Self {
        assert!(radius > 0.0, "radius must be positive");

        Self::Sphere { center, radius }
    }
}
