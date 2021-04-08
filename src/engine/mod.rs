pub(crate) mod shaders;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use std::error::Error;

pub struct Engine {
    pub(crate) gl_context: WebGlRenderingContext,
    active: bool,
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
                active: true
            })
        } else {
            Err(JsValue::from("fuck me".to_string()))
        }
    }

    pub(crate) fn initialize_shaders(&self, frag_source: &str, vert_source: &str) -> Result<JsValue, String> {
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
        self.gl_context.use_program(Some(&program));

        Ok(self.gl_context.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS))
    }
}

