use crate::{Colour, Hittable, Interval, Point, Vector};

pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn direction(&self) -> &Vector {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }

    pub fn colour(&self, hittable: &impl Hittable) -> Colour {
        if let Some(hit_record) = hittable.hit(self, &Interval::range(0., f64::INFINITY)) {
            let direction = Vector::random_on_hemisphere(hit_record.normal());

            0.5 * Ray::new(*hit_record.point(), direction).colour(hittable)
        } else {
            let unit_direction = self.direction().unit_vector();
            let a = 0.5 * (unit_direction.y() + 1.);

            (1. - a) * Colour::new(1., 1., 1.) + a * Colour::new(0.5, 0.7, 1.)
        }
    }
}
