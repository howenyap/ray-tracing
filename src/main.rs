use ray_tracing::{
    Camera, Colour, HittableList, Material, Object, Point, Shape, Vector, random_double,
    random_double_with_range,
};

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 1200;
    let samples_per_pixel = 5;
    let max_depth = 50;
    let vfov = 20.;
    let look_from = Point::new(13, 2, 3);
    let look_at = Point::new(0, 0, 0);
    let vup = Vector::new(0, 1, 0);
    let defocus_angle = 0.6;
    let focus_distance = 10.;

    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        look_from,
        look_at,
        vup,
        defocus_angle,
        focus_distance,
    );

    let mut world = Vec::new();

    let ground_material = Material::lambertian(Colour::new(0.5, 0.5, 0.5));
    world.push(Object::new(
        Shape::sphere(Point::new(0, -1000, 0), 1000),
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point::new(
                f64::from(a) + 0.9 * random_double(),
                0.2,
                f64::from(b) + 0.9 * random_double(),
            );

            if (center - Point::new(4, 0.2, 0)).len() > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    // diffuse
                    Material::lambertian(Colour::random() * Colour::random())
                } else if choose_mat < 0.95 {
                    // metal
                    Material::metal(
                        Colour::random_with_range(0.5, 1.),
                        random_double_with_range(0., 0.5),
                    )
                } else {
                    // glass
                    Material::dielectric(1.5)
                };

                world.push(Object::new(Shape::sphere(center, 0.2), sphere_material));
            }
        }
    }

    let material1 = Material::dielectric(1.5);
    world.push(Object::new(
        Shape::sphere(Point::new(0, 1, 0), 1),
        material1,
    ));

    let material2 = Material::lambertian(Colour::new(0.4, 0.2, 0.1));
    world.push(Object::new(
        Shape::sphere(Point::new(-4, 1, 0), 1),
        material2,
    ));

    let material3 = Material::metal(Colour::new(0.7, 0.6, 0.5), 0.);
    world.push(Object::new(
        Shape::sphere(Point::new(4, 1, 0), 1),
        material3,
    ));

    let world = HittableList::new(world);

    camera.render(&world);
}
