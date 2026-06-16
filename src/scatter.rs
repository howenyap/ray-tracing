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
            Self::Dielectric { refraction_index } => {
                let attenuation = Colour::white();
                let refraction_index = if hit_record.front_face() {
                    1. / refraction_index
                } else {
                    *refraction_index
                };

                let unit_direction = ray.direction().unit_vector();
                let cos_theta = (-unit_direction.dot(&hit_record.normal())).min(1.0);
                let sin_theta = (1. - cos_theta * cos_theta).sqrt();
                let cannot_refract = refraction_index * sin_theta > 1.;

                let direction = if cannot_refract
                    || reflectance(cos_theta, refraction_index) > rand::random::<f64>()
                {
                    unit_direction.reflect(hit_record.normal())
                } else {
                    unit_direction.refract(hit_record.normal(), refraction_index)
                };

                let ray = Ray::new(hit_record.point(), direction);

                Some((attenuation, ray))
            }
        }
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1. - refraction_index) / (1. + refraction_index);
    let r0 = r0 * r0;

    r0 + (1. - r0) * (1. - cosine).powf(5.)
}
