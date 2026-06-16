use crate::{Colour, HitRecord, Material, Ray, Vector};

pub trait Scatter {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Colour, Ray)>;
}

impl Scatter for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Colour, Ray)> {
        match self {
            Self::Lambertian { albedo } => {
                let scatter_direction = hit_record.normal() + Vector::random_unit_vector();

                let scatter_direction = if scatter_direction.near_zero() {
                    hit_record.normal()
                } else {
                    scatter_direction
                };

                let scattered = Ray::new(hit_record.point(), scatter_direction);

                Some((*albedo, scattered))
            }
            Self::Metal { albedo, fuzz } => {
                let reflected = ray.direction().reflect(hit_record.normal());
                let reflected = reflected.unit_vector() + (*fuzz * Vector::random_unit_vector());
                let scattered = Ray::new(hit_record.point(), reflected);

                if reflected.dot(&hit_record.normal()) > 0. {
                    Some((*albedo, scattered))
                } else {
                    None
                }
            }
        }
    }
}
