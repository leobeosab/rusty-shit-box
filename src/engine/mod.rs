pub(crate) mod shaders;
mod camera;
mod cube;
mod components;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlBuffer};
use std::collections::HashMap;
use gl_matrix::mat4;
use crate::engine::camera::Camera;
use crate::engine::cube::Cube;

pub struct Engine {
    pub(crate) gl_context: WebGlRenderingContext,
    camera: Camera,
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

        context.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

        let camera = Camera::new(1.0, 0.1, 100.0, 45.0);

        if true {
            Ok(Engine {
                gl_context: context,
                camera,
                shaders: HashMap::new(),
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
        self.gl_context.use_program(Some(program));
    }

    pub(crate) fn draw(&self, rotation: f32) -> Result<JsValue, String> {
        self.gl_context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl_context.clear_depth(1.0);
        self.gl_context.enable(WebGlRenderingContext::DEPTH_TEST);
        self.gl_context.depth_func(WebGlRenderingContext::LEQUAL);
        self.gl_context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

        let program = self.fetch_shader("simple_shader");
        self.gl_context.use_program(Some(program));

        let projection_matrix = self.gl_context.get_uniform_location(&program, "uProjectionMatrix").unwrap();

        // Setup the camera matrix
        self.gl_context.uniform_matrix4fv_with_f32_array(
            Some(&projection_matrix),
            false,
            &self.camera.projection_matrix,
        );

        let mut cube = Cube::new(&self.gl_context);

        cube.transform.translate(2.0, 0.0, -6.0);
        cube.transform.rotate(rotation, rotation, rotation * 0.8);

        let mut cube2 = Cube::new(&self.gl_context);
        cube2.transform.translate(-2.0, 0.0, -6.0);
        cube2.transform.rotate(rotation, 0.0, rotation * 0.2);
        cube2.transform.scale(2.0, 2.0, 2.0 );

        let mut offset = cube.draw(&self.gl_context, program, 0.0);
        offset += cube2.draw(&self.gl_context, program, 0.0);

        Ok(JsValue::from("Success!"))
    }
}


