pub struct CRaster {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
}

impl CRaster {

    pub fn new(width: usize, height: usize) -> CRaster {
        return CRaster {
            width: width,
            height: height,
            data: vec![0 as u32; width*height],
        };
    }

    pub fn get(&self, x: usize, y: usize) -> u32 {
        if x < self.width && y < self.height {
            return self.data[x + self.height*y];
        }
        else {
            println!("[CRaster/WARN] Cannot get index ({}, {}) in a {}x{} raster", x, y, self.width, self.height);
            return 0x00000000;
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: u32) {
        if x < self.width && y < self.height {
            self.data[x + self.height*y] = value;
        }
        else {
            println!("[CRaster/WARN] Cannot set index ({}, {}) in a {}x{} raster", x, y, self.width, self.height);
        }
    }

}