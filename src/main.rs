use object::{create_program, IBO, VAO, VBO};
use sdl2::event::Event;

use crate::winsdl::Winsdl;

mod object;
mod winsdl;

fn main() {
    let mut winsdl = Winsdl::new("Window", 800, 600).unwrap();
    unsafe {
        gl::Viewport(0, 0, 800, 600);
    }

    let program = create_program().unwrap();
    program.set();

    let verticles: Vec<f32> = vec![-0.5, -0.5, 0.5, -0.5, 0.5, 0.5];
    let indices: Vec<u32> = vec![0, 1, 2];

    let vbo = VBO::gen();
    vbo.set(&verticles);

    let vao = VAO::gen();
    vao.set();

    let ibo = IBO::gen();
    ibo.set(&indices);

    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as *const _,
            );
        }
        winsdl.window.gl_swap_window()
    }
}
