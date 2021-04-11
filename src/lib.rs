mod engine;

use console_error_panic_hook;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::engine::*;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use std::collections::HashMap;

#[wasm_bindgen]
pub struct Application {
    engine: Engine,
    shaders: HashMap<String, WebGlProgram>,
}

#[wasm_bindgen]
impl Application {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Application {
        console_error_panic_hook::set_once();

        let engine = Engine::initialize_game_engine().expect("rip");
        let shader_program = engine.initialize_shaders( include_str!("./shaders/frag.fs"), include_str!("./shaders/vert.vs")).unwrap();

        let mut shader_map: HashMap<String, WebGlProgram> = HashMap::new();
        shader_map.insert(String::from("simpleShader"), shader_program);

        let app = Application{
            engine,
            shaders: shader_map
        };

        // Gross? I need to know more rust
        app.engine.activate_shader(app.fetch_shader("simpleShader"));

        app
    }

    #[wasm_bindgen]
    pub fn render(&self) {
        let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
        self.engine.draw(&vertices);
    }

    fn fetch_shader(&self, key: &str) -> &WebGlProgram {
        let shader;

        match self.shaders.get(key) {
            Some(s) => shader = s,
            None => panic!("Could not find shader {}", key)
        }

        shader
    }
}

