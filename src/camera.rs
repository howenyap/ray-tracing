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
    u: Vector,                  // Camera frame basis vectors
    v: Vector,
    w: Vector,
    defocus_disk_u: Vector, // Defocus disk horizontal radius
    defocus_disk_v: Vector, // Defocus disk vertical radius
    max_depth: u32,         // Maximum number of ray bounces into scene
    defocus_angle: f64,     // Variation angle of rays through each pixel
    focus_distance: f64,    // Distance from camera lookfrom point to plane of perfect focus
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f64,
        look_from: Point,
        look_at: Point,
        vup: Vector,
        defocus_angle: f64,  // Variation angle of rays through each pixel
        focus_distance: f64, // Distance from camera lookfrom point to plane of perfect focus
    ) -> Self {
        let image_height: u32 = ((image_width as f64 / aspect_ratio) as u32).max(1);

        let pixel_samples_scale = 1. / samples_per_pixel as f64;

        let center = look_from;

        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * focus_distance;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        let viewport_upper_left = center - (focus_distance * w) - viewport_u / 2 - viewport_v / 2;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_distance * (defocus_angle / 2.).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

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
            u,
            v,
            w,
            max_depth,
            defocus_disk_u,
            defocus_disk_v,
            defocus_angle,
            focus_distance,
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
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.

        let offset = Vector::new(rand::random::<f64>() - 0.5, rand::random::<f64>() - 0.5, 0.);
        let pixel_sample = self.pixel00_loc
            + ((f64::from(i) + offset.x()) * self.pixel_delta_u)
            + ((f64::from(j) + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point {
        let p = Vector::random_in_unit_disk();

        self.center + p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v
    }
}
