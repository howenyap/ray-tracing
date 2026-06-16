use crate::Colour;

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian { albedo: Colour },
    Metal { albedo: Colour, fuzz: f64 },
}

impl Material {
    pub fn lambertian(albedo: Colour) -> Self {
        Self::Lambertian { albedo }
    }

    pub fn metal(albedo: Colour, fuzz: impl Into<f64>) -> Self {
        let fuzz = fuzz.into().clamp(0., 1.);

        Self::Metal { albedo, fuzz }
    }
}
