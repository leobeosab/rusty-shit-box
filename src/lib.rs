mod engine;

use console_error_panic_hook;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::engine::*;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

#[wasm_bindgen]
pub struct Application {
    engine: Engine,
}

#[wasm_bindgen]
impl Application {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Application {
        console_error_panic_hook::set_once();

        let engine = Engine::initialize_game_engine().expect("rip");
        engine.initialize_shaders( include_str!("./shaders/frag.fs"), include_str!("./shaders/vert.vs")).expect("oof");

        Application{
            engine,
        }
    }

    #[wasm_bindgen]
    pub fn render(&self) {
        let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
        self.engine.draw(&vertices);
    }
}

