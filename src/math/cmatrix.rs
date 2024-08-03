
///A 3x3 f64 matrix. Fields are marked using row-column notation (e12 is the element in row 1, column 2).
pub struct Matrix3x3 {
    pub e11: f64,
    pub e12: f64,
    pub e13: f64,
    pub e21: f64,
    pub e22: f64,
    pub e23: f64,
    pub e31: f64,
    pub e32: f64,
    pub e33: f64,
}

///A 3x1 f64 matrix. Fields are marked using by row.
pub struct Vec3 {
    pub e1: f64,
    pub e2: f64,
    pub e3: f64,
}

impl Matrix3x3 {
    pub fn new() -> Matrix3x3 {
        return Matrix3x3 {
            e11: 1.0,
            e12: 0.0,
            e13: 0.0,
            e21: 0.0,
            e22: 1.0,
            e23: 0.0,
            e31: 0.0,
            e32: 0.0,
            e33: 1.0,
        };
    }

    pub fn new_rot(x_rot: f64, y_rot: f64, z_rot: f64) -> Matrix3x3 {
        let mut X = Matrix3x3::new();
        let mut Y = Matrix3x3::new();
        let mut Z = Matrix3x3::new();

        X.e22 = x_rot.cos();
        X.e32 = x_rot.sin();
        X.e23 = -1.0*X.e32; // -1.0*x_rot.sin()
        X.e33 = X.e22;      //x_rot.cos()

        Y.e11 = y_rot.cos();
        Y.e13 = y_rot.sin();
        Y.e31 = -1.0*Y.e13; // -1.0*y_rot.sin()
        Y.e33 = Y.e11;      //y_rot.cos()

        Z.e11 = z_rot.cos();
        Z.e21 = z_rot.sin();
        Z.e12 = -1.0*Z.e21; // -1.0*z_rot.sin()
        Z.e22 = Z.e11;      //z_rot.cos()

        return matrix_3x3_mult(&Z, &matrix_3x3_mult(&Y, &X));
    }
}

impl Vec3 {
    pub fn new() -> Vec3 {
        return Vec3 {
            e1: 0.0,
            e2: 0.0,
            e3: 0.0,
        };
    }
}


/*
                    b11 b12 b13
                    b21 b22 b23
                    b31 b32 b33

    a11 a12 a13     e11 e12 e13
    a21 a22 a23     e21 e22 e23
    a31 a32 a33     e31 e32 e33
*/

///3x3 Matrix multiplication. Computes M = A*B.
pub fn matrix_3x3_mult(A: &Matrix3x3, B: &Matrix3x3) -> Matrix3x3 {
    return Matrix3x3 {
        e11: A.e11*B.e11 + A.e12*B.e21 + A.e13*B.e31,
        e12: A.e11*B.e12 + A.e12*B.e22 + A.e13*B.e32,
        e13: A.e11*B.e13 + A.e12*B.e23 + A.e13*B.e33,

        e21: A.e21*B.e11 + A.e22*B.e21 + A.e23*B.e31,
        e22: A.e21*B.e12 + A.e22*B.e22 + A.e23*B.e32,
        e23: A.e21*B.e13 + A.e22*B.e23 + A.e23*B.e33,

        e31: A.e31*B.e11 + A.e32*B.e21 + A.e33*B.e31,
        e32: A.e31*B.e12 + A.e32*B.e22 + A.e33*B.e32,
        e33: A.e31*B.e13 + A.e32*B.e23 + A.e33*B.e33,
    }
}