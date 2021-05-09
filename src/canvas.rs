use web_sys::{WebGlRenderingContext, HtmlCanvasElement};
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;

pub fn create_canvas() -> Result<HtmlCanvasElement, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
    Ok(canvas)
}

pub fn retrieve_context_from_canvas(canvas: &HtmlCanvasElement) -> Result<WebGlRenderingContext, JsValue> {
    Ok(canvas
        .get_context("webgl").unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>().unwrap())
}