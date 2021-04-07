pub(crate) mod shaders;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use std::error::Error;

pub struct Engine {
    pub(crate) gl_context: WebGlRenderingContext,
    active: bool,
}

pub fn initialize_game_engine() -> Result<Engine, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    if (true) {
        Ok(Engine {
            gl_context: context,
            active: true
        })
    } else {
        Err(JsValue::from("fuck me".to_string()))
    }
}