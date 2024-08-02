pub struct CModel {
    pub vertices: Vec<(f64, f64, f64)>,
    pub triangles: Vec<(usize, usize, usize, u32)>,
}

pub fn new_cube() -> CModel {

    let red = 0x00ff0000;
    let green = 0x0000ff00;
    let blue = 0x000000ff;
    let yellow = 0x00ffff00;
    let purple = 0x00a020f0;
    let cyan = 0x0000ffff;

    let mut v: Vec<(f64, f64, f64)> = Vec::<(f64, f64, f64)>::new();
    let mut t: Vec<(usize, usize, usize, u32)> = Vec::<(usize, usize, usize, u32)>::new();

    v.push((1.0, 1.0, 1.0));        //0
    v.push((-1.0, 1.0, 1.0));       //1
    v.push((-1.0, -1.0, 1.0));      //2
    v.push((1.0, -1.0, 1.0));       //3
    v.push((1.0, 1.0, -1.0));       //4
    v.push((-1.0, 1.0, -1.0));      //5
    v.push((-1.0, -1.0, -1.0));     //6
    v.push((1.0, -1.0, -1.0));      //7

    t.push((0, 1, 2, red));         //0
    t.push((0, 2, 3, red));         //1
    t.push((4, 0, 3, green));       //2
    t.push((4, 3, 7, green));       //3
    t.push((5, 4, 7, blue));        //4
    t.push((5, 7, 6, blue));        //5
    t.push((1, 5, 6, yellow));      //6
    t.push((1, 6, 2, yellow));      //7
    t.push((4, 4, 1, purple));      //8
    t.push((4, 1, 0, purple));      //9
    t.push((2, 6, 7, cyan));        //10
    t.push((2, 7, 3, cyan));        //11

    return CModel {
        vertices: v,
        triangles: t,
    };
}