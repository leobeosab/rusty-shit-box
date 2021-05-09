extern crate web_sys;

use console_error_panic_hook;
use wasm_bindgen::prelude::*;

use crate::engine::*;
use crate::renderable::Renderable;
use crate::components::camera::Camera;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

mod engine;
mod renderable;
mod utils;
mod components;
mod canvas;
mod events;

pub struct Scene {
    scene_camera: Camera,
    renderables: Vec<Renderable>,
}

impl Scene {
    pub fn new() -> Self {
        let camera = Camera::default();
        let renderables: Vec<Renderable> = Vec::new();
        Self { scene_camera: camera, renderables }
    }

    pub fn camera(&self) -> &Camera {
        &self.scene_camera
    }

    pub fn renderables(&self) -> &Vec<Renderable> {
        &self.renderables
    }

    pub fn msg(&mut self, msg: Msg) {
        match msg {
            Msg::WindowResized(width, height) => self.scene_camera.update_aspect_ratio(width / height),
        }
    }
}

pub enum Msg {
    WindowResized(f32, f32),
}

#[wasm_bindgen]
pub struct Application {
    engine: Engine,
    current_scene: Rc<RefCell<Scene>>,
}

#[wasm_bindgen]
impl Application {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Application {
        console_error_panic_hook::set_once();

        let canvas = canvas::create_canvas().unwrap();
        let context = canvas::retrieve_context_from_canvas(&canvas).unwrap();

        let scene = RefCell::new(Scene::new());

        scene.borrow_mut().msg(Msg::WindowResized(canvas.width() as f32, canvas.height() as f32));

        let mut app = Application{
            engine: Engine::new(context),
            current_scene: Rc::new(scene),
        };

        events::attach_event_handlers(Rc::clone(&app.current_scene));
        app.engine.initialize_shaders("simple_shader", include_str!("./shaders/frag.fs"), include_str!("./shaders/vert.vs"));

        app
    }

    #[wasm_bindgen]
    pub fn render(&self, rotation: f32) {
        self.engine.draw(rotation, self.current_scene.borrow());
    }
}

