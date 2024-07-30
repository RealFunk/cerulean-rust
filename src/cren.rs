
use crate::craster::CRaster;
use crate::cmath;

pub struct CRen {
    pub raster: CRaster,    //CRen owns its raster
}

impl CRen {

    pub fn new(w: usize, h: usize) -> CRen {
        let mut raster: CRaster = CRaster::new(w, h);
        return CRen {raster};
    }

    pub fn clear(&mut self) {
        for i in self.raster.data.iter_mut() {
            *i = 0x000000;
        }
    }

    pub fn fill(&mut self, color: u32) {
        for i in self.raster.data.iter_mut() {
            *i = color;
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.raster.width && y < self.raster.height {
            self.raster.data[x + self.raster.width*y] = color;
        }
    }

    pub fn draw_line(&mut self, mut x0: usize, mut y0: usize, mut x1: usize, mut y1: usize, color: u32) {
        //Does not use Bresenham's line algorithm, but maybe do this later?
        
        // if x1 < x0 {
        //    let tx = x0;
        //    let ty = y0;
        //    x0 = x1;
        //    y0 = y1;
        //    x1 = tx;
        //    y1 = tx;
        // }

        let fx0: f64 = x0 as f64;
        let fy0: f64 = y0 as f64;
        let fx1: f64 = x1 as f64;
        let fy1: f64 = y1 as f64;

        let a: f64 = (fy1 - fy0) / (fx1 - fx0);

        if y0 == y1 {
            if x0 < x1 {
                self.draw_gentle_line(x0, y0, x1, y1, color);
            }
            else {
                self.draw_gentle_line(x1, y1, x0, y0, color);
            }
        }
        else if x0 == x1 {
            if y0 < y1 {
                self.draw_steep_line(x0, y0, x1, y1, color);
            }
            else {
                self.draw_steep_line(x1, y1, x0, y0, color);
            }
        }
        else if a > 1.0 || a < -1.0 {
            //steep
            if x0 < x1 {
                self.draw_steep_line(x0, y0, x1, y1, color);
            }
            else {
                self.draw_steep_line(x1, y1, x0, y0, color);
            }
        }     
        else {
            //gentle
            if x0 < x1 {
                self.draw_gentle_line(x0, y0, x1, y1, color);
            }
            else {
                self.draw_gentle_line(x1, y1, x0, y0, color);
            }
        }
    }

    pub fn draw_gentle_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
        let y_values: Vec<usize> = cmath::usize_lerp(x0, y0, x1, y1);
        for x in x0..x1 {
            self.set_pixel(x, y_values[x - x0], color);
        }
    }

    pub fn draw_steep_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
        let x_values: Vec<usize> = cmath::usize_lerp(y0, x0, y1, x1);
        for y in y0..y1 {
            self.set_pixel(x_values[y - y0], y, color);
        }
    }

    // pub fn draw_antialiased_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
    //     //Uses Xiaolin Wu's line algorithm

    // }

    pub fn draw_raster(&mut self, x: usize, y: usize, raster: &CRaster) {
        for i in 0..raster.width {
            for j in 0..raster.height {
                self.set_pixel(x + i, y + j, raster.data[i + raster.width*j]);
            }
        }
    }

}