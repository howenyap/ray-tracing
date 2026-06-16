use crate::Colour;

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian { albedo: Colour },
    Metal { albedo: Colour, fuzz: f64 },
    Dielectric { refraction_index: f64 },
}

impl Material {
    pub fn lambertian(albedo: Colour) -> Self {
        Self::Lambertian { albedo }
    }

    pub fn metal(albedo: Colour, fuzz: impl Into<f64>) -> Self {
        let fuzz = fuzz.into().clamp(0., 1.);

        Self::Metal { albedo, fuzz }
    }

    pub fn dielectric(refraction_index: impl Into<f64>) -> Self {
        let refraction_index = refraction_index.into();

        Self::Dielectric { refraction_index }
    }
}
