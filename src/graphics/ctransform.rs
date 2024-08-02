pub struct CTransform {
    pub scale: f64,
    // rot:
    pub pos: (f64, f64, f64),
}

pub fn new(x: f64, y: f64, z: f64) -> CTransform {
    return CTransform {
        scale: 1.0,
        //rot: ???,
        pos: (x, y, z),
    };
}