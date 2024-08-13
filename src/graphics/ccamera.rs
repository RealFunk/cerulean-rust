use crate::ctransform::CTransform;

use super::ctransform;

pub struct CCamera {
    pub fov: f64, //x-axis
    pub transform: CTransform,
}

pub fn new() -> CCamera {
    return CCamera {
        // fov: 1.047198, //60 degrees
        fov: 1.47079632679,
        transform: ctransform::new(0.0, 0.0, 0.0),
    };
}