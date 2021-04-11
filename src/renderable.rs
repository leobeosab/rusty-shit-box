use wasm_bindgen::{JsCast, JsValue};
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use std::error::Error;
use crate::engine::Engine;

pub struct Renderable {
    shader_name: String,
    color: [f32; 4] // R G B A 0 - 1
}

impl Renderable {
    pub fn new(shader: String) -> Renderable {
        Renderable {
            shader_name: shader,
            color: [0.0, 0.0, 0.0, 0.0]
        }
    }

    // TODO:// remove dependency on engine crate
    // It doesn't look good to have the child depend on the parent
    pub fn draw(&self, engine: &Engine) {
        let program = engine.fetch_shader(&self.shader_name);
        engine.activate_shader(program);
        engine.gl_context.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            3 as i32,
        )
    }
}