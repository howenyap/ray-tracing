use crate::Colour;

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian { albedo: Colour },
    Metal { albedo: Colour },
}

impl Material {
    pub fn lambertian(albedo: Colour) -> Self {
        Self::Lambertian { albedo }
    }

    pub fn metal(albedo: Colour) -> Self {
        Self::Metal { albedo }
    }
}
