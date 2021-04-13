extern crate web_sys;

use console_error_panic_hook;

use wasm_bindgen::prelude::*;
use crate::engine::*;
use crate::renderable::Renderable;
use nalgebra::{Matrix4, Isometry3, Vector3};

mod engine;
mod renderable;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Application {
    engine: Engine,
    renderables: Vec<Renderable>,
}

#[wasm_bindgen]
impl Application {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Application {
        console_error_panic_hook::set_once();

        let mut engine = Engine::initialize_game_engine().expect("rip");
        log!("Created engine and WebGL context");
        // TODO:// put the error handling back
        engine.initialize_shaders("simple_shader", include_str!("./shaders/frag.fs"), include_str!("./shaders/vert.vs"));
        log!("initialized simple_shader");

        let renderables: Vec<Renderable> = Vec::new();

        let mut app = Application{
            engine,
            renderables
        };

        // Just getting this working -- gross
        let transform = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            1.0, 1.0, 1.0, 1.0,
            0.0, -0.1, 0.0, 1.0
        );

        let scale = Matrix4::new_nonuniform_scaling(&Vector3::new(1.0, 1.0, 1.0));

        let model = Isometry3::new(Vector3::new(0.5, 0.5, 1.0), nalgebra::zero());
        let model = model.to_homogeneous();
        let model = scale * model;

        let mut model_array: [f32; 16] = [0.; 16];
        model_array.copy_from_slice(transform.as_slice());

        let dump_triangle = Renderable::new(String::from("simple_shader"), model_array);
        app.renderables.push(dump_triangle);

        app
    }

    #[wasm_bindgen]
    pub fn render(&self) {
        // let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
        let vertices = [
            // Front face
            -1.0, -1.0,  1.0,
            1.0, -1.0,  1.0,
            1.0,  1.0,  1.0,
            -1.0,  1.0,  1.0,

            // Back face
            -1.0, -1.0, -1.0,
            -1.0,  1.0, -1.0,
            1.0,  1.0, -1.0,
            1.0, -1.0, -1.0,

            // Top face
            -1.0,  1.0, -1.0,
            -1.0,  1.0,  1.0,
            1.0,  1.0,  1.0,
            1.0,  1.0, -1.0,

            // Bottom face
            -1.0, -1.0, -1.0,
            1.0, -1.0, -1.0,
            1.0, -1.0,  1.0,
            -1.0, -1.0,  1.0,

            // Right face
            1.0, -1.0, -1.0,
            1.0,  1.0, -1.0,
            1.0,  1.0,  1.0,
            1.0, -1.0,  1.0,

            // Left face
            -1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
            -1.0,  1.0,  1.0,
            -1.0,  1.0, -1.0
        ];

        let elements = [
            0,  1,  2,      0,  2,  3,    // front
            4,  5,  6,      4,  6,  7,    // back
            8,  9,  10,     8,  10, 11,   // top
            12, 13, 14,     12, 14, 15,   // bottom
            16, 17, 18,     16, 18, 19,   // right
            20, 21, 22,     20, 22, 23    // left
        ];

        self.engine.draw(&vertices, &elements);

        for obj in self.renderables.iter() {
            obj.draw(&self.engine);
        }
    }
}

