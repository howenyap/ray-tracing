use crate::Point;

pub enum Shape {
    Sphere { center: Point, radius: f64 },
}

impl Shape {
    pub fn sphere(center: Point, radius: impl Into<f64>) -> Self {
        let radius = radius.into();

        assert!(radius > 0.0, "radius must be positive");

        Self::Sphere { center, radius }
    }
}
