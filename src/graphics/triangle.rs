pub struct Triangle {
    pub i0: usize,
    pub i1: usize,
    pub i2: usize,
    pub color: u32,
}

impl Triangle {

    pub fn new(i0: usize, i1: usize, i2: usize, color: u32) -> Triangle {
        return Triangle {
            i0,
            i1,
            i2,
            color,
        };
    }

}

impl Clone for Triangle {

    fn clone(&self) -> Triangle {
        Triangle {
            i0: self.i0,
            i1: self.i1,
            i2: self.i2,
            color: self.color,
        }
    }

}