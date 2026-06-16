use crate::{Interval, Material, Object, Point, Ray, Shape, Vector};

#[derive(Debug)]
pub struct HitRecord {
    point: Point,
    normal: Vector,
    t: f64,
    // true: ray is outside surface
    // false: ray is inside surface
    front_face: bool,
    material: Material,
}

impl HitRecord {
    pub fn new(point: Point, normal: Vector, t: f64, front_face: bool, material: Material) -> Self {
        Self {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }

    pub fn normal(&self) -> Vector {
        self.normal
    }

    pub fn point(&self) -> Point {
        self.point
    }

    pub fn material(&self) -> &Material {
        &self.material
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        match self.shape() {
            Shape::Sphere { center, radius } => {
                let oc = *center - ray.origin();
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

                let material = self.material().clone();

                Some(HitRecord::new(point, normal, t, front_face, material))
            }
        }
    }
}

pub struct HittableList(Vec<Object>);

impl HittableList {
    pub fn new(shapes: impl IntoIterator<Item = Object>) -> Self {
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
