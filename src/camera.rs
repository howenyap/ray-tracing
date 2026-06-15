use crate::{Colour, Hittable, Interval, Point, Ray, Vector};

pub struct Camera {
    pub aspect_ratio: f64, // Ratio of image width over height
    pub image_width: u32,  // Rendered image width in pixel count
    image_height: u32,     // Rendered image height in pixel count
    center: Point,         // Camera center
    pixel00_loc: Point,    // Location of pixel 0, 0 (top left)
    pixel_delta_u: Vector, // Offset to pixel to the right
    pixel_delta_v: Vector, // Offset to pixel below
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
        let image_height: u32 = ((image_width as f64 / aspect_ratio) as u32).max(1);

        let center = Point::zero();

        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vector::new(viewport_width, 0., 0.);
        let viewport_v = Vector::new(0., -viewport_height, 0.);

        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        let viewport_upper_left =
            center - Vector::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &impl Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            let remaining = self.image_height - j;
            eprintln!("Scanlines remaining: {remaining}");

            for i in 0..self.image_width {
                let pixel_center =
                    self.pixel00_loc + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);
                let pixel_colour = Self::ray_colour(&ray, world);

                println!("{pixel_colour}");
            }
        }
    }

    fn ray_colour(ray: &Ray, hittable: &impl Hittable) -> Colour {
        if let Some(hit_record) = hittable.hit(ray, &Interval::range(0., f64::INFINITY)) {
            let n = hit_record.normal();

            0.5 * Colour::new(n.x() + 1., n.y() + 1., n.z() + 1.)
        } else {
            let unit_direction = ray.direction().unit_vector();
            let a = 0.5 * (unit_direction.y() + 1.);

            (1. - a) * Colour::new(1., 1., 1.) + a * Colour::new(0.5, 0.7, 1.)
        }
    }
}
