//! Transform for matrix calculations
//!
//! Provides a slight abstraction over gl_matrix to make it easier to do
//! rotations, scale and translations
//!
//! This will be a part of the components structure

use gl_matrix::mat4;

pub struct Transform {
    pub matrix: [f32; 16]
}

impl Transform {
    /// Create a matrix
    /// This will return a blank matrix if you choose not to pass in position rotation and scale
    pub fn new(position: Option<[f32; 3]>, rotation: Option<[f32; 3]>, scale_factor: Option<[f32; 3]>) -> Transform {
        let mut position_matrix = mat4::create();

        /// Is this how we do optionals in Rust
        /// Beats the fuck out of me
        match position {
            Some(x) => {
                translate(&mut position_matrix, x[0], x[1], x[2]);
            },
            _ => (),
        }

        match rotation {
            Some(x) => {
                rotate(&mut position_matrix, x[0], x[1], x[2]);
            },
            _ => (),
        }

        match scale_factor {
            Some(x) => {
                scale(&mut position_matrix, x[0], x[1], x[2]);
            },
            _ => (),
        }


        Transform {
            matrix: mat4::create(),
        }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        translate(&mut self.matrix, x, y, z);
    }

    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        rotate(&mut self.matrix, x, y, z);
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        scale(&mut self.matrix, x, y, z);
    }
}

fn translate(mut out: &mut [f32; 16], x: f32, y: f32, z: f32) {
    let mut clone = out.clone();

    mat4::translate(
        &mut out,
        &mut clone,
        &[x, y, z],
    );
}


fn scale(mut out: &mut [f32; 16], x: f32, y: f32, z: f32) {
    let mut clone = out.clone();

    mat4::scale(
        &mut out,
        &mut clone,
        &[x, y, z],
    );
}

pub fn rotate(mut out: &mut [f32; 16], x: f32, y: f32, z: f32) {
    let mut clone = out.clone();

    mat4::rotate(
        &mut out,
        &mut clone,
        x,
        &[1.0, 0.0, 0.0]
    );

    clone = out.clone();

    mat4::rotate(
        &mut out,
        &mut clone,
        y,
        &[0.0, 1.0, 0.0]
    );

    clone = out.clone();

    mat4::rotate(
        &mut out,
        &mut clone,
        z,
        &[0.0, 0.0, 1.0]
    );
}