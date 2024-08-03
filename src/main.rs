
pub mod framework;
pub mod graphics;
pub mod math;

use std::clone;

use framework::*;
use graphics::*;
use math::*;

use cren::CRen;
use cwin::CWin;
use cmodel::CModel;
use cmodel_instance::CModelInstance;
use ctransform::CTransform;
use ccamera::CCamera;
use math::cmatrix;
use cmatrix::Matrix3x3;
use cmatrix::Vec3;

fn main() {

    let mut angle: f64 = 0.0;

    let mut cube1: CModelInstance = cmodel_instance::new_cube(-2.0, 0.0, 10.0);
    let mut cube2: CModelInstance = cmodel_instance::new_cube(2.0, 0.0, 10.0);
    let mut cube3: CModelInstance = cmodel_instance::new_cube(3.0, 1.0, 15.0);

    let mut R: Matrix3x3 = Matrix3x3::new_rot(0.1, 0.1, 0.1);
        cube1.transform.rot = R;

    let mut camera: CTransform = ctransform::new(0.0, 0.0, 0.0);
    let mut ren: CRen = CRen::new(1066, 800);
    let mut win: cwin::CWin = CWin::new(&ren.raster);

    let mut scene: Vec<CModelInstance> = Vec::<CModelInstance>::new();  //The CModelInstances live here!!
    scene.push(cube1);
    scene.push(cube2);
    scene.push(cube3);

    while win.is_open() {

        angle += 0.025;
        // if angle > 6.28 { angle -= 6.28; };
        let mut R: Matrix3x3 = Matrix3x3::new_rot(0.5*angle, angle, -0.1*angle);
        scene[0].transform.rot = R;

        ren.render_scene(&camera, &scene);
        win.draw(&ren.raster);
        ren.clear();

    };
}
    