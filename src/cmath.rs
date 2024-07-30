pub fn usize_lerp(i0: usize, d0: usize, i1: usize, d1: usize) -> Vec<usize> {
    let mut v: Vec<usize> = Vec::<usize>::new();
    let a: f64 = ((d1 - d0) as f64) / ((i1 - i0) as f64);
    let mut d: f64 = d0 as f64;
    for i in i0..i1 {
        v.push(d as usize);
        d = d + a;
    }
    return v;
}

pub fn sin(theta: f64) -> f64 {
    return theta.sin();
}

pub fn cos(theta: f64) -> f64 {
    return theta.cos();
}

/*
Interpolate (i0, d0, i1, d1) {
    values = []
    a = (d1 - d0) / (i1 - i0)
    d = d0
    for i = i0 to i1 {
        values.append(d)
        d = d + a
    }
    return values
}
 */