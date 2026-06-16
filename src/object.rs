use crate::{Material, Shape};

pub struct Object {
    shape: Shape,
    material: Material,
}

impl Object {
    pub fn new(shape: Shape, material: Material) -> Self {
        Self { shape, material }
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn material(&self) -> &Material {
        &self.material
    }
}
