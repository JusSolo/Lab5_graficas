//matrix.rs
#![allow(dead_code)]

use raylib::prelude::Matrix;

/// Creates a 4x4 matrix from 16 float values, specified in traditional row-major order.
pub fn new_matrix4(
    // Row 0
    r0c0: f32,
    r0c1: f32,
    r0c2: f32,
    r0c3: f32,
    // Row 1
    r1c0: f32,
    r1c1: f32,
    r1c2: f32,
    r1c3: f32,
    // Row 2
    r2c0: f32,
    r2c1: f32,
    r2c2: f32,
    r2c3: f32,
    // Row 3
    r3c0: f32,
    r3c1: f32,
    r3c2: f32,
    r3c3: f32,
) -> Matrix {
    // Raylib's Matrix is column-major, so we transpose the row-major input.
    Matrix {
        m0: r0c0,
        m1: r1c0,
        m2: r2c0,
        m3: r3c0, // Column 0
        m4: r0c1,
        m5: r1c1,
        m6: r2c1,
        m7: r3c1, // Column 1
        m8: r0c2,
        m9: r1c2,
        m10: r2c2,
        m11: r3c2, // Column 2
        m12: r0c3,
        m13: r1c3,
        m14: r2c3,
        m15: r3c3, // Column 3
    }
}

/// Creates a 4x4 transformation matrix from a 3x3 matrix, specified in row-major order.
pub fn new_matrix3(
    // Row 0
    r0c0: f32,
    r0c1: f32,
    r0c2: f32,
    // Row 1
    r1c0: f32,
    r1c1: f32,
    r1c2: f32,
    // Row 2
    r2c0: f32,
    r2c1: f32,
    r2c2: f32,
) -> Matrix {
    new_matrix4(
        r0c0, r0c1, r0c2, 0.0, r1c0, r1c1, r1c2, 0.0, r2c0, r2c1, r2c2, 0.0, 0.0, 0.0, 0.0, 1.0,
    )
}
