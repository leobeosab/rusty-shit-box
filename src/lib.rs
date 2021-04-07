mod engine;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::engine::*;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let ENGINE: Engine = initialize_game_engine().expect("Rip");

    let vert_shader = shaders::compile_shader(
        &(ENGINE.gl_context),
        WebGlRenderingContext::VERTEX_SHADER,
        include_str!("shaders/vert.vs"),

    )?;
    let frag_shader = shaders::compile_shader(
        &(ENGINE.gl_context),
        WebGlRenderingContext::FRAGMENT_SHADER,
        include_str!("shaders/frag.fs"),
    )?;
    let program = shaders::link_program(&(ENGINE.gl_context), &vert_shader, &frag_shader)?;
    (ENGINE.gl_context).use_program(Some(&program));

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let buffer = (ENGINE.gl_context).create_buffer().ok_or("failed to create buffer")?;
    (ENGINE.gl_context).bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);

        (ENGINE.gl_context).buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    (ENGINE.gl_context).vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    (ENGINE.gl_context).enable_vertex_attrib_array(0);

    (ENGINE.gl_context).clear_color(0.0, 1.0, 0.0, 1.0);
    (ENGINE.gl_context).clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    (ENGINE.gl_context).draw_arrays(
        WebGlRenderingContext::TRIANGLES,
        0,
        (vertices.len() / 3) as i32,
    );
    Ok(())
}