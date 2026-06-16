use crate::{Colour, Hittable, Point, Ray, Vector};

pub struct Camera {
    pub aspect_ratio: f64,      // Ratio of image width over height
    pub image_width: u32,       // Rendered image width in pixel count
    pub samples_per_pixel: u32, // Count of random samples per pixel
    pixel_samples_scale: f64,   // Colour scale factor for pixel samples
    image_height: u32,          // Rendered image height in pixel count
    center: Point,              // Camera center
    pixel00_loc: Point,         // Location of pixel 0, 0 (top left)
    pixel_delta_u: Vector,      // Offset to pixel to the right
    pixel_delta_v: Vector,      // Offset to pixel below
    max_depth: u32,             // Maximum number of ray bounces into scene
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f64,
    ) -> Self {
        let image_height: u32 = ((image_width as f64 / aspect_ratio) as u32).max(1);

        let pixel_samples_scale = 1. / samples_per_pixel as f64;

        let center = Point::zero();

        let focal_length = 1.;
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * focal_length;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vector::new(viewport_width, 0, 0);
        let viewport_v = Vector::new(0, -viewport_height, 0);

        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        let viewport_upper_left =
            center - Vector::new(0, 0, focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            pixel_samples_scale,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            max_depth,
        }
    }

    pub fn render(&self, world: &impl Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            let remaining = self.image_height - j;
            eprintln!("Scanlines remaining: {remaining}");

            for i in 0..self.image_width {
                let pixel_colour = (0..self.samples_per_pixel).fold(Colour::black(), |acc, _| {
                    acc + self.get_ray(i, j).colour(world, self.max_depth)
                }) * self.pixel_samples_scale;

                println!("{pixel_colour}");
            }
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Vector::new(rand::random::<f64>() - 0.5, rand::random::<f64>() - 0.5, 0.);
        let pixel_sample = self.pixel00_loc
            + ((f64::from(i) + offset.x()) * self.pixel_delta_u)
            + ((f64::from(j) + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }
}
