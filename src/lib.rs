extern crate web_sys;

use console_error_panic_hook;

use wasm_bindgen::prelude::*;
use crate::engine::*;
use crate::renderable::Renderable;

mod engine;
mod renderable;
mod utils;

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

        let mut model_array: [f32; 16] = [0.; 16];

        let dump_triangle = Renderable::new(String::from("simple_shader"), model_array);

        app
    }

    #[wasm_bindgen]
    pub fn render(&self, rotation: f32) {
        self.engine.draw(rotation);
    }
}

