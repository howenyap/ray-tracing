use ray_tracing::{Colour, Hittable, Point, Ray, Shape, Vector};

fn main() {
    let aspect_ratio = 16. / 9.;

    let image_width = 400;
    let image_height = ((image_width as f64 / aspect_ratio) as i32).max(1);

    let focal_length = 1.;
    let viewport_height = 2.;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point::zero();

    let viewport_u = Vector::new(viewport_width, 0., 0.);
    let viewport_v = Vector::new(0., -viewport_height, 0.);

    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    let viewport_upper_left =
        camera_center - Vector::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    let sphere = Shape::sphere(Point::new(0., 0., -1.), 0.5);

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        let remaining = image_height - j;
        eprintln!("Scanlines remaining: {remaining}");

        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let pixel_colour = ray_colour(&ray, &sphere);

            println!("{pixel_colour}");
        }
    }
}

fn ray_colour(ray: &Ray, sphere: &impl Hittable) -> Colour {
    if let Some(hit_record) = sphere.hit(ray, 0., f64::INFINITY) {
        let n = hit_record.normal();

        0.5 * Colour::new(n.x() + 1., n.y() + 1., n.z() + 1.)
    } else {
        let unit_direction = ray.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.);

        (1. - a) * Colour::new(1., 1., 1.) + a * Colour::new(0.5, 0.7, 1.)
    }
}
