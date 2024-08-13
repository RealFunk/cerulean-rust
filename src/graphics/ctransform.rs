use crate::math::cmatrix::Matrix3x3;
use crate::math::cmatrix::Matrix4x4;

pub struct CTransform {
    pub scale: (f64 ,f64, f64),
    pub rot: (f64, f64, f64),
    // pub rot: Matrix3x3,
    pub pos: (f64, f64, f64),
}

impl CTransform {
    pub fn asMatrix(&self) -> Matrix4x4 {

        let (kx, ky, kz) = self.scale;
        let (sx, sy, sz) = (self.rot.0.sin(), self.rot.1.sin(), self.rot.2.sin());
        let (cx, cy, cz) = (self.rot.0.cos(), self.rot.1.cos(), self.rot.2.cos());
        let (tx, ty, tz) = self.pos;

        return Matrix4x4 {
            e11: kx*cy*cz,
            e12: ky*(sx*sy*cz - sz*cx),
            e13: kz*(sy*cx*cz + sx*sz),
            e14: tx,
            e21: kx*sz*cy,
            e22: ky*(sx*sy*sz + cx*cz),
            e23: kz*(sy*sz*cx - sx*cz),
            e24: ty,
            e31: -1.0*kx*sy,
            e32: ky*sx*cy,
            e33: kz*cx*cy,
            e34: tz,
            e41: 0.0,
            e42: 0.0,
            e43: 0.0,
            e44: 1.0,
        };
    }
}

pub fn new(x: f64, y: f64, z: f64) -> CTransform {
    return CTransform {
        scale: (1.0, 1.0, 1.0),
        rot: (0.0, 0.0, 0.0),
        // rot: Matrix3x3::new(),
        pos: (x, y, z),
    };
}