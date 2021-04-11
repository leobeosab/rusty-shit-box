pub(crate) mod shaders;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use std::error::Error;
use std::collections::HashMap;

pub struct Engine {
    pub(crate) gl_context: WebGlRenderingContext,
    shaders: HashMap<String, WebGlProgram>,
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
                shaders: HashMap::new()
            })
        } else {
            Err(JsValue::from("fuck me".to_string()))
        }
    }

    pub(crate) fn initialize_shaders(&mut self, shader_name: &str, frag_source: &str, vert_source: &str) {
        let vert_shader = shaders::compile_shader(
            &self.gl_context,
            WebGlRenderingContext::VERTEX_SHADER,
            vert_source
        ).unwrap();
        let frag_shader = shaders::compile_shader(
            &self.gl_context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            frag_source
        ).unwrap();

        let program = shaders::link_program(&self.gl_context, &vert_shader, &frag_shader).unwrap();

        self.shaders.insert(String::from(shader_name), program);
    }

    pub(crate) fn fetch_shader(&self, shader_name: &str) -> &WebGlProgram {
        let shader;

        match self.shaders.get(shader_name) {
            Some(s) => shader = s,
            None => panic!("Could not find shader {}", shader_name)
        }

        shader
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

        Ok(JsValue::from("Success!"))
    }
}

