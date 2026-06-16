use crate::{Colour, Hittable, Interval, Point, Scatter, Vector};

pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Vector {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }

    pub fn colour(&self, hittable: &impl Hittable, depth: u32) -> Colour {
        if depth == 0 {
            return Colour::black();
        }

        if let Some(hit_record) = hittable.hit(self, &Interval::range(0.001, f64::INFINITY)) {
            hit_record
                .material()
                .scatter(self, &hit_record)
                .map(|(attenuation, scattered)| attenuation * scattered.colour(hittable, depth - 1))
                .unwrap_or(Colour::black())
        } else {
            let unit_direction = self.direction().unit_vector();
            let a = 0.5 * (unit_direction.y() + 1.);

            (1. - a) * Colour::new(1, 1, 1) + a * Colour::new(0.5, 0.7, 1.)
        }
    }
}
