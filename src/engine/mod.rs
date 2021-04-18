pub(crate) mod shaders;

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlBuffer};
use std::collections::HashMap;
use gl_matrix::mat4;
use gl_matrix::mat3::projection;

pub struct Engine {
    pub(crate) gl_context: WebGlRenderingContext,
    shaders: HashMap<String, WebGlProgram>,
}


#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
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

        let field_of_view = 45.0 * std::f64::consts::PI / 180.0; // in radians
        let aspect =  800 / 800;
        let z_near = 0.1;
        let z_far = 100.0;
        let mut internal_projection_matrix = mat4::create();

        mat4::perspective(
            &mut internal_projection_matrix,
            field_of_view as f32,
            aspect as f32,
            z_near,
            Some(z_far)
        );

        let mut internal_model_view_matrix = mat4::create();
        let mut internal_model_view_matrix_2 = internal_model_view_matrix.clone();

        mat4::translate(
            &mut internal_model_view_matrix,
            &mut internal_model_view_matrix_2,
            &[-0.0, 0.0, -6.0]
        );

        internal_model_view_matrix_2 = internal_model_view_matrix.clone();

        mat4::rotate(
            &mut internal_model_view_matrix,
            &mut internal_model_view_matrix_2,
            rotation,
            &[0.0, 0.0, 1.0]
        );

        // We need to copy after _every_ translation/rotation -_-
        internal_model_view_matrix_2 = internal_model_view_matrix.clone();

        mat4::rotate(
            &mut internal_model_view_matrix,
            &mut internal_model_view_matrix_2,
            rotation * 0.7,
            &[0.0, 1.0, 0.0]
        );

        let vertex_position = self.gl_context.get_attrib_location(&program, "aVertexPosition");
        let projection_matrix = self.gl_context.get_uniform_location(&program, "uProjectionMatrix").unwrap();
        let model_view_matrix = self.gl_context.get_uniform_location(&program, "uModelViewMatrix").unwrap();

        let buffers = init_buffers(&self.gl_context)?;

        self.gl_context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffers.position));
        self.gl_context.vertex_attrib_pointer_with_f64(
            vertex_position as u32,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0.0
        );

        self.gl_context.enable_vertex_attrib_array(vertex_position as u32);

        self.gl_context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffers.indices));
        self.activate_shader(program);

        self.gl_context.uniform_matrix4fv_with_f32_array(
            Some(&projection_matrix),
            false,
            &internal_projection_matrix
        );

        self.gl_context.uniform_matrix4fv_with_f32_array(
            Some(&model_view_matrix),
            false,
            &internal_model_view_matrix
        );

        self.gl_context.draw_elements_with_f64(
            WebGlRenderingContext::TRIANGLES,
            36,
            WebGlRenderingContext::UNSIGNED_SHORT,
            0.0
        );

        log(format!("model_view_matrix: {:?}", internal_model_view_matrix).as_str());

        Ok(JsValue::from("Success!"))
    }
}


fn init_buffers(context: &WebGlRenderingContext) -> Result<Buffers, String> {
    // Setup the positions buffer

    let position_buffer = context.create_buffer().ok_or("failed to create position buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&position_buffer));

    let positions: [f32; 72] = [
        // Front face
        -1.0, -1.0,  1.0,
        1.0, -1.0,  1.0,
        1.0,  1.0,  1.0,
        -1.0,  1.0,  1.0,

        // Back face
        -1.0, -1.0, -1.0,
        -1.0,  1.0, -1.0,
        1.0,  1.0, -1.0,
        1.0, -1.0, -1.0,

        // Top face
        -1.0,  1.0, -1.0,
        -1.0,  1.0,  1.0,
        1.0,  1.0,  1.0,
        1.0,  1.0, -1.0,

        // Bottom face
        -1.0, -1.0, -1.0,
        1.0, -1.0, -1.0,
        1.0, -1.0,  1.0,
        -1.0, -1.0,  1.0,

        // Right face
        1.0, -1.0, -1.0,
        1.0,  1.0, -1.0,
        1.0,  1.0,  1.0,
        1.0, -1.0,  1.0,

        // Left face
        -1.0, -1.0, -1.0,
        -1.0, -1.0,  1.0,
        -1.0,  1.0,  1.0,
        -1.0,  1.0, -1.0,
    ];

    // More info on why this is unsafe
    // https://docs.rs/js-sys/0.3.45/js_sys/struct.Float32Array.html#unsafety
    unsafe {
        let position_array = js_sys::Float32Array::view(&positions);

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &position_array,
            WebGlRenderingContext::STATIC_DRAW
        )
    }

    // Setup the color buffer

    let colors: [f32; 96] = [
        1.0,  1.0,  1.0,  1.0,    // Front face: white
        1.0,  1.0,  1.0,  1.0,    // Front face: white
        1.0,  1.0,  1.0,  1.0,    // Front face: white
        1.0,  1.0,  1.0,  1.0,    // Front face: white
        1.0,  0.0,  0.0,  1.0,    // Back face: red
        1.0,  0.0,  0.0,  1.0,    // Back face: red
        1.0,  0.0,  0.0,  1.0,    // Back face: red
        1.0,  0.0,  0.0,  1.0,    // Back face: red
        0.0,  1.0,  0.0,  1.0,    // Top face: green
        0.0,  1.0,  0.0,  1.0,    // Top face: green
        0.0,  1.0,  0.0,  1.0,    // Top face: green
        0.0,  1.0,  0.0,  1.0,    // Top face: green
        0.0,  0.0,  1.0,  1.0,    // Bottom face: blue
        0.0,  0.0,  1.0,  1.0,    // Bottom face: blue
        0.0,  0.0,  1.0,  1.0,    // Bottom face: blue
        0.0,  0.0,  1.0,  1.0,    // Bottom face: blue
        1.0,  1.0,  0.0,  1.0,    // Right face: yellow
        1.0,  1.0,  0.0,  1.0,    // Right face: yellow
        1.0,  1.0,  0.0,  1.0,    // Right face: yellow
        1.0,  1.0,  0.0,  1.0,    // Right face: yellow
        1.0,  0.0,  1.0,  1.0,    // Left face: purple
        1.0,  0.0,  1.0,  1.0,    // Left face: purple
        1.0,  0.0,  1.0,  1.0,    // Left face: purple
        1.0,  0.0,  1.0,  1.0,    // Left face: purple
    ];


    let color_buffer = context.create_buffer().ok_or("failed to create color buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&color_buffer));

    // More info on why this is unsafe
    // https://docs.rs/js-sys/0.3.45/js_sys/struct.Float32Array.html#unsafety
    unsafe {
        let color_array = js_sys::Float32Array::view(&colors);

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &color_array,
            WebGlRenderingContext::STATIC_DRAW
        )
    }

    let index_buffer = context.create_buffer().ok_or("failed to create index buffer")?;
    context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));

    let indices = [
        0,  1,  2,      0,  2,  3,    // front
        4,  5,  6,      4,  6,  7,    // back
        8,  9,  10,     8,  10, 11,   // top
        12, 13, 14,     12, 14, 15,   // bottom
        16, 17, 18,     16, 18, 19,   // right
        20, 21, 22,     20, 22, 23,   // left
    ];

    // More info on why this is unsafe
    // https://docs.rs/js-sys/0.3.45/js_sys/struct.Float32Array.html#unsafety
    unsafe {
        let index_array = js_sys::Uint16Array::view(&indices);

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            &index_array,
            WebGlRenderingContext::STATIC_DRAW
        )
    }

    Ok(Buffers {
        position: position_buffer,
        color: color_buffer,
        indices: index_buffer
    })
}

struct Buffers {
    position: WebGlBuffer,
    color: WebGlBuffer,
    indices: WebGlBuffer,
}
