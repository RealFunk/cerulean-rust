pub fn usize_lerp_vec(i0: usize, d0: usize, i1: usize, d1: usize) -> Vec<usize> {
    let mut v: Vec<usize> = Vec::<usize>::new();
    let a: f64 = ((d1 - d0) as f64) / ((i1 - i0) as f64);
    let mut d: f64 = d0 as f64;
    for i in i0..i1 {
        v.push(d as usize);
        d = d + a;
    }
    return v;
}

pub fn i32_lerp_vec(i0: i32, d0: i32, i1: i32, d1: i32) -> Vec<i32> {
    if i0 >= i1 {
        return vec![(d0+d1)/2; 1];
    }

    let mut v: Vec<i32> = Vec::<i32>::new();
    let mut d: f64 = d0 as f64;
    let change = (d1 - d0)  as f64 / ((i1 - i0) as f64);

    for i in i0..i1 {
        v.push((d + 0.5) as i32);
        d += change;
    }
    v.push(d1);
    return v;
}

pub fn f64_lerp_vec(i0: f64, d0: f64, i1: f64, d1: f64) -> Vec<f64> {
    if i0 >= i1 {
        return vec![(d0+d1)/2.0; 1];
    }

    let mut v: Vec<f64> = Vec::<f64>::new();
    let mut d: f64 = d0 as f64;
    let change = (d1 - d0) / (i1 - i0);

    for i in (i0 as i32)..(i1 as i32) {
        v.push(d);
        d += change;
    }
    v.push(d1);
    return v;
}

pub fn u32_lerp(a: u32, b: u32, f: f64) -> u32 {
    let m: f64 = b as f64 - a as f64;
    return (m*f + (a as f64) + 0.5) as u32;
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