use web_sys::{WebGlRenderingContext};
use crate::engine::Engine;
use nalgebra::Matrix4;

pub struct Renderable {
    shader_name: String,
    color: [f32; 4], // R G B A 0 - 1,
    transform: [f32; 16],
}


impl Renderable {
    pub fn new(shader: String, transform: [f32; 16]) -> Renderable {
        Renderable {
            shader_name: shader,
            color: [0.0, 0.0, 0.0, 0.0],
            transform,
        }
    }

    // TODO:// remove dependency on engine crate
    // It doesn't look good to have the child depend on the parent
    pub fn draw(&self, engine: &Engine) {
        let program = engine.fetch_shader(&self.shader_name);
        engine.activate_shader(program);

        let transform_location = engine.gl_context.get_uniform_location(program, "modelTransform").unwrap();
        // Load the transformation in
        engine.gl_context.uniform_matrix4fv_with_f32_array(Some(&transform_location), true, &self.transform);

        engine.gl_context.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            3 as i32,
        )
    }
}