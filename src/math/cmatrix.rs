
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

pub struct Matrix4x4 {
    pub e11: f64,
    pub e12: f64,
    pub e13: f64,
    pub e14: f64,
    pub e21: f64,
    pub e22: f64,
    pub e23: f64,
    pub e24: f64,
    pub e31: f64,
    pub e32: f64,
    pub e33: f64,
    pub e34: f64,
    pub e41: f64,
    pub e42: f64,
    pub e43: f64,
    pub e44: f64,
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

    ///Computes v' = Mv and returns v'.
    pub fn applyTo(&self, v: &mut (f64, f64, f64)) {
        let (v1, v2, v3) = (v.0, v.1, v.2);
        v.0 = self.e11*v1 + self.e12*v2 + self.e13*v3;
        v.1 = self.e21*v1 + self.e22*v2 + self.e23*v3;
        v.2 = self.e31*v1 + self.e32*v2 + self.e33*v3;
    }

    ///Transposes this mutable matrix. To return a transposed copy of the matrix, please do `M.transpose_copy();`
    pub fn transpose(&mut self) {
        let M: Matrix3x3 = Matrix3x3 {
            e11: self.e11,
            e12: self.e21,
            e13: self.e31,
            e21: self.e12,
            e22: self.e22,
            e23: self.e32,
            e31: self.e13,
            e32: self.e32,
            e33: self.e33,
        };

        self.e11 = M.e11;
        self.e12 = M.e12;
        self.e13 = M.e13;
        self.e21 = M.e21;
        self.e22 = M.e22;
        self.e23 = M.e23;
        self.e31 = M.e31;
        self.e32 = M.e32;
        self.e33 = M.e33;
    }

    ///Returns a copy of the transposed matrix.
    pub fn tranpose_copy(&self)  -> Matrix3x3 {
        let M: Matrix3x3 = Matrix3x3 {
            e11: self.e11,
            e12: self.e21,
            e13: self.e31,
            e21: self.e12,
            e22: self.e22,
            e23: self.e32,
            e31: self.e13,
            e32: self.e32,
            e33: self.e33,
        };

        return M;
    }

    ///Returns a copy of this matrix placed in a 4x4 matrix.
    pub fn as4x4(&self) -> Matrix4x4 {
        return Matrix4x4 {
            e11: self.e11,
            e12: self.e12,
            e13: self.e13,
            e14: 0.0,
            e21: self.e21,
            e22: self.e22,
            e23: self.e23,
            e24: 0.0,
            e31: self.e31,
            e32: self.e32,
            e33: self.e33,
            e34: 0.0,
            e41: 0.0,
            e42: 0.0,
            e43: 0.0,
            e44: 1.0,
        };
    }

}

impl Matrix4x4 {
    pub fn new() -> Matrix4x4 {
        return Matrix4x4 {
            e11: 1.0,
            e12: 0.0,
            e13: 0.0,
            e14: 0.0,
            e21: 0.0,
            e22: 1.0,
            e23: 0.0,
            e24: 0.0,
            e31: 0.0,
            e32: 0.0,
            e33: 1.0,
            e34: 0.0,
            e41: 0.0,
            e42: 0.0,
            e43: 0.0,
            e44: 1.0,
        };
    }

    ///Computes v' = Mv and returns v'.
    pub fn applyTo(&self, v: &mut (f64, f64, f64, f64)) {
        let (v1, v2, v3, v4) = (v.0, v.1, v.2, v.3);
        v.0 = self.e11*v1 + self.e12*v2 + self.e13*v3 + self.e14*v4;
        v.1 = self.e21*v1 + self.e22*v2 + self.e23*v3 + self.e24*v4;
        v.2 = self.e31*v1 + self.e32*v2 + self.e33*v3 + self.e34*v4;
        v.3 = self.e41*v1 + self.e42*v2 + self.e43*v3 + self.e44*v4;
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
    };
}

///4x4 Matrix multiplication. Computes M = A*B.
pub fn matrix_4x4_mult(A: &Matrix4x4, B: &Matrix4x4) -> Matrix4x4 {
    return Matrix4x4 {
        e11: A.e11*B.e11 + A.e12*B.e21 + A.e13*B.e31 + A.e14*B.e41,
        e12: A.e11*B.e12 + A.e12*B.e22 + A.e13*B.e32 + A.e14*B.e42,
        e13: A.e11*B.e13 + A.e12*B.e23 + A.e13*B.e33 + A.e14*B.e43,
        e14: A.e11*B.e14 + A.e12*B.e24 + A.e13*B.e34 + A.e14*B.e44,

        e21: A.e21*B.e11 + A.e22*B.e21 + A.e23*B.e31 + A.e24*B.e41,
        e22: A.e21*B.e12 + A.e22*B.e22 + A.e23*B.e32 + A.e24*B.e42,
        e23: A.e21*B.e13 + A.e22*B.e23 + A.e23*B.e33 + A.e24*B.e43,
        e24: A.e21*B.e14 + A.e22*B.e24 + A.e23*B.e34 + A.e24*B.e44,

        e31: A.e31*B.e11 + A.e32*B.e21 + A.e33*B.e31 + A.e34*B.e41,
        e32: A.e31*B.e12 + A.e32*B.e22 + A.e33*B.e32 + A.e34*B.e42,
        e33: A.e31*B.e13 + A.e32*B.e23 + A.e33*B.e33 + A.e34*B.e43,
        e34: A.e31*B.e14 + A.e32*B.e24 + A.e33*B.e34 + A.e34*B.e44,

        e41: A.e41*B.e11 + A.e42*B.e21 + A.e43*B.e31 + A.e44*B.e41,
        e42: A.e41*B.e12 + A.e42*B.e22 + A.e43*B.e32 + A.e44*B.e42,
        e43: A.e41*B.e13 + A.e42*B.e23 + A.e43*B.e33 + A.e44*B.e43,
        e44: A.e41*B.e14 + A.e42*B.e24 + A.e43*B.e34 + A.e44*B.e44,
    };
}