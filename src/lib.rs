mod engine;
mod renderable;

use console_error_panic_hook;

use wasm_bindgen::prelude::*;
use crate::engine::*;
use crate::renderable::Renderable;

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
        // TODO:// put the error handling back
        engine.initialize_shaders("simple_shader", include_str!("./shaders/frag.fs"), include_str!("./shaders/vert.vs"));

        let renderables: Vec<Renderable> = Vec::new();

        let mut app = Application{
            engine,
            renderables
        };

        // Just getting this working -- gross
        let dump_triangle = Renderable::new(String::from("simple_shader"));
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

