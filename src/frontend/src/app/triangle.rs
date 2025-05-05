use crate::libs::rendering::canvas::*;

use leptos::html::Canvas;
use leptos::logging;
use leptos::prelude::*;
use leptos_use::{
    use_raf_fn, use_resize_observer, use_window_size, UseRafFnCallbackArgs, UseWindowSizeReturn,
};

use wasm_bindgen::JsCast;
use web_sys::{ResizeObserverEntry, WebGl2RenderingContext, WebGlProgram, WebGlShader};

use std::fmt;
use std::rc::Rc;

const VERTEX_SHADER_SOURCE: &str = r#"
attribute vec4 position;
void main() {
    gl_Position = position;
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
precision mediump float;
void main() {
    gl_FragColor = vec4(0.0, 1.0, 0.0, 1.0);
}
"#;

fn triangle_init(web_gl_canvas: &WebGlCanvas) -> bool {
    let mut gl_opt = web_gl_canvas.get_context_mut();
    let canvas = web_gl_canvas
        .get_canvas_mut()
        .as_mut()
        .unwrap()
        .get()
        .unwrap();
    if let Some(gl) = gl_opt.as_mut() {
        // do stuff
        canvas.set_width(1000);
        canvas.set_height(1000);
        gl.viewport(0, 0, 1000, 1000);
        let vert_s = match compile_shader(
            &gl,
            WebGl2RenderingContext::VERTEX_SHADER,
            VERTEX_SHADER_SOURCE,
        ) {
            Ok(vert_s) => vert_s,
            Err(error) => {
                logging::error!(
                    "Vertex shader compilation failed in triangle_init. Error: {}",
                    error
                );
                return false;
            }
        };
        let frag_s = match compile_shader(
            &gl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            FRAGMENT_SHADER_SOURCE,
        ) {
            Ok(vert_s) => vert_s,
            Err(error) => {
                logging::error!(
                    "Fragment shader compilation failed in triangle_init. Error: {}",
                    error
                );
                return false;
            }
        };
        let program = match link_program(&gl, &frag_s, &vert_s) {
            Ok(program) => program,
            Err(error) => {
                logging::error!("Program linking failed in triangle_init. Error: {}", error);
                return false;
            }
        };
        gl.use_program(Some(&program));

        let buffer = if let Some(buffer) = gl.create_buffer() {
            buffer
        } else {
            logging::error!(
                "Could not create buffer in triangle_init, using {:?}",
                web_gl_canvas
            );
            return false;
        };
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
        let vertices: [f32; 6] = [0.0, 0.5, -0.5, -0.5, 0.5, -0.5];
        unsafe {
            let vertices_array = js_sys::Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vertices_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        let position = gl.get_attrib_location(&program, "position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);
        true
    } else {
        logging::error!(
            "GL context is None in triangle_init, using {:?}",
            web_gl_canvas
        );
        false
    }
}

#[component]
pub fn Triangle2() -> impl IntoView {
    let web_gl_canvas = WebGlCanvas::new("Triangle");
    let init_task = InitTask::new(triangle_init, "Triangle init");
    web_gl_canvas.add_init_task(init_task);
    view! { <WebGlCanvasComponent web_gl_canvas=web_gl_canvas /> }
}

#[component]
pub fn Triangle() -> impl IntoView {
    let canvas_ref = NodeRef::<Canvas>::new();
    let UseWindowSizeReturn { width, height } = use_window_size();

    leptos::logging::log!("triangle func called");
    Effect::new(move |_| {
        leptos::logging::log!("event called");
        if let Some(canvas) = canvas_ref.get() {
            canvas.set_width(width.get() as u32);
            canvas.set_height(height.get() as u32);
            let canvas_element = &canvas;
            let gl = canvas_element
                .get_context("webgl2")
                .expect("error when getting webgl2 context")
                .expect("failed to get webgl2 context")
                .dyn_into::<WebGl2RenderingContext>()
                .expect("failed to cast canvas into WebGl2RenderingContext");
            let vertex_shader = compile_shader(
                &gl,
                WebGl2RenderingContext::VERTEX_SHADER,
                VERTEX_SHADER_SOURCE,
            )
            .expect("vertex shader does not compile");
            let fragment_shader = compile_shader(
                &gl,
                WebGl2RenderingContext::FRAGMENT_SHADER,
                FRAGMENT_SHADER_SOURCE,
            )
            .expect("fragment shader does not compile");
            let program = link_program(&gl, &vertex_shader, &fragment_shader)
                .expect("gl program link failed");
            gl.use_program(Some(&program));
            let vertices: [f32; 6] = [0.0, 0.5, -0.5, -0.5, 0.5, -0.5];
            let buffer = gl
                .create_buffer()
                .ok_or("failed to create buffer")
                .expect("error when creating buffer");
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
            unsafe {
                let vertices_array = js_sys::Float32Array::view(&vertices);
                gl.buffer_data_with_array_buffer_view(
                    WebGl2RenderingContext::ARRAY_BUFFER,
                    &vertices_array,
                    WebGl2RenderingContext::STATIC_DRAW,
                );
            }
            let position = gl.get_attrib_location(&program, "position") as u32;
            gl.vertex_attrib_pointer_with_i32(
                position,
                2,
                WebGl2RenderingContext::FLOAT,
                false,
                0,
                0,
            );
            gl.enable_vertex_attrib_array(position);
            // Clear the canvas and draw the triangle
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
            gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);
            // window_event_listener(leptos::ev::resize, move |_event| {
            //     leptos::logging::log!("window resized");
            //     // let UseWindowSizeReturn { width, height } = use_window_size();
            //     canvas.set_width(width.get() as u32);
            //     canvas.set_height(height.get() as u32);
            //     gl.clear_color(0.0, 0.0, 0.0, 1.0);
            //     gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
            //     gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);
            // });
        }
    });
    view! { <canvas node_ref=canvas_ref /> }
}

fn compile_shader(
    gl: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or("Unable to create shader object")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    if gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(&shader).unwrap_or_default())
    }
}
fn link_program(
    gl: &WebGl2RenderingContext,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or("Unable to create shader program")?;
    gl.attach_shader(&program, vertex_shader);
    gl.attach_shader(&program, fragment_shader);
    gl.link_program(&program);
    if gl
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program).unwrap_or_default())
    }
}
