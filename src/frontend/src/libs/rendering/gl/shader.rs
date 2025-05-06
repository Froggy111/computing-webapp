use leptos::logging;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

use crate::libs::{rendering::canvas::WebGlCanvas, types::errors::ErrorStr};

#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    VertexShader,
    FragmentShader,
}

#[derive(Debug)]
pub struct Shader<'a> {
    canvas: &'a WebGlCanvas<'a>,
    shader: Option<WebGlShader>,
    shader_type: ShaderType,
    program_source: &'a str,
    name: &'a str,
}

impl<'a> Shader<'a> {
    pub fn new(
        canvas: &'a WebGlCanvas<'a>,
        shader_type: ShaderType,
        program_source: &'a str,
        name: &'a str,
    ) -> Self {
        let shader = compile_shader(canvas, shader_type, program_source, name);
        let shader = match shader {
            Ok(shader) => shader,
            Err(error) => {
                logging::error!("{}", error);
                return Self {
                    program_source,
                    name,
                    canvas,
                    shader: None,
                    shader_type,
                };
            }
        };
        Self {
            program_source,
            name,

            canvas,
            shader: Some(shader),
            shader_type,
        }
    }

    pub fn shader(&self) -> Option<&WebGlShader> {
        self.shader.as_ref()
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn shader_type(&self) -> ShaderType {
        self.shader_type
    }
}

fn compile_shader<'a>(
    canvas: &'a WebGlCanvas<'a>,
    shader_type: ShaderType,
    program_source: &'a str,
    name: &'a str,
) -> Result<WebGlShader, ErrorStr> {
    let context = canvas.get_context();
    let gl = if let Some(gl) = context.as_ref() {
        gl
    } else {
        let error = format!(
            "Unable to get GL context when compiling shader '{}', in {:?}",
            name, canvas
        );
        return Err(ErrorStr::new(error));
    };
    let shader_type: u32 = {
        match shader_type {
            ShaderType::VertexShader => WebGl2RenderingContext::VERTEX_SHADER,
            ShaderType::FragmentShader => WebGl2RenderingContext::FRAGMENT_SHADER,
        }
    };
    let shader = if let Some(shader) = gl.create_shader(shader_type) {
        shader
    } else {
        return Err(ErrorStr::new("Unable to create shader object"));
    };
    gl.shader_source(&shader, program_source);
    gl.compile_shader(&shader);
    let compile_status = if let Some(status) = gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
    {
        status
    } else {
        let error = format!(
            "Unable to get shader compile status for shader '{}', in {:?}",
            name, canvas
        );
        return Err(ErrorStr::new(error));
    };
    if !compile_status {
        let compile_error = if let Some(error) = gl.get_shader_info_log(&shader) {
            error
        } else {
            let error = format!(
                "Unable to get shader compilation error for shader '{}', in {:?}",
                name, canvas
            );
            return Err(ErrorStr::new(error));
        };
        let error = format!(
            "Compilation error for shader '{}': {}. In: {:?}",
            name, compile_error, canvas
        );
        return Err(ErrorStr::new(error));
    }
    Ok(shader)
}
