
use std::backtrace;
use std::env::args;

use crate::graphics::craster::CRaster;
use crate::math::{cmath, cmatrix};
use crate::graphics::ccamera::CCamera;
use crate::graphics::cmodel_instance::CModelInstance;
use crate::graphics::cmodel;
use cmodel::CModel;
use cmatrix::*;
use crate::triangle;
use triangle::Triangle;

use super::ctransform::CTransform;

pub struct CRen {
    pub raster: CRaster,    //CRen owns its raster
}

pub struct Clipper {

}

impl Clipper {

    ///Clips the model according to the canonical boundary `-w < v < w` in clip space.
    pub fn clip(vertices: &mut Vec<(f64, f64, f64, f64)>, triangles: &mut Vec<Triangle>) {

        Clipper::clip_to_plane(vertices, triangles, Clipper::get_clip_near());
        Clipper::clip_to_plane(vertices, triangles, Clipper::get_clip_far());
        Clipper::clip_to_plane(vertices, triangles, Clipper::get_clip_right());
        Clipper::clip_to_plane(vertices, triangles, Clipper::get_clip_left());
        Clipper::clip_to_plane(vertices, triangles, Clipper::get_clip_top());
        Clipper::clip_to_plane(vertices, triangles, Clipper::get_clip_bottom());

    }

    ///Clips a list of vertices and triangles according to the closure `clip_function()`. This closure
    /// must take the arguments `usize, (f64, f64, f64, f64), (f64, f64, f64, f64), (f64, f64, f64, f64), u32` and return 
    /// a tuple `(Vec<(f64, f64, f64, f64)>, Vec<Triangle>)` containing the vertices and points that should exist.
    fn clip_to_plane
    <F: Fn(usize, (f64, f64, f64, f64), (f64, f64, f64, f64), (f64, f64, f64, f64), u32) -> (Vec<(f64, f64, f64, f64)>, Vec<Triangle>)>(
        vertices: &mut Vec<(f64, f64, f64, f64)>, 
        triangles: &mut Vec<Triangle>,
        clip_function: F) {

        let mut clipped_vertices = Vec::<(f64, f64, f64, f64)>::new();
        let mut clipped_triangles = Vec::<Triangle>::new();

        for t in triangles.iter() {

            let v0 = vertices[t.i0];
            let v1 = vertices[t.i1];
            let v2 = vertices[t.i2];

            let (new_vertices, new_triangles) = clip_function(clipped_vertices.len(), v0, v1, v2, t.color);
            
            for v in new_vertices {
                clipped_vertices.push(v);
            }

            for t in new_triangles.iter() {
                clipped_triangles.push(t.clone());
            }
        }

        vertices.clear();
        triangles.clear();

        for v in clipped_vertices {
            vertices.push(v);
        }

        for t in clipped_triangles.iter() {
            triangles.push(t.clone());
        }

    }

    pub fn get_clip_near() -> Box<dyn Fn(usize, (f64, f64, f64, f64), (f64, f64, f64, f64), (f64, f64, f64, f64), u32) -> (Vec<(f64, f64, f64, f64)>, Vec<Triangle>)> 
    {
        Box::new(
            |index: usize, v0: (f64, f64, f64, f64), v1: (f64, f64, f64, f64), v2: (f64, f64, f64, f64), color: u32| {

                let mut new_vertices = Vec::<(f64, f64, f64, f64)>::new();
                let mut new_triangles = Vec::<Triangle>::new();
        
                let mut n = 0; //number of points outside of boundary
        
                if v0.2 > v0.3 { n += 1 };
                if v1.2 > v1.3 { n += 1 };
                if v2.2 > v2.3 { n += 1 };
        
                if n == 0 {
                    new_vertices.push(v0);
                    new_vertices.push(v1);
                    new_vertices.push(v2);
                    new_triangles.push(Triangle {
                        i0: index,
                        i1: index + 1,
                        i2: index + 2,
                        color,
                    })
                }
                else if n == 1 {
                    /*
                    - find which vertex is out of bounds
                    - calculate the two midpoints
                    - append vertices
                    - add two triangles
                    */
        
                    let v_out: (f64, f64, f64, f64);
                    let v_in0: (f64, f64, f64, f64);
                    let v_in1: (f64, f64, f64, f64);
        
                    if v0.2 > v0.3 {
                        v_out = v0;
                        v_in0 = v1;
                        v_in1 = v2;
                    }
                    else if v1.2 > v1.3 {
                        v_out = v1;
                        v_in0 = v0;
                        v_in1 = v2;
                    }
                    else {
                        v_out = v2;
                        v_in0 = v0;
                        v_in1 = v1;
                    }
        
                    let t0 = (v_out.2 - v_out.3) / (v_in0.3 - v_in0.2 + v_out.2 - v_out.3);
                    let t1 = (v_out.2 - v_out.3) / (v_in1.3 - v_in1.2 + v_out.2 - v_out.3);
        
                    let p0 = (
                        (1.0 - t0)*v_out.0 + t0*v_in0.0,
                        (1.0 - t0)*v_out.1 + t0*v_in0.1,
                        (1.0 - t0)*v_out.2 + t0*v_in0.2,
                        (1.0 - t0)*v_out.3 + t0*v_in0.3);
                    let p1 = (
                        (1.0 - t1)*v_out.0 + t1*v_in1.0,
                        (1.0 - t1)*v_out.1 + t1*v_in1.1,
                        (1.0 - t1)*v_out.2 + t1*v_in1.2,
                        (1.0 - t1)*v_out.3 + t1*v_in1.3);
        
                    new_vertices.push(v_in0);
                    new_vertices.push(v_in1);
                    new_vertices.push(p0);
                    new_vertices.push(p1);
        
                    new_triangles.push(Triangle {
                        i0: index,
                        i1: index + 1,
                        i2: index + 2,
                        color,
                    });
        
                    new_triangles.push(Triangle {
                        i0: index + 1,
                        i1: index + 2,
                        i2: index + 3,
                        color,
                    });
        
                }
                else if n == 2 {
                    /*
                    - find which vertex is in-bounds
                    - calculate the two midpoints
                    - append vertices
                    - add one triangle
                    */
        
                    let v_in: (f64, f64, f64, f64);
                    let v_out0: (f64, f64, f64, f64);
                    let v_out1: (f64, f64, f64, f64);
        
                    if v0.2 <= v0.3 {
                        v_in = v0;
                        v_out0 = v1;
                        v_out1 = v2;
                    }
                    else if v1.2 <= v1.3 {
                        v_in = v1;
                        v_out0 = v0;
                        v_out1 = v2;
                    }
                    else {
                        v_in = v2;
                        v_out0 = v0;
                        v_out1 = v1;
                    }
        
                    let t0 = (v_in.2 - v_in.3) / (v_out0.3 - v_out0.2 + v_in.2 - v_in.3);
                    let t1 = (v_in.2 - v_in.3) / (v_out1.3 - v_out1.2 + v_in.2 - v_in.3);
                
                    let p0 = (
                        (1.0 - t0)*v_in.0 + t0*v_out0.0,
                        (1.0 - t0)*v_in.1 + t0*v_out0.1,
                        (1.0 - t0)*v_in.2 + t0*v_out0.2,
                        (1.0 - t0)*v_in.3 + t0*v_out0.3);
                    let p1 = (
                        (1.0 - t1)*v_in.0 + t1*v_out1.0,
                        (1.0 - t1)*v_in.1 + t1*v_out1.1,
                        (1.0 - t1)*v_in.2 + t1*v_out1.2,
                        (1.0 - t1)*v_in.3 + t1*v_out1.3);
        
                    new_vertices.push(v_in);
                    new_vertices.push(p0);
                    new_vertices.push(p1);
        
                    new_triangles.push(Triangle {
                        i0: index,
                        i1: index + 1,
                        i2: index + 2,
                        color,
                    });
                }
                else {
                    //do nothing!
                }
        
                (new_vertices, new_triangles)
            }
        )
    }

    pub fn get_clip_far() -> Box<dyn Fn(usize, (f64, f64, f64, f64), (f64, f64, f64, f64), (f64, f64, f64, f64), u32) -> (Vec<(f64, f64, f64, f64)>, Vec<Triangle>)> 
    {
        Box::new(
            |index: usize, v0: (f64, f64, f64, f64), v1: (f64, f64, f64, f64), v2: (f64, f64, f64, f64), color: u32| {

                let mut new_vertices = Vec::<(f64, f64, f64, f64)>::new();
                let mut new_triangles = Vec::<Triangle>::new();
        
                let mut n = 0; //number of points outside of boundary
        
                if v0.2 < -v0.3 { n += 1 };
                if v1.2 < -v1.3 { n += 1 };
                if v2.2 < -v2.3 { n += 1 };
        
                if n == 0 {
                    new_vertices.push(v0);
                    new_vertices.push(v1);
                    new_vertices.push(v2);
                    new_triangles.push(Triangle {
                        i0: index,
                        i1: index + 1,
                        i2: index + 2,
                        color,
                    })
                }
                else if n == 1 {
                    /*
                    - find which vertex is out of bounds
                    - calculate the two midpoints
                    - append vertices
                    - add two triangles
                    */
        
                    let v_out: (f64, f64, f64, f64);
                    let v_in0: (f64, f64, f64, f64);
                    let v_in1: (f64, f64, f64, f64);
        
                    if v0.2 < -v0.3 {
                        v_out = v0;
                        v_in0 = v1;
                        v_in1 = v2;
                    }
                    else if v1.2 < -v1.3 {
                        v_out = v1;
                        v_in0 = v0;
                        v_in1 = v2;
                    }
                    else {
                        v_out = v2;
                        v_in0 = v0;
                        v_in1 = v1;
                    }
        
                    let t0 = (v_out.2 + v_out.3) / (v_out.2 + v_out.3 - v_in0.3 - v_in0.2);
                    let t1 = (v_out.2 + v_out.3) / (v_out.2 + v_out.3 - v_in1.3 - v_in1.2);
        
                    let p0 = (
                        (1.0 - t0)*v_out.0 + t0*v_in0.0,
                        (1.0 - t0)*v_out.1 + t0*v_in0.1,
                        (1.0 - t0)*v_out.2 + t0*v_in0.2,
                        (1.0 - t0)*v_out.3 + t0*v_in0.3);
                    let p1 = (
                        (1.0 - t1)*v_out.0 + t1*v_in1.0,
                        (1.0 - t1)*v_out.1 + t1*v_in1.1,
                        (1.0 - t1)*v_out.2 + t1*v_in1.2,
                        (1.0 - t1)*v_out.3 + t1*v_in1.3);
        
                    new_vertices.push(v_in0);
                    new_vertices.push(v_in1);
                    new_vertices.push(p0);
                    new_vertices.push(p1);
        
                    new_triangles.push(Triangle {
                        i0: index,
                        i1: index + 1,
                        i2: index + 2,
                        color,
                    });
        
                    new_triangles.push(Triangle {
                        i0: index + 1,
                        i1: index + 2,
                        i2: index + 3,
                        color,
                    });
        
                }
                else if n == 2 {
                    /*
                    - find which vertex is in-bounds
                    - calculate the two midpoints
                    - append vertices
                    - add one triangle
                    */
        
                    let v_in: (f64, f64, f64, f64);
                    let v_out0: (f64, f64, f64, f64);
                    let v_out1: (f64, f64, f64, f64);
        
                    if v0.2 >= -v0.3 {
                        v_in = v0;
                        v_out0 = v1;
                        v_out1 = v2;
                    }
                    else if v1.2 >= -v1.3 {
                        v_in = v1;
                        v_out0 = v0;
                        v_out1 = v2;
                    }
                    else {
                        v_in = v2;
                        v_out0 = v0;
                        v_out1 = v1;
                    }
        
                    let t0 = (v_in.2 + v_in.3) / (v_in.2 + v_in.3 - v_out0.3 - v_out0.2);
                    let t1 = (v_in.2 + v_in.3) / (v_in.2 + v_in.3 - v_out1.3 - v_out1.2);
                
                    let p0 = (
                        (1.0 - t0)*v_in.0 + t0*v_out0.0,
                        (1.0 - t0)*v_in.1 + t0*v_out0.1,
                        (1.0 - t0)*v_in.2 + t0*v_out0.2,
                        (1.0 - t0)*v_in.3 + t0*v_out0.3);
                    let p1 = (
                        (1.0 - t1)*v_in.0 + t1*v_out1.0,
                        (1.0 - t1)*v_in.1 + t1*v_out1.1,
                        (1.0 - t1)*v_in.2 + t1*v_out1.2,
                        (1.0 - t1)*v_in.3 + t1*v_out1.3);
        
                    new_vertices.push(v_in);
                    new_vertices.push(p0);
                    new_vertices.push(p1);
        
                    new_triangles.push(Triangle {
                        i0: index,
                        i1: index + 1,
                        i2: index + 2,
                        color,
                    });
                }
                else {
                    //do nothing!
                }
        
                (new_vertices, new_triangles)
            }
        )
    }
    
    pub fn get_clip_right() -> Box<dyn Fn(usize, (f64, f64, f64, f64), (f64, f64, f64, f64), (f64, f64, f64, f64), u32) -> (Vec<(f64, f64, f64, f64)>, Vec<Triangle>)> 
    {
        Box::new(
            |index: usize, v0: (f64, f64, f64, f64), v1: (f64, f64, f64, f64), v2: (f64, f64, f64, f64), color: u32| {

                let mut new_vertices = Vec::<(f64, f64, f64, f64)>::new();
                let mut new_triangles = Vec::<Triangle>::new();
        
                let mut n = 0; //number of points outside of boundary
        
                if v0.0 > v0.3 { n += 1 };
                if v1.0 > v1.3 { n += 1 };
                if v2.0 > v2.3 { n += 1 };
        
                if n == 0 {
                    new_vertices.push(v0);
                    new_vertices.push(v1);
                    new_vertices.push(v2);
                    new_triangles.push(Triangle {
                        i0: index,
                        i1: index + 1,
                        i2: index + 2,
                        color,
                    })
                }
                else if n == 1 {
                    /*
                    - find which vertex is out of bounds
                    - calculate the two midpoints
                    - append vertices
                    - add two triangles
                    */
        
                    let v_out: (f64, f64, f64, f64);
                    let v_in0: (f64, f64, f64, f64);
                    let v_in1: (f64, f64, f64, f64);
        
                    if v0.0 > v0.3 {
                        v_out = v0;
                        v_in0 = v1;
                        v_in1 = v2;
                    }
                    else if v1.0 > v1.3 {
                        v_out = v1;
                        v_in0 = v0;
                        v_in1 = v2;
                    }
                    else {
                        v_out = v2;
                        v_in0 = v0;
                        v_in1 = v1;
                    }
        
                    let t0 = (v_out.0 - v_out.3) / (v_in0.3 - v_in0.0 + v_out.0 - v_out.3);
                    let t1 = (v_out.0 - v_out.3) / (v_in1.3 - v_in1.0 + v_out.0 - v_out.3);
        
                    let p0 = (
                        (1.0 - t0)*v_out.0 + t0*v_in0.0,
                        (1.0 - t0)*v_out.1 + t0*v_in0.1,
                        (1.0 - t0)*v_out.2 + t0*v_in0.2,
                        (1.0 - t0)*v_out.3 + t0*v_in0.3);
                    let p1 = (
                        (1.0 - t1)*v_out.0 + t1*v_in1.0,
                        (1.0 - t1)*v_out.1 + t1*v_in1.1,
                        (1.0 - t1)*v_out.2 + t1*v_in1.2,
                        (1.0 - t1)*v_out.3 + t1*v_in1.3);
        
                    new_vertices.push(v_in0);
                    new_vertices.push(v_in1);
                    new_vertices.push(p0);
                    new_vertices.push(p1);
        
                    new_triangles.push(Triangle {
                        i0: index,
                        i1: index + 1,
                        i2: index + 2,
                        color,
                    });
        
                    new_triangles.push(Triangle {
                        i0: index + 1,
                        i1: index + 2,
                        i2: index + 3,
                        color,
                    });
        
                }
                else if n == 2 {
                    /*
                    - find which vertex is in-bounds
                    - calculate the two midpoints
                    - append vertices
                    - add one triangle
                    */
        
                    let v_in: (f64, f64, f64, f64);
                    let v_out0: (f64, f64, f64, f64);
                    let v_out1: (f64, f64, f64, f64);
        
                    if v0.0 <= v0.3 {
                        v_in = v0;
                        v_out0 = v1;
                        v_out1 = v2;
                    }
                    else if v1.0 <= v1.3 {
                        v_in = v1;
                        v_out0 = v0;
                        v_out1 = v2;
                    }
                    else {
                        v_in = v2;
                        v_out0 = v0;
                        v_out1 = v1;
                    }
        
                    let t0 = (v_in.0 - v_in.3) / (v_out0.3 - v_out0.0 + v_in.0 - v_in.3);
                    let t1 = (v_in.0 - v_in.3) / (v_out1.3 - v_out1.0 + v_in.0 - v_in.3);
                
                    let p0 = (
                        (1.0 - t0)*v_in.0 + t0*v_out0.0,
                        (1.0 - t0)*v_in.1 + t0*v_out0.1,
                        (1.0 - t0)*v_in.2 + t0*v_out0.2,
                        (1.0 - t0)*v_in.3 + t0*v_out0.3);
                    let p1 = (
                        (1.0 - t1)*v_in.0 + t1*v_out1.0,
                        (1.0 - t1)*v_in.1 + t1*v_out1.1,
                        (1.0 - t1)*v_in.2 + t1*v_out1.2,
                        (1.0 - t1)*v_in.3 + t1*v_out1.3);
        
                    new_vertices.push(v_in);
                    new_vertices.push(p0);
                    new_vertices.push(p1);
        
                    new_triangles.push(Triangle {
                        i0: index,
                        i1: index + 1,
                        i2: index + 2,
                        color,
                    });
                }
                else {
                    //do nothing!
                }
        
                (new_vertices, new_triangles)
            }
        )
    }

    pub fn get_clip_left() -> Box<dyn Fn(usize, (f64, f64, f64, f64), (f64, f64, f64, f64), (f64, f64, f64, f64), u32) -> (Vec<(f64, f64, f64, f64)>, Vec<Triangle>)> 
    {
        Box::new(
            |index: usize, v0: (f64, f64, f64, f64), v1: (f64, f64, f64, f64), v2: (f64, f64, f64, f64), color: u32| {

                let mut new_vertices = Vec::<(f64, f64, f64, f64)>::new();
                let mut new_triangles = Vec::<Triangle>::new();
        
                let mut n = 0; //number of points outside of boundary
        
                if v0.0 < -v0.3 { n += 1 };
                if v1.0 < -v1.3 { n += 1 };
                if v2.0 < -v2.3 { n += 1 };
        
                if n == 0 {
                    new_vertices.push(v0);
                    new_vertices.push(v1);
                    new_vertices.push(v2);
                    new_triangles.push(Triangle {
                        i0: index,
                        i1: index + 1,
                        i2: index + 2,
                        color,
                    })
                }
                else if n == 1 {
                    /*
                    - find which vertex is out of bounds
                    - calculate the two midpoints
                    - append vertices
                    - add two triangles
                    */
        
                    let v_out: (f64, f64, f64, f64);
                    let v_in0: (f64, f64, f64, f64);
                    let v_in1: (f64, f64, f64, f64);
        
                    if v0.0 < -v0.3 {
                        v_out = v0;
                        v_in0 = v1;
                        v_in1 = v2;
                    }
                    else if v1.0 < -v1.3 {
                        v_out = v1;
                        v_in0 = v0;
                        v_in1 = v2;
                    }
                    else {
                        v_out = v2;
                        v_in0 = v0;
                        v_in1 = v1;
                    }
        
                    let t0 = (v_out.0 + v_out.3) / (v_out.0 + v_out.3 - v_in0.3 - v_in0.0);
                    let t1 = (v_out.0 + v_out.3) / (v_out.0 + v_out.3 - v_in1.3 - v_in1.0);
        
                    let p0 = (
                        (1.0 - t0)*v_out.0 + t0*v_in0.0,
                        (1.0 - t0)*v_out.1 + t0*v_in0.1,
                        (1.0 - t0)*v_out.2 + t0*v_in0.2,
                        (1.0 - t0)*v_out.3 + t0*v_in0.3);
                    let p1 = (
                        (1.0 - t1)*v_out.0 + t1*v_in1.0,
                        (1.0 - t1)*v_out.1 + t1*v_in1.1,
                        (1.0 - t1)*v_out.2 + t1*v_in1.2,
                        (1.0 - t1)*v_out.3 + t1*v_in1.3);
        
                    new_vertices.push(v_in0);
                    new_vertices.push(v_in1);
                    new_vertices.push(p0);
                    new_vertices.push(p1);
        
                    new_triangles.push(Triangle {
                        i0: index,
                        i1: index + 1,
                        i2: index + 2,
                        color,
                    });
        
                    new_triangles.push(Triangle {
                        i0: index + 1,
                        i1: index + 2,
                        i2: index + 3,
                        color,
                    });
        
                }
                else if n == 2 {
                    /*
                    - find which vertex is in-bounds
                    - calculate the two midpoints
                    - append vertices
                    - add one triangle
                    */
        
                    let v_in: (f64, f64, f64, f64);
                    let v_out0: (f64, f64, f64, f64);
                    let v_out1: (f64, f64, f64, f64);
        
                    if v0.0 >= -v0.3 {
                        v_in = v0;
                        v_out0 = v1;
                        v_out1 = v2;
                    }
                    else if v1.0 >= -v1.3 {
                        v_in = v1;
                        v_out0 = v0;
                        v_out1 = v2;
                    }
                    else {
                        v_in = v2;
                        v_out0 = v0;
                        v_out1 = v1;
                    }
        
                    let t0 = (v_in.2 + v_in.3) / (v_in.2 + v_in.3 - v_out0.3 - v_out0.2);
                    let t1 = (v_in.2 + v_in.3) / (v_in.2 + v_in.3 - v_out1.3 - v_out1.2);
                
                    let p0 = (
                        (1.0 - t0)*v_in.0 + t0*v_out0.0,
                        (1.0 - t0)*v_in.1 + t0*v_out0.1,
                        (1.0 - t0)*v_in.2 + t0*v_out0.2,
                        (1.0 - t0)*v_in.3 + t0*v_out0.3);
                    let p1 = (
                        (1.0 - t1)*v_in.0 + t1*v_out1.0,
                        (1.0 - t1)*v_in.1 + t1*v_out1.1,
                        (1.0 - t1)*v_in.2 + t1*v_out1.2,
                        (1.0 - t1)*v_in.3 + t1*v_out1.3);
        
                    new_vertices.push(v_in);
                    new_vertices.push(p0);
                    new_vertices.push(p1);
        
                    new_triangles.push(Triangle {
                        i0: index,
                        i1: index + 1,
                        i2: index + 2,
                        color,
                    });
                }
                else {
                    //do nothing!
                }
        
                (new_vertices, new_triangles)
            }
        )
    }

    pub fn get_clip_top() -> Box<dyn Fn(usize, (f64, f64, f64, f64), (f64, f64, f64, f64), (f64, f64, f64, f64), u32) -> (Vec<(f64, f64, f64, f64)>, Vec<Triangle>)> 
    {
        Box::new(
            |index: usize, v0: (f64, f64, f64, f64), v1: (f64, f64, f64, f64), v2: (f64, f64, f64, f64), color: u32| {

                let mut new_vertices = Vec::<(f64, f64, f64, f64)>::new();
                let mut new_triangles = Vec::<Triangle>::new();
        
                let mut n = 0; //number of points outside of boundary
        
                if v0.1 > v0.3 { n += 1 };
                if v1.1 > v1.3 { n += 1 };
                if v2.1 > v2.3 { n += 1 };
        
                if n == 0 {
                    new_vertices.push(v0);
                    new_vertices.push(v1);
                    new_vertices.push(v2);
                    new_triangles.push(Triangle {
                        i0: index,
                        i1: index + 1,
                        i2: index + 2,
                        color,
                    })
                }
                else if n == 1 {
                    /*
                    - find which vertex is out of bounds
                    - calculate the two midpoints
                    - append vertices
                    - add two triangles
                    */
        
                    let v_out: (f64, f64, f64, f64);
                    let v_in0: (f64, f64, f64, f64);
                    let v_in1: (f64, f64, f64, f64);
        
                    if v0.1 > v0.3 {
                        v_out = v0;
                        v_in0 = v1;
                        v_in1 = v2;
                    }
                    else if v1.1 > v1.3 {
                        v_out = v1;
                        v_in0 = v0;
                        v_in1 = v2;
                    }
                    else {
                        v_out = v2;
                        v_in0 = v0;
                        v_in1 = v1;
                    }
        
                    let t0 = (v_out.1 - v_out.3) / (v_in0.3 - v_in0.1 + v_out.1 - v_out.3);
                    let t1 = (v_out.1 - v_out.3) / (v_in1.3 - v_in1.1 + v_out.1 - v_out.3);
        
                    let p0 = (
                        (1.0 - t0)*v_out.0 + t0*v_in0.0,
                        (1.0 - t0)*v_out.1 + t0*v_in0.1,
                        (1.0 - t0)*v_out.2 + t0*v_in0.2,
                        (1.0 - t0)*v_out.3 + t0*v_in0.3);
                    let p1 = (
                        (1.0 - t1)*v_out.0 + t1*v_in1.0,
                        (1.0 - t1)*v_out.1 + t1*v_in1.1,
                        (1.0 - t1)*v_out.2 + t1*v_in1.2,
                        (1.0 - t1)*v_out.3 + t1*v_in1.3);
        
                    new_vertices.push(v_in0);
                    new_vertices.push(v_in1);
                    new_vertices.push(p0);
                    new_vertices.push(p1);
        
                    new_triangles.push(Triangle {
                        i0: index,
                        i1: index + 1,
                        i2: index + 2,
                        color,
                    });
        
                    new_triangles.push(Triangle {
                        i0: index + 1,
                        i1: index + 2,
                        i2: index + 3,
                        color,
                    });
        
                }
                else if n == 2 {
                    /*
                    - find which vertex is in-bounds
                    - calculate the two midpoints
                    - append vertices
                    - add one triangle
                    */
        
                    let v_in: (f64, f64, f64, f64);
                    let v_out0: (f64, f64, f64, f64);
                    let v_out1: (f64, f64, f64, f64);
        
                    if v0.1 <= v0.3 {
                        v_in = v0;
                        v_out0 = v1;
                        v_out1 = v2;
                    }
                    else if v1.1 <= v1.3 {
                        v_in = v1;
                        v_out0 = v0;
                        v_out1 = v2;
                    }
                    else {
                        v_in = v2;
                        v_out0 = v0;
                        v_out1 = v1;
                    }
        
                    let t0 = (v_in.1 - v_in.3) / (v_out0.3 - v_out0.1 + v_in.1 - v_in.3);
                    let t1 = (v_in.1 - v_in.3) / (v_out1.3 - v_out1.1 + v_in.1 - v_in.3);
                
                    let p0 = (
                        (1.0 - t0)*v_in.0 + t0*v_out0.0,
                        (1.0 - t0)*v_in.1 + t0*v_out0.1,
                        (1.0 - t0)*v_in.2 + t0*v_out0.2,
                        (1.0 - t0)*v_in.3 + t0*v_out0.3);
                    let p1 = (
                        (1.0 - t1)*v_in.0 + t1*v_out1.0,
                        (1.0 - t1)*v_in.1 + t1*v_out1.1,
                        (1.0 - t1)*v_in.2 + t1*v_out1.2,
                        (1.0 - t1)*v_in.3 + t1*v_out1.3);
        
                    new_vertices.push(v_in);
                    new_vertices.push(p0);
                    new_vertices.push(p1);
        
                    new_triangles.push(Triangle {
                        i0: index,
                        i1: index + 1,
                        i2: index + 2,
                        color,
                    });
                }
                else {
                    //do nothing!
                }
        
                (new_vertices, new_triangles)
            }
        )
    }

    pub fn get_clip_bottom() -> Box<dyn Fn(usize, (f64, f64, f64, f64), (f64, f64, f64, f64), (f64, f64, f64, f64), u32) -> (Vec<(f64, f64, f64, f64)>, Vec<Triangle>)> 
    {
        Box::new(
            |index: usize, v0: (f64, f64, f64, f64), v1: (f64, f64, f64, f64), v2: (f64, f64, f64, f64), color: u32| {

                let mut new_vertices = Vec::<(f64, f64, f64, f64)>::new();
                let mut new_triangles = Vec::<Triangle>::new();
        
                let mut n = 0; //number of points outside of boundary
        
                if v0.1 < -v0.3 { n += 1 };
                if v1.1 < -v1.3 { n += 1 };
                if v2.1 < -v2.3 { n += 1 };
        
                if n == 0 {
                    new_vertices.push(v0);
                    new_vertices.push(v1);
                    new_vertices.push(v2);
                    new_triangles.push(Triangle {
                        i0: index,
                        i1: index + 1,
                        i2: index + 2,
                        color,
                    })
                }
                else if n == 1 {
                    /*
                    - find which vertex is out of bounds
                    - calculate the two midpoints
                    - append vertices
                    - add two triangles
                    */
        
                    let v_out: (f64, f64, f64, f64);
                    let v_in0: (f64, f64, f64, f64);
                    let v_in1: (f64, f64, f64, f64);
        
                    if v0.1 < -v0.3 {
                        v_out = v0;
                        v_in0 = v1;
                        v_in1 = v2;
                    }
                    else if v1.1 < -v1.3 {
                        v_out = v1;
                        v_in0 = v0;
                        v_in1 = v2;
                    }
                    else {
                        v_out = v2;
                        v_in0 = v0;
                        v_in1 = v1;
                    }
        
                    let t0 = (v_out.1 + v_out.3) / (v_out.1 + v_out.3 - v_in0.3 - v_in0.1);
                    let t1 = (v_out.1 + v_out.3) / (v_out.1 + v_out.3 - v_in1.3 - v_in1.1);
        
                    let p0 = (
                        (1.0 - t0)*v_out.0 + t0*v_in0.0,
                        (1.0 - t0)*v_out.1 + t0*v_in0.1,
                        (1.0 - t0)*v_out.2 + t0*v_in0.2,
                        (1.0 - t0)*v_out.3 + t0*v_in0.3);
                    let p1 = (
                        (1.0 - t1)*v_out.0 + t1*v_in1.0,
                        (1.0 - t1)*v_out.1 + t1*v_in1.1,
                        (1.0 - t1)*v_out.2 + t1*v_in1.2,
                        (1.0 - t1)*v_out.3 + t1*v_in1.3);
        
                    new_vertices.push(v_in0);
                    new_vertices.push(v_in1);
                    new_vertices.push(p0);
                    new_vertices.push(p1);
        
                    new_triangles.push(Triangle {
                        i0: index,
                        i1: index + 1,
                        i2: index + 2,
                        color,
                    });
        
                    new_triangles.push(Triangle {
                        i0: index + 1,
                        i1: index + 2,
                        i2: index + 3,
                        color,
                    });
        
                }
                else if n == 2 {
                    /*
                    - find which vertex is in-bounds
                    - calculate the two midpoints
                    - append vertices
                    - add one triangle
                    */
        
                    let v_in: (f64, f64, f64, f64);
                    let v_out0: (f64, f64, f64, f64);
                    let v_out1: (f64, f64, f64, f64);
        
                    if v0.1 >= -v0.3 {
                        v_in = v0;
                        v_out0 = v1;
                        v_out1 = v2;
                    }
                    else if v1.1 >= -v1.3 {
                        v_in = v1;
                        v_out0 = v0;
                        v_out1 = v2;
                    }
                    else {
                        v_in = v2;
                        v_out0 = v0;
                        v_out1 = v1;
                    }
        
                    let t0 = (v_in.1 + v_in.3) / (v_in.1 + v_in.3 - v_out0.3 - v_out0.1);
                    let t1 = (v_in.1 + v_in.3) / (v_in.1 + v_in.3 - v_out1.3 - v_out1.1);
                
                    let p0 = (
                        (1.0 - t0)*v_in.0 + t0*v_out0.0,
                        (1.0 - t0)*v_in.1 + t0*v_out0.1,
                        (1.0 - t0)*v_in.2 + t0*v_out0.2,
                        (1.0 - t0)*v_in.3 + t0*v_out0.3);
                    let p1 = (
                        (1.0 - t1)*v_in.0 + t1*v_out1.0,
                        (1.0 - t1)*v_in.1 + t1*v_out1.1,
                        (1.0 - t1)*v_in.2 + t1*v_out1.2,
                        (1.0 - t1)*v_in.3 + t1*v_out1.3);
        
                    new_vertices.push(v_in);
                    new_vertices.push(p0);
                    new_vertices.push(p1);
        
                    new_triangles.push(Triangle {
                        i0: index,
                        i1: index + 1,
                        i2: index + 2,
                        color,
                    });
                }
                else {
                    //do nothing!
                }
        
                (new_vertices, new_triangles)
            }
        )
    }

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

    ///Renders model instances to the raster.
    pub fn render_scene(&mut self, camera: &CCamera, scene: &Vec<CModelInstance>) {

        let fov: f64 = camera.fov; //60 degrees
        let aspect_ratio: f64 = (self.raster.width as f64) / (self.raster.height as f64);

        let P = {   //OpenGL-Style Projeciton matrix

            let (n, f) = (1.0, 50.0);
            let r = (fov/2.0).sin();
            let l = -1.0*r;
            let t = r / aspect_ratio;
            let b = -1.0*t;

            Matrix4x4 {
                e11: 2.0*n/(r-l),
                e12: 0.0,
                e13: 1.0*(r+l)/(r-l),
                e14: 0.0,
                e21: 0.0,
                e22: 2.0*n/(t-b),
                e23: 1.0*(t+b)/(t-b),
                e24: 0.0,
                e31: 0.0,
                e32: 0.0,
                e33: -1.0*(f+n)/(f-n),
                e34: 2.0*f*n/(f-n),
                e41: 0.0,
                e42: 0.0,
                e43: 1.0,
                e44: 0.0,
            }
        };

        ////Projection Matrix Test
        // {
        //     let (n, f) = (1.0, 50.0);
        //     let r = (fov/2.0).sin();
        //     let l = -1.0*r;
        //     let t = r / aspect_ratio;
        //     let b = -1.0*t;

        //     let mut nrt = (r, t, n, 1.0);
        //     let mut nlb = (l, b, n, 1.0);
        //     let mut frt = (r*f/n, t*f/n, f, 1.0);
        //     let mut flb = (l*f/n, b*f/n, f, 1.0);

        //     println!("nrt: ({}, {}, {}, {})", nrt.0, nrt.1, nrt.2, nrt.3);
        //     println!("nlb: ({}, {}, {}, {})", nlb.0, nlb.1, nlb.2, nlb.3);
        //     println!("frt: ({}, {}, {}, {})", frt.0, frt.1, frt.2, frt.3);
        //     println!("flb: ({}, {}, {}, {})", flb.0, flb.1, flb.2, flb.3);

        //     println!("*apply P*");
        //     P.applyTo(&mut nrt);
        //     P.applyTo(&mut nlb);
        //     P.applyTo(&mut frt);
        //     P.applyTo(&mut flb);

        //     println!("nrt: ({}, {}, {}, {})", nrt.0, nrt.1, nrt.2, nrt.3);
        //     println!("nlb: ({}, {}, {}, {})", nlb.0, nlb.1, nlb.2, nlb.3);
        //     println!("frt: ({}, {}, {}, {})", frt.0, frt.1, frt.2, frt.3);
        //     println!("flb: ({}, {}, {}, {})", flb.0, flb.1, flb.2, flb.3);

        //     println!("*w-divide*");
        //     let nrt_w = nrt.3;
        //     let nlb_w = nlb.3;
        //     let frt_w = frt.3;
        //     let flb_w = flb.3;

        //     nrt.0 /= nrt_w;
        //     nrt.1 /= nrt_w;
        //     nrt.2 /= nrt_w;
        //     nrt.3 /= nrt_w;

        //     nlb.0 /= nlb_w;
        //     nlb.1 /= nlb_w;
        //     nlb.2 /= nlb_w;
        //     nlb.3 /= nlb_w;

        //     frt.0 /= frt_w;
        //     frt.1 /= frt_w;
        //     frt.2 /= frt_w;
        //     frt.3 /= frt_w;

        //     flb.0 /= flb_w;
        //     flb.1 /= flb_w;
        //     flb.2 /= flb_w;
        //     flb.3 /= flb_w;

        //     println!("nrt: ({}, {}, {}, {})", nrt.0, nrt.1, nrt.2, nrt.3);
        //     println!("nlb: ({}, {}, {}, {})", nlb.0, nlb.1, nlb.2, nlb.3);
        //     println!("frt: ({}, {}, {}, {})", frt.0, frt.1, frt.2, frt.3);
        //     println!("flb: ({}, {}, {}, {})\n", flb.0, flb.1, flb.2, flb.3);
        // }

        let C = {
            let mut C3x3 = Matrix3x3::new_rot(camera.transform.rot.0, camera.transform.rot.1, camera.transform.rot.2);
            C3x3.transpose();
            let mut C4x4 = C3x3.as4x4();
            let mut t = camera.transform.pos.clone();
            C3x3.applyTo(&mut t);
            C4x4.e14 = -1.0*t.0;
            C4x4.e24 = -1.0*t.1;
            C4x4.e34 = -1.0*t.2;
            C4x4
        };

        let A = cmatrix::matrix_4x4_mult(&P, &C);   //Perspective projection, camera rotation, and camera displacement matrix

        let mut z_buffer = Vec::<f64>::new();
        for z in 0..self.raster.data.len() {
            z_buffer.push(-1.0);
        }

        /* Vertex Shader */

        for model_instance in scene.iter() {

            let mut vertices = Vec::<(f64, f64, f64, f64)>::new();
            let mut triangles = model_instance.model.triangles.clone();

            for v in model_instance.model.vertices.iter() { vertices.push((v.0, v.1, v.2, 1.0)); }

            let B = model_instance.transform.asMatrix();    //Model transform matrix
            let M = cmatrix::matrix_4x4_mult(&A, &B);

            //Model space --> Clip space
            for v in vertices.iter_mut() {
                M.applyTo(v);
            }

            //TODO: Clipping
            Clipper::clip(&mut vertices, &mut triangles);
            // self.clip(&mut vertices, &mut triangles);

            //Clip space --> NDC space
            for v in vertices.iter_mut() {
                if v.3 == 0.0 { println!("Divide by zero! aaa"); }
                let w = v.3;
                v.0 /= w;
                v.1 /= w;
                v.2 /= w;
                v.3 /= w;
            }

            //Lighting stuff? idk

            //NDC space --> Screen space
            let mut M = Matrix4x4::new();
            M.e11 = (self.raster.width as f64)/2.0;
            M.e22 = -1.0*(self.raster.height as f64)/2.0;
            M.e14 = (self.raster.width as f64)/2.0;
            M.e24 = (self.raster.height as f64)/2.0;

            for v in vertices.iter_mut() {
                M.applyTo(v);
            }

            //Draw triangles
            for t in triangles {
                
                let v0: (f64, f64, f64, f64) = vertices[t.i0];
                let v1 = vertices[t.i1];
                let v2 = vertices[t.i2];

                let x0 = (v0.0 + 0.5) as i32;
                let y0 = (v0.1 + 0.5) as i32;
                let z0 = v0.2;
                let x1 = (v1.0 + 0.5) as i32;
                let y1 = (v1.1 + 0.5) as i32;
                let z1 = v1.2;
                let x2 = (v2.0 + 0.5) as i32;
                let y2 = (v2.1 + 0.5) as i32;
                let z2 = v2.2;

                let color = t.color;

                //Fill triangle, unless z-buffer prevents it
                self.fill_triangle_with_z_buffer(&mut z_buffer, x0, y0, z0, x1, y1, z1, x2, y2, z2, color);
                // self.draw_triangle(x0, y0, x1, y1, x2, y2, color);

            }

        }

    }

    pub fn fill_triangle_with_z_buffer(&mut self, z_buffer: &mut Vec<f64>, mut x0: i32, mut y0: i32, mut z0: f64, mut x1: i32, mut y1: i32, mut z1: f64, mut x2: i32, mut y2: i32, mut z2: f64, color: u32) {
        
        //swap points such that y0 < y1 < y2
        if y1 < y0 {
            let tx0 = x0;
            let ty0 = y0;
            let tz0 = z0;
            x0 = x1;
            y0 = y1;
            z0 = z1;
            x1 = tx0;
            y1 = ty0;
            z1 = tz0;
        }

        if y2 < y0 {
            let tx0 = x0;
            let ty0 = y0;
            let tz0 = z0;
            x0 = x2;
            y0 = y2;
            z0 = z2;
            x2 = tx0;
            y2 = ty0;
            z2 = tz0;
        }

        if y2 < y1 {
            let tx1 = x1;
            let ty1 = y1;
            let tz1 = z1;
            x1 = x2;
            y1 = y2;
            z1 = z2;
            x2 = tx1;
            y2 = ty1;
            z2 = tz1;
        }

        let mut x01: Vec<i32> = cmath::i32_lerp_vec(y0, x0, y1, x1);
        let mut z01: Vec<f64> = cmath::f64_lerp_vec(y0 as f64, z0, y1 as f64, z1);

        let mut x12: Vec<i32> = cmath::i32_lerp_vec(y1, x1, y2, x2);
        let mut z12: Vec<f64> = cmath::f64_lerp_vec(y1 as f64, z1, y2 as f64, z2);

        let mut x02: Vec<i32> = cmath::i32_lerp_vec(y0, x0, y2, x2);
        let mut z02: Vec<f64> = cmath::f64_lerp_vec(y0 as f64, z0, y2 as f64, z2);

        x01.pop();
        z01.pop();

        x01.append(&mut x12);
        z01.append(&mut z12);
    
        let x012 = x01;
        let z012 = z01;

        let x_left: Vec<i32>;
        let z_left: Vec<f64>;

        let x_right: Vec<i32>;
        let z_right: Vec<f64>;

        let m = x012.len() / 2;
        if x02[m] < x012[m] {
            x_left = x02;
            z_left = z02;
            x_right = x012;
            z_right = z012;
        }
        else {
            x_left = x012;
            z_left = z012;
            x_right = x02;
            z_right = z02;
        }

        for y in y0..(y2+1) {

            let x_left_this_y: i32 = x_left[(y - y0) as usize];
            let z_left_this_y: f64 = z_left[(y - y0) as usize];

            let x_right_this_y: i32 = x_right[(y - y0) as usize];
            let z_right_this_y: f64 = z_right[(y - y0) as usize];

            let z_values: Vec<f64> = cmath::f64_lerp_vec(
                x_left_this_y as f64,
                z_left_this_y,
                x_right_this_y as f64,
                z_right_this_y
            );

            for x in x_left[(y - y0) as usize]..x_right[(y - y0) as usize] {
                if x < 0 || x >= self.raster.width as i32 || y < 0 || y >= self.raster.height as i32 { }
                else {
                    if z_buffer[ (x + (self.raster.width as i32)*y) as usize ] < z_values[(x - x_left_this_y) as usize] {
                        self.set_pixel(x, y, color);
                        z_buffer[(x + (self.raster.width as i32)*y) as usize] = z_values[(x - x_left_this_y) as usize];
                    }
                }                
            }
        }

    }

}

