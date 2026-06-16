use crate::{Colour, HitRecord, Material, Ray, Vector};

pub trait Scatter {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Colour, Ray)>;
}

impl Scatter for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Colour, Ray)> {
        match self {
            Self::Lambertian { albedo } => {
                let scatter_direction = *hit_record.normal() + Vector::random_unit_vector();

                let scatter_direction = if scatter_direction.near_zero() {
                    *hit_record.normal()
                } else {
                    scatter_direction
                };

                let scattered = Ray::new(*hit_record.point(), scatter_direction);

                Some((*albedo, scattered))
            }
        }
    }
}
