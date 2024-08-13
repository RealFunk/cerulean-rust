
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

    // let mut cube1: CModelInstance = cmodel_instance::new_cube(-2.0, 0.0, 10.0);
    // let mut cube2: CModelInstance = cmodel_instance::new_cube(2.0, 0.0, 10.0);
    let mut cube3: CModelInstance = cmodel_instance::new_cube(0.0, 0.0, 10.0);

    let mut camera: CCamera = ccamera::new();
    let mut ren: CRen = CRen::new(1066, 800);
    let mut win: cwin::CWin = CWin::new(&ren.raster);

    let mut scene: Vec<CModelInstance> = Vec::<CModelInstance>::new();  //The CModelInstances live here!!
    // scene.push(cube1);
    // scene.push(cube2);
    scene.push(cube3);

    while win.is_open() {

        scene[0].transform.rot.0 += 0.01;
        scene[0].transform.rot.1 += 0.005;
        scene[0].transform.rot.2 += 0.0025;

        // scene[1].transform.rot.1 += 0.01;
        // scene[1].transform.rot.2 -= 0.005;
        // scene[1].transform.rot.0 += 0.0025;

        // scene[0].transform.rot.1 -= 0.001;
        // scene[0].transform.rot.0 += 0.0005;
        // scene[0].transform.rot.2 -= 0.00025;

        // scene[0].transform.pos.2 += 0.05;

        // camera.transform.rot.1 += 0.001;

        // camera.transform.rot.0 -= 0.01;

        ren.render_scene(&camera, &scene);
        win.draw(&ren.raster);
        ren.clear();

    };

}
