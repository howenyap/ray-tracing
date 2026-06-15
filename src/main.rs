use ray_tracing::{Camera, HittableList, Point, Shape};

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 400;

    let world = HittableList::new([
        Shape::sphere(Point::new(0., 0., -1.), 0.5),
        Shape::sphere(Point::new(0., -100.5, -1.), 100.),
    ]);

    let camera = Camera::new(aspect_ratio, image_width);
    camera.render(&world);
}
