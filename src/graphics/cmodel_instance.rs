
use crate::graphics::cmodel;
use crate::graphics::ctransform;

pub struct CModelInstance {
    pub model: cmodel::CModel,
    pub transform: ctransform::CTransform,
}

pub fn new_cube(x: f64, y: f64, z: f64) -> CModelInstance {
    return CModelInstance {
        model: cmodel::new_cube(),
        transform: ctransform::new(x, y, z),
    };
}