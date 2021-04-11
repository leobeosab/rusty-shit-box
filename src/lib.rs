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
            1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0
        );

        let scale = Matrix4::new_nonuniform_scaling(&Vector3::new(1.0, 1.0, 1.0));

        let model = Isometry3::new(Vector3::new(0.5, 0.5, 1.0), nalgebra::zero());
        let model = model.to_homogeneous();
        let model = scale * model;

        let mut model_array: [f32; 16] = [0.; 16];
        model_array.copy_from_slice(model.as_slice());

        let dump_triangle = Renderable::new(String::from("simple_shader"), model_array);
        app.renderables.push(dump_triangle);

        app
    }

    #[wasm_bindgen]
    pub fn render(&self) {
        let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
        self.engine.draw(&vertices);

        for obj in self.renderables.iter() {
            obj.draw(&self.engine);
        }
    }
}

