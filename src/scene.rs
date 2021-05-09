use crate::components::camera::Camera;
use crate::renderable::Renderable;

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
