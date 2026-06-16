use crate::Colour;

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian { albedo: Colour },
}

impl Material {
    pub fn lambertian(albedo: Colour) -> Self {
        Self::Lambertian { albedo }
    }
}
