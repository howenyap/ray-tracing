use crate::{Interval, Point, Ray, Shape, Vector};

#[derive(Debug, Default)]
pub struct HitRecord {
    point: Point,
    normal: Vector,
    t: f64,
    // true: ray is outside surface
    // false: ray is inside surface
    front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point, normal: Vector, t: f64, front_face: bool) -> Self {
        Self {
            point,
            normal,
            t,
            front_face,
        }
    }

    pub fn normal(&self) -> &Vector {
        &self.normal
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;
}

impl Hittable for Shape {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        match self {
            Self::Sphere { center, radius } => {
                let oc = *center - *ray.origin();
                let a = ray.direction().len_squared();
                let h = ray.direction().dot(&oc);
                let c = oc.len_squared() - radius * radius;

                let discriminant = h * h - a * c;

                if discriminant.is_sign_negative() {
                    return None;
                }

                let sqrt_discriminant = discriminant.sqrt();
                let root = [(h - sqrt_discriminant) / a, (h + sqrt_discriminant) / a]
                    .into_iter()
                    .find(|r| interval.contains(r))?;

                let point = ray.at(root);
                let normal = (point - *center) / *radius;
                let t = root;

                let front_face = ray.direction().dot(&normal).is_sign_negative();
                let normal = if front_face { normal } else { -normal };

                Some(HitRecord::new(point, normal, t, front_face))
            }
        }
    }
}

pub struct HittableList(Vec<Shape>);

impl HittableList {
    pub fn new(shapes: impl IntoIterator<Item = Shape>) -> Self {
        Self(shapes.into_iter().collect())
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        self.0
            .iter()
            .filter_map(|obj| obj.hit(ray, interval))
            .min_by(|a, b| a.t.total_cmp(&b.t))
    }
}
