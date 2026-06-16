use ray_tracing::{Camera, Colour, HittableList, Material, Object, Point, Shape};
use std::f32::consts::PI;

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vfov = 90.;

    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
    );

    let material_ground = Material::lambertian(Colour::new(0.8, 0.8, 0));
    let material_center = Material::lambertian(Colour::new(0.1, 0.2, 0.5));
    let material_left = Material::dielectric(1.5);
    let material_bubble = Material::dielectric(1. / 1.5);
    let material_right = Material::metal(Colour::new(0.8, 0.6, 0.2), 1);

    let world = HittableList::new([
        Object::new(
            Shape::sphere(Point::new(0, -100.5, -1), 100),
            material_ground,
        ),
        Object::new(Shape::sphere(Point::new(0, 0, -1.2), 0.5), material_center),
        Object::new(Shape::sphere(Point::new(-1, 0, -1), 0.5), material_left),
        Object::new(Shape::sphere(Point::new(-1, 0, -1), 0.4), material_bubble),
        Object::new(Shape::sphere(Point::new(1, 0, -1), 0.5), material_right),
    ]);

    let R = (PI / 4.).cos();
    let material_left = Material::lambertian(Colour::blue());
    let material_right = Material::lambertian(Colour::red());

    let world = HittableList::new([
        Object::new(Shape::sphere(Point::new(-R, 0, -1), R), material_left),
        Object::new(Shape::sphere(Point::new(R, 0, -1), R), material_right),
    ]);

    camera.render(&world);
}
