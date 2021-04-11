pub(crate) mod shaders;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use std::error::Error;

pub struct Engine {
    pub(crate) gl_context: WebGlRenderingContext,
}

impl Engine {
     pub(crate) fn initialize_game_engine() -> Result<Engine, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

        let context = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()?;

        if true {
            Ok(Engine {
                gl_context: context,
            })
        } else {
            Err(JsValue::from("fuck me".to_string()))
        }
    }

    pub(crate) fn initialize_shaders(&self, frag_source: &str, vert_source: &str) -> Result<WebGlProgram, String> {
        let vert_shader = shaders::compile_shader(
            &self.gl_context,
            WebGlRenderingContext::VERTEX_SHADER,
            vert_source
        )?;
        let frag_shader = shaders::compile_shader(
            &self.gl_context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            frag_source
        )?;

        let program = shaders::link_program(&self.gl_context, &vert_shader, &frag_shader)?;

        Ok(program)
    }

    pub(crate) fn activate_shader(&self, program: &WebGlProgram) {
        self.gl_context.use_program(Some(&program));
        self.gl_context.enable_vertex_attrib_array(0);
    }

    pub(crate) fn draw(&self, vertex_array: &[f32]) -> Result<JsValue, String> {
        let buffer = self.gl_context.create_buffer().ok_or("failed to create buffer")?;
        self.gl_context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let vert_array = js_sys::Float32Array::view(vertex_array);

            (self.gl_context).buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        self.gl_context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);

        self.gl_context.clear_color(0.0, 1.0, 0.0, 1.0);
        self.gl_context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.gl_context.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            (vertex_array.len() / 3) as i32,
        );

        Ok(JsValue::from("Success!"))
    }
}

