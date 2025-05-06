use leptos::logging;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::libs::{
    rendering::{
        canvas::WebGlCanvas,
        gl::shader::{Shader, ShaderType},
    },
    types::errors::ErrorStr,
};

struct Program<'a> {
    canvas: &'a WebGlCanvas<'a>,
    vertex_shader: &'a Shader<'a>,
    fragment_shader: &'a Shader<'a>,
    program: Option<WebGlProgram>,
    name: &'a str,
}

impl<'a> Program<'a> {
    pub fn new(
        canvas: &'a WebGlCanvas<'a>,
        vertex_shader: &'a Shader<'a>,
        fragment_shader: &'a Shader<'a>,
        name: &'a str,
    ) -> Self {
        let program = link_program(canvas, vertex_shader, fragment_shader, name);
        let program = match program {
            Ok(program) => program,
            Err(error) => {
                logging::error!("{}", error);
                return Self {
                    canvas,
                    vertex_shader,
                    fragment_shader,
                    program: None,
                    name,
                };
            }
        };
        Self {
            canvas,
            vertex_shader,
            fragment_shader,
            program: Some(program),
            name,
        }
    }

    pub fn use_program(&self) -> Result<(), ErrorStr> {
        let context = self.canvas.get_context();
        let gl = if let Some(gl) = context.as_ref() {
            gl
        } else {
            let error = format!(
                "Unable to get GL context when using program '{}', in {:?}",
                self.name, self.canvas
            );
            return Err(ErrorStr::new(error));
        };
        let program = if let Some(program) = self.program.as_ref() {
            program
        } else {
            let error = format!(
                "Tried to use non-existent program '{}', in {:?}",
                self.name, self.canvas
            );
            return Err(ErrorStr::new(error));
        };
        gl.use_program(Some(program));
        Ok(())
    }
}

fn link_program<'a>(
    canvas: &'a WebGlCanvas<'a>,
    vertex_shader: &'a Shader<'a>,
    fragment_shader: &'a Shader<'a>,
    name: &'a str,
) -> Result<WebGlProgram, ErrorStr> {
    let context = canvas.get_context();
    let gl = if let Some(gl) = context.as_ref() {
        gl
    } else {
        let error = format!(
            "Unable to get GL context when linking program '{}', in {:?}",
            name, canvas
        );
        return Err(ErrorStr::new(error));
    };

    let program = if let Some(program) = gl.create_program() {
        program
    } else {
        let error = format!("Unable to create program '{}', in {:?}", name, canvas);
        return Err(ErrorStr::new(error));
    };

    match vertex_shader.shader_type() {
        ShaderType::VertexShader => {}
        ShaderType::FragmentShader => {
            let error = format!(
                "Vertex shader '{}' is a fragment shader when linking program '{}', in {:?}",
                vertex_shader.name(),
                name,
                canvas,
            );
            return Err(ErrorStr::new(error));
        }
    }
    let vertex_shader = if let Some(shader) = vertex_shader.shader() {
        shader
    } else {
        let error = format!(
            "Unable to get vertex shader '{}' when linking program '{}', in {:?}",
            vertex_shader.name(),
            name,
            canvas
        );
        return Err(ErrorStr::new(error));
    };

    match fragment_shader.shader_type() {
        ShaderType::FragmentShader => {}
        ShaderType::VertexShader => {
            let error = format!(
                "Vertex shader '{}' is a fragment shader when linking program '{}', in {:?}",
                fragment_shader.name(),
                name,
                canvas,
            );
            return Err(ErrorStr::new(error));
        }
    }
    let fragment_shader = if let Some(shader) = fragment_shader.shader() {
        shader
    } else {
        let error = format!(
            "Unable to get vertex shader '{}' when linking program '{}', in {:?}",
            fragment_shader.name(),
            name,
            canvas
        );
        return Err(ErrorStr::new(error));
    };

    gl.attach_shader(&program, vertex_shader);
    gl.attach_shader(&program, fragment_shader);
    gl.link_program(&program);

    let link_status = if let Some(status) = gl
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
    {
        status
    } else {
        let error = format!(
            "Unable to get link status when linking program '{}', in {:?}",
            name, canvas
        );
        return Err(ErrorStr::new(error));
    };

    if link_status {
        Ok(program)
    } else {
        let linker_error = if let Some(error) = gl.get_program_info_log(&program) {
            error
        } else {
            let error = format!(
                "Unable to get program linker error for program '{}', in {:?}",
                name, canvas
            );
            return Err(ErrorStr::new(error));
        };
        let error = format!(
            "Linker error for program '{}': {}. In: {:?}",
            name, linker_error, canvas
        );
        Err(ErrorStr::new(error))
    }
}
