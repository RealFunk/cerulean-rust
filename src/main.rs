
mod lib;
use lib::{framework, graphics, math};
use framework::cwin;
// use framework::cwin;
// use graphics::{
//     ccamera, 
//     cmodel_instance, 
//     cmodel, 
//     craster, 
//     cren, 
//     ctransform 
// };
// use math::cmath;

// mod cwin;
// mod cren;
// mod cmath;
// mod cmodel;
// mod cmodel_instance;
// mod ctransform;
// mod ccamera;

use std::clone;

use cren::CRen;
use cwin::CWin;
use cmodel::CModel;
use cmodel_instance::CModelInstance;
use ctransform::CTransform;
use ccamera::CCamera;

fn main() {

    // let mut x = 100;
    // let mut y = 0;

    // let mut theta: f64 = 0.0;

    let cube1: CModelInstance = cmodel_instance::new_cube(-2.0, 0.0, 10.0);
    let cube2: CModelInstance = cmodel_instance::new_cube(2.0, 0.0, 10.0);

    let mut ren: CRen = CRen::new(1066, 800);
    let mut win: cwin::CWin = CWin::new(&ren.raster);

    while win.is_open() {

        // theta += 0.01;
        // if theta == 2.0*3.14 { theta -= 2.0*3.14; }
        // x = (250.0 * theta.cos()) as i32;
        // y = (250.0 * theta.sin()) as i32;
        
        // ren.fill_shaded_triangle(100 + x, 100 + y, 1.0, 300 - x, 400 + y, 0.5, 200 + x, 700 - y, 0.0, 
        //     0x0000ff00);

        // ren.fill_shaded_triangle(400, 200, 0.0, 400, 400, 1.0, 400 + x, 400 + y, 0.0, 
        //     0x0000ff00);

        render_model_instance(&mut ren, &cube1);
        render_model_instance(&mut ren, &cube2);

        win.draw(&ren.raster);
        ren.clear();

    };

}

fn render_model_instance(ren: &mut CRen, instance: &CModelInstance) {
    let mut projected: Vec<(i32, i32)> = Vec::<(i32, i32)>::new();
    let mut vertices: Vec<(f64, f64, f64)> = instance.model.vertices.clone();

    for v in vertices.iter_mut() {
        scale_vec(v, instance.transform.scale);
        //rot_vec()
        transform_vec(v, instance.transform.pos);
        projected.push(point_to_pixel_coords(v));
    }

    for t in instance.model.triangles.iter() {
        ren.draw_triangle(
            projected[t.0].0, projected[t.0].1, 
            projected[t.1].0, projected[t.1].1, 
            projected[t.2].0, projected[t.2].1, 
            t.3
        );
    }
}

fn scale_vec(v: &mut (f64, f64, f64), k: f64) {
    v.0 *= k;
    v.1 *= k;
    v.2 *= k;
}

fn transform_vec(v: &mut (f64, f64, f64), pos: (f64, f64, f64)) {
    v.0 += pos.0;
    v.1 += pos.1;
    v.2 += pos.2;
}

fn render_model(ren: &mut CRen, model: &CModel) {

    let mut projected: Vec<(i32, i32)> = Vec::<(i32, i32)>::new();
    let mut vertices: Vec<(f64, f64, f64)> = model.vertices.clone();

    for v in vertices.iter_mut() {
        v.0 += -1.5;
        v.1 +=  0.0;
        v.2 +=  10.0;
    }

    for v in vertices.iter() {
        projected.push(point_to_pixel_coords(v));
    }

    for t in model.triangles.iter() {
        ren.draw_triangle(
            projected[t.0].0, projected[t.0].1, 
            projected[t.1].0, projected[t.1].1, 
            projected[t.2].0, projected[t.2].1, 
            t.3
        );
    }
}

fn point_to_pixel_coords(v: &(f64, f64, f64)) -> (i32, i32) {
    let d = 1.0;
    return viewpoirt_to_canvas( v.0*d/v.2, v.1*d/v.2 );
}

fn viewpoirt_to_canvas(x: f64, y: f64) -> (i32, i32) {
    let Cw = 1066.0;
    let Ch = 800.0;
    let Vw = 1.0;
    let Vh = 0.7505;
    return ((x*Cw/Vw + 0.5) as i32 + 533, (y*Ch/Vh + 0.5) as i32 + 400);
}