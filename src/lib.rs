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

        let mut engine: Engine = Engine::initialize_game_engine();
        log!("Created engine and WebGL context");
        engine.initialize_shaders("simple_shader", include_str!("./shaders/frag.fs"), include_str!("./shaders/vert.vs"));
        log!("initialized simple_shader");

        let renderables: Vec<Renderable> = Vec::new();

        let app = Application{
            engine,
            renderables
        };

        app
    }

    #[wasm_bindgen]
    pub fn render(&self, rotation: f32) {
        self.engine.draw(rotation);
    }
}

