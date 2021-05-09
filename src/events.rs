use wasm_bindgen::prelude::*;
use std::cell::{RefCell, RefMut};
use crate::{Scene, Msg, log};
use web_sys::WebGlRenderingContext;
use wasm_bindgen::JsCast;
use std::rc::Rc;

pub fn attach_event_handlers(scene: Rc<RefCell<Scene>>) {
    attach_window_resized_handler(scene);
}

fn attach_window_resized_handler(scene: Rc<RefCell<Scene>>) {
    log!("resize handler initialized");
    let resize_function = move |_evt: web_sys::EventTarget| {
        let canvas = web_sys::window().unwrap().document().unwrap().get_element_by_id("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let context = canvas.get_context("webgl").unwrap().unwrap().dyn_into::<WebGlRenderingContext>().unwrap();
        context.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

        scene.borrow_mut().msg(Msg::WindowResized(canvas.width() as f32, canvas.height() as f32));

        log!("Resize!");
    };

    let handler = Closure::wrap(Box::new(resize_function) as Box<dyn Fn(_)>);

    web_sys::window().unwrap().set_onresize(Some(handler.as_ref().unchecked_ref()));

    handler.forget();
}