use crate::math::cmatrix::Matrix3x3;

pub struct CTransform {
    pub scale: f64,
    pub rot: Matrix3x3,
    pub pos: (f64, f64, f64),
}

pub fn new(x: f64, y: f64, z: f64) -> CTransform {
    return CTransform {
        scale: 1.0,
        rot: Matrix3x3::new(),
        pos: (x, y, z),
    };
}