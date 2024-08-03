
use crate::graphics::craster::CRaster;
use crate::math::cmath;
use crate::graphics::ccamera::CCamera;
use crate::graphics::cmodel_instance::CModelInstance;

use super::ctransform::CTransform;

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

    pub fn set_pixel(&mut self, x: i32, y: i32, mut color: u32) {
        // self.set_alpha_pixel(x, y, color);

        if x >= 0 && y >= 0 && x < self.raster.width as i32 && y < self.raster.height as i32 {
            self.raster.data[(x + (self.raster.width as i32)*y) as usize] = color;
        }
    }

    pub fn draw_line_f64(&mut self, mut x0: f64, mut y0: f64, mut x1: f64, mut y1: f64, color: u32) {

        if x0 == x1 {
            if y0 < y1 {
                for y in (y0 as i32)..((y1 + 1.0) as i32) {
                    self.set_pixel(x0 as i32, y as i32, color);
                }
            }
            else {
                for y in (y1 as i32)..((y0 + 1.0) as i32) {
                    self.set_pixel(x0 as i32, y as i32, color);
                }
            }
        }
        else if y0 == y1 {
            if x0 < x1 {
                for x in (x0 as i32)..((x1 + 1.0) as i32) {
                    self.set_pixel(x as i32, y0 as i32, color);
                }
            }
            else {
                for x in (x1 as i32)..((x0 + 1.0) as i32) {
                    self.set_pixel(x as i32, y0 as i32, color);
                }
            }
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let D = 2.0*dy - dx;
        let y = y0;

        for x in (x0 as i32)..((x1 + 1.0) as i32) {
            self.set_pixel(x, y as i32, color);
        }
    }

    pub fn draw_line(&mut self, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: u32) {
                
        if x0 == x1 {   //vertical line
            if y0 < y1 {
                for y in y0..(y1 + 1) {
                    self.set_pixel(x0, y, color);
                }
            }
            else {
                for y in y1..(y0 + 1) {
                    self.set_pixel(x0, y, color);
                }
            }

            return;
        }
        else if y0 == y1 {  //horizontal line
            if x0 < x1 {
                for x in x0..(x1+1) {
                    self.set_pixel(x, y0, color);
                }
            }
            else {
                for x in x1..(x0+1) {
                    self.set_pixel(x, y0, color);
                }
            }

            return;
        }

        if (y1 - y0).abs() < (x1 - x0).abs() {
			if x0 > x1 {
				self.draw_gentle_line(x1, y1, x0, y0, color);
			}
			else {
                self.draw_gentle_line(x0, y0, x1, y1, color);
            }
		}
		else {
            if y0 > y1 {
                self.draw_steep_line(x1, y1, x0, y0, color);
            }
            else {
                self.draw_steep_line(x0, y0, x1, y1, color);
            }
        }
    }

    fn draw_gentle_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        let dx = x1 - x0;
        let mut dy = y1 - y0;
        let mut yi = 1;
        if dy < 0 { //would it be faster to just have two if statements, instead of having a mutable variable?
            yi = -1;
            dy = -dy;
        }
        let mut D = 2*dy - dx;
        let mut y = y0;

        for x in x0..(x1+1) {
            self.set_pixel(x, y, color);
            if D > 0 {
                y += yi;
                D += -2*dx;
            }
            D += 2*dy;
        }
    }

    fn draw_steep_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        let mut dx = x1 - x0;
        let dy = y1 - y0;
        let mut xi = 1;
        if dx < 0 {
            xi = -1;
            dx = -dx;
        }
        let mut D = 2*dx - dy;
        let mut x = x0;

        for y in y0..(y1+1) {
            self.set_pixel(x, y, color);
            if D > 0 {
                x += xi;
                D += -2*dy;
            }
            D += 2*dx;
        }
    }

    pub fn draw_raster(&mut self, x: i32, y: i32, raster: &CRaster) {
        for i in 0..raster.width {
            for j in 0..raster.height {
                self.set_pixel(x + (i as i32), y + (j as i32), raster.data[i + raster.width*j]);
            }
        }
    }

    pub fn draw_rect(&mut self, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: u32) {
        //TODO: optimize
        self.draw_line(x0, y0, x0, y1, color);
        self.draw_line(x1, y0, x1, y1, color);
        self.draw_line(x0, y0, x1, y0, color);
        self.draw_line(x0, y1, x1, y1, color);
    }

    pub fn fill_rect(&mut self, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: u32) {
        //TODO: optimize
        for y in y0..y1 {
            self.draw_line(x0, y, x1, y, color);
        }
    }

    pub fn draw_triangle(&mut self, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, color: u32) {
        self.draw_line(x0, y0, x1, y1, color);
        self.draw_line(x0, y0, x2, y2, color);
        self.draw_line(x1, y1, x2, y2, color);
    }

    pub fn fill_triangle(&mut self, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, color: u32) {
        
        //swap points such that y0 < y1 < y2
        if y1 < y0 {
            let tx0 = x0;
            let ty0 = y0;
            x0 = x1;
            y0 = y1;
            x1 = tx0;
            y1 = ty0;
        }

        if y2 < y0 {
            let tx0 = x0;
            let ty0 = y0;
            x0 = x2;
            y0 = y2;
            x2 = tx0;
            y2 = ty0;
        }

        if y2 < y1 {
            let tx1 = x1;
            let ty1 = y1;
            x1 = x2;
            y1 = y2;
            x2 = tx1;
            y2 = ty1;
        }

        let mut x01: Vec<i32> = cmath::i32_lerp_vec(y0, x0, y1, x1);
        let mut x12: Vec<i32> = cmath::i32_lerp_vec(y1, x1, y2, x2);
        let mut x02: Vec<i32> = cmath::i32_lerp_vec(y0, x0, y2, x2);
        x01.pop();
        x01.append(&mut x12);
        let x012 = x01;

        let x_left: Vec<i32>;
        let x_right: Vec<i32>;

        let m = x012.len() / 2;
        if x02[m] < x012[m] {
            x_left = x02;
            x_right = x012;
        }
        else {
            x_left = x012;
            x_right = x02;
        }

        for y in y0..(y2+1) {
            for x in x_left[(y - y0) as usize]..x_right[(y - y0) as usize] {
                self.set_pixel(x, y, color);
            }
        }

    }

    pub fn fill_shaded_triangle(&mut self, mut x0: i32, mut y0: i32, mut h0: f64, mut x1: i32, mut y1: i32, mut h1: f64, mut x2: i32, mut y2: i32, mut h2: f64, color: u32) {
        
        /*
        Current problem: the algorithm assigns whichever point is in the middle
        the value of the lowest point
         */

        //swap points such that y0 < y1 < y2
        if y1 < y0 {
            let tx0 = x0;
            let ty0 = y0;
            let th0 = h0;
            x0 = x1;
            y0 = y1;
            h0 = h1;
            x1 = tx0;
            y1 = ty0;
            h1 = th0;
        }

        if y2 < y0 {
            let tx0 = x0;
            let ty0 = y0;
            let th0 = h0;
            x0 = x2;
            y0 = y2;
            h0 = h2;
            x2 = tx0;
            y2 = ty0;
            h2 = th0;
        }

        if y2 < y1 {
            let tx1 = x1;
            let ty1 = y1;
            let th1 = h1;
            x1 = x2;
            y1 = y2;
            h1 = h2;
            x2 = tx1;
            y2 = ty1;
            h2 = th1;
        }

        let mut x01: Vec<i32> = cmath::i32_lerp_vec(y0, x0, y1, x1);
        let mut h01: Vec<f64> = cmath::f64_lerp_vec(y0 as f64, h0, y1 as f64, h1);

        let mut x12: Vec<i32> = cmath::i32_lerp_vec(y1, x1, y2, x2);
        let mut h12: Vec<f64> = cmath::f64_lerp_vec(y1 as f64, h1, y2 as f64, h2);

        let mut x02: Vec<i32> = cmath::i32_lerp_vec(y0, x0, y2, x2);
        let mut h02: Vec<f64> = cmath::f64_lerp_vec(y0 as f64, h0, y2 as f64, h2);

        x01.pop();
        h01.pop();

        x01.append(&mut x12);
        h01.append(&mut h12);
    
        let x012 = x01;
        let h012 = h01;

        let x_left: Vec<i32>;
        let h_left: Vec<f64>;

        let x_right: Vec<i32>;
        let h_right: Vec<f64>;

        let m = x012.len() / 2;
        if x02[m] < x012[m] {
            x_left = x02;
            h_left = h02;
            x_right = x012;
            h_right = h012;
        }
        else {
            x_left = x012;
            h_left = h012;
            x_right = x02;
            h_right = h02;
        }

        for y in y0..(y2+1) {

            let x_left_this_y: i32 = x_left[(y - y0) as usize];
            let h_left_this_y: f64 = h_left[(y - y0) as usize];

            let x_right_this_y: i32 = x_right[(y - y0) as usize];
            let h_right_this_y: f64 = h_right[(y - y0) as usize];

            let h_values: Vec<f64> = cmath::f64_lerp_vec(
                x_left_this_y as f64,
                h_left_this_y,
                x_right_this_y as f64,
                h_right_this_y
            );

            for x in x_left[(y - y0) as usize]..x_right[(y - y0) as usize] {
                let shaded_color: u32 = self.scale_color(color, h_values[(x - x_left_this_y) as usize]);
                self.set_pixel(x, y, shaded_color);
            }
        }

    }

    fn scale_color(&mut self, color: u32, scalar: f64) -> u32 {
        let r = (self.get_red(color) as f64 * scalar + 0.5) as u8;
        let g = (self.get_green(color) as f64 * scalar + 0.5) as u8;
        let b = (self.get_blue(color) as f64 * scalar + 0.5) as u8;
        return self.get_color(r, g, b);
    }

    fn get_alpha(&mut self, color: u32) -> u8 {
        return (color / 0x01000000) as u8;
    }

    fn get_red(&mut self, color: u32) -> u8 {
        return ((color % 0x01000000) / 0x00010000) as u8;
    }

    fn get_green(&mut self, color: u32) -> u8 {
        return ((color % 0x00010000) / 0x00000100) as u8;
    }

    fn get_blue(&mut self, color: u32) -> u8 {
        return ((color % 0x00000100) / 0x00000001) as u8;
    }

    fn get_color(&mut self, r: u8, g: u8, b: u8) -> u32 {
        return  (r as u32)*256*256 + (g as u32)*256 + (b as u32);
    }

    pub fn render_scene(&mut self, camera: &CTransform, scene: &Vec<CModelInstance>) {

        /*
        For each model instance {
             - clone array of vertices
             - create array of rasterized points
            For each vertex in the cloned array {
                //Perhaps do the following with matrix multiplication?
                - scale according to model instance transform
                - rotate according to model instance transform
                - position according to model instance transform
                - position according to camera
                - rotate according to camera
                - rasterize?
            }
            For each triangle {
                - render triangle
            }
        }
         */

        for model_instance in scene.iter() {

            let mut vertices: Vec<(f64, f64, f64)> = model_instance.model.vertices.clone();
            let mut rasterized_vertices: Vec<(i32, i32)> = Vec::<(i32, i32)>::new();

            //Affine transformations
            for v in vertices.iter_mut() {
                //model's scale
                let k: f64 = model_instance.transform.scale;
                v.0 *= k;
                v.1 *= k;
                v.2 *= k;

                //model's rot
                let M = &model_instance.transform.rot;
                let (v1, v2, v3) = v.clone();
                v.0 = M.e11*v1 + M.e12*v2 + M.e13*v3;
                v.1 = M.e21*v1 + M.e22*v2 + M.e23*v3;
                v.2 = M.e31*v1 + M.e32*v2 + M.e33*v3;

                //model's pos
                let xyz = model_instance.transform.pos;
                v.0 += xyz.0;
                v.1 += xyz.1;
                v.2 += xyz.2;

                //camera's pos
                let xyz = camera.pos;
                v.0 -= xyz.0;
                v.1 -= xyz.1;
                v.2 -= xyz.2;

                //camera's rot
                let M = &camera.rot;
                let (v1, v2, v3) = v.clone();
                v.0 = M.e11*v1 + M.e21*v2 + M.e31*v3;
                v.1 = M.e12*v1 + M.e22*v2 + M.e32*v3;
                v.2 = M.e13*v1 + M.e23*v2 + M.e33*v3;

                //TODO: projection matrix?
            }

            //TODO: Do clipping here

            //Rasterization
            for v in vertices.iter() {
                let rv: (i32, i32) = self.rasterize(camera, v);
                rasterized_vertices.push(rv);
            }

            for triangle in model_instance.model.triangles.iter() {
                let p0 = rasterized_vertices[triangle.0];
                let p1 = rasterized_vertices[triangle.1];
                let p2 = rasterized_vertices[triangle.2];
                self.draw_triangle(p0.0, p0.1, p1.0, p1.1, p2.0, p2.1, triangle.3);
            }
        }
    }

    fn rasterize(&self, camera: &CTransform, v: &(f64, f64, f64)) -> (i32, i32) {

        let d = 1.0;

        return self.viewport_to_canvas( v.0*d/v.2, v.1*d/v.2 );
    }
    
    fn viewport_to_canvas(&self, x: f64, y: f64) -> (i32, i32) {
        let Cw = self.raster.width as f64;
        let Ch = self.raster.height as f64;
        let Vw = 1.0;
        let Vh = Ch/Cw;
        return ((x*Cw/Vw + 0.5) as i32 + 533, (y*Ch/Vh + 0.5) as i32 + 400);
    }

}

