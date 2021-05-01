use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlBuffer};
use gl_matrix::{mat4};
use crate::engine::components::transform::*;

pub struct Cube {
    buffers: Buffers,
    pub transform: Transform,
}

impl Cube {
    pub fn new(context: &WebGlRenderingContext) -> Cube {
        let buffers = init_buffers(context).unwrap();

        Cube {
            buffers,
            transform: Transform::new(None, None, None),
        }
    }

    pub fn draw(&self, context: &WebGlRenderingContext, program: &WebGlProgram, offset: f64) -> f64 {
        let vertex_position = context.get_attrib_location(&program, "aVertexPosition");
        let vertex_color = context.get_attrib_location(&program, "aVertexColor");
        let model_view_matrix = context.get_uniform_location(&program, "uModelViewMatrix").unwrap();

        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.buffers.position));
        context.vertex_attrib_pointer_with_f64(
            vertex_position as u32,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0.0
        );

        context.enable_vertex_attrib_array(vertex_position as u32);

        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.buffers.color));
        context.vertex_attrib_pointer_with_f64(
            vertex_color as u32,
            4,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0.0
        );

        context.enable_vertex_attrib_array(vertex_color as u32);

        context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&self.buffers.indices));

        context.uniform_matrix4fv_with_f32_array(
            Some(&model_view_matrix),
            false,
            &self.transform.matrix
        );

        context.draw_elements_with_f64(
            WebGlRenderingContext::TRIANGLES,
            36,
            WebGlRenderingContext::UNSIGNED_SHORT,
            offset
        );

        36 as f64
    }
}

/// Cheaky cheat method until I get object loading in
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
