use crate::{HitRecord, Ray};

pub trait Scatter {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Ray>;
}
