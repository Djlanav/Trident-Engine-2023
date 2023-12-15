use std::error::Error;
use std::ops::Deref;
use std::process;
use std::default::Default;
use glium::backend::glutin::Display;
use glium::glutin::surface::WindowSurface;
use glium::{DrawError, implement_vertex, IndexBuffer, Program, Surface, VertexBuffer};
use glium::index::PrimitiveType::{Points, TrianglesList};
use glium::uniforms::EmptyUniforms;
use crate::easy_file_io;
use crate::easy_file_io::error_write_to_output;

#[derive(Copy, Clone)]
pub struct MyVertex {
    positions: [f32; 3],
}

fn quick_vertex() -> String {
    let code =
        "#version 460 core\n
        in vec3 Positions;\n
        void main() {\n
           gl_Position = vec4(Positions, 1.0f);\n
        }\n";

    let code_string = code.to_string();
    code_string
}

fn quick_pixel() -> String {
    let code =
        "#version 460 core\n
        out vec4 frag_color;\n
        void main() {\n
           frag_color = vec4(0.4f, 0.0f, 0.0f, 1.0f);\n
        }\n";

    let code_string = code.to_string();
    code_string
}

/// Apply implement_vertex! to MyVertex struct and initialize vertex and index buffers.
fn prepare_drawing(display: &Display<WindowSurface>) -> Result<(VertexBuffer<MyVertex>, IndexBuffer<u32>), Box<dyn Error>> {
    implement_vertex!(MyVertex, positions);

    let data = vec![
        MyVertex { positions: [-0.5, 0.5, 0.0] }, // 0

        MyVertex { positions: [-0.5, -0.5, 0.0] }, // 1

        MyVertex { positions: [0.5, -0.5, 0.0]  }, //2

        MyVertex { positions: [0.5, 0.5, 0.0] } // 3
    ];

    let indices = &[0, 1, 3, 3, 1, 2];

    let vertex_buffer = VertexBuffer::new(display, data.as_ref())?;
    let index_buffer = IndexBuffer::new(display, TrianglesList, indices)?;
    Ok((vertex_buffer, index_buffer))
}

pub struct DrawData {
    vertex_buffer: VertexBuffer<MyVertex>,
    index_buffer: IndexBuffer<u32>,
    program: Program
}

pub fn context_setup(display: &Display<WindowSurface>) -> DrawData {
    let (vertex_buffer, index_buffer) = prepare_drawing(&display).unwrap_or_else(
        |error| {
            let error_deref = error.deref();
            eprintln!("DRAW PREPARATION FAILED: {error_deref}");
            process::exit(1)
        });

    let vertex_shader = easy_file_io::basic_file_load("map_editor/shaders/vertex.glsl").unwrap_or_else(|e| {
        let error_deref = e.deref();
        eprintln!("Error occurred in loading shader file: {error_deref}");
        println!("Creating new base vertex shader...");
        quick_vertex()
    });

    let pixel_shader = easy_file_io::basic_file_load("map_editor/shaders/pixel.glsl").unwrap_or_else(|e| {
        let error_deref = e.deref();
        eprintln!("Error occurred in loading shader file: {error_deref}");
        println!("Creating new base pixel shader...");
        quick_pixel()
    });

    let program = match Program::from_source(display, vertex_shader.as_str(), pixel_shader.as_str(), None) {
        Ok(p) => p,
        Err(e) => {
            error_write_to_output("map_editor/engine_program_output.txt", e);
            process::exit(1);
        }
    };

    let draw_data = DrawData {
        vertex_buffer,
        index_buffer,
        program
    };

    draw_data
}

pub fn draw_to_context(display: &Display<WindowSurface>, draw_data: &DrawData) -> Result<(), DrawError> {
    let mut frame = display.draw();
    frame.clear_color(0.0, 0.4, 0.6, 1.0);
    frame.draw(&draw_data.vertex_buffer, &draw_data.index_buffer, &draw_data.program, &EmptyUniforms, &Default::default())?;
    frame.finish().unwrap();

    Ok(())
}