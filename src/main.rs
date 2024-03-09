use std::ffi::CString;

use object::{create_program, IBO, VAO, VBO};
use sdl2::{event::Event, keyboard::Keycode};

use crate::winsdl::Winsdl;

mod object;
mod winsdl;

fn main() {
    let resolution = vec![1080.0, 720.0];

    let mut winsdl = Winsdl::new(
        "Raymarching",
        resolution[0] as usize,
        resolution[1] as usize,
    )
    .unwrap();
    unsafe {
        gl::Viewport(0, 0, resolution[0] as i32, resolution[1] as i32);
    }

    let program = create_program().unwrap();
    program.set();

    let mut time: f32 = 0.0;

    let mut angle_y: f32 = -0.35;
    let mut angle_x: f32 = 0.0;

    let mut camera_position = vec![0.0, 14.0, -30.0];

    let angle_x_name = CString::new("angleX").unwrap();
    let angle_y_name = CString::new("angleY").unwrap();
    let camera_position_name = CString::new("cameraPosition").unwrap();
    let resolution_name = CString::new("resolution").unwrap();
    let time_name = CString::new("time").unwrap();

    let verticles: Vec<f32> = vec![
        -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0,
    ];
    let indices: Vec<u32> = vec![0, 1, 2, 3, 4, 5];

    let vbo = VBO::gen();
    vbo.set(&verticles);

    let vao = VAO::gen();
    vao.set();

    let ibo = IBO::gen();
    ibo.set(&indices);

    'running: loop {
        time += 0.01;
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W => {
                        camera_position[0] -= angle_x.sin() * 0.5;
                        camera_position[1] += angle_y.sin() * 0.5;
                        camera_position[2] += angle_x.cos() * 0.5;
                    }
                    Keycode::S => {
                        camera_position[0] += angle_x.sin() * 0.5;
                        camera_position[1] -= angle_y.sin() * 0.5;
                        camera_position[2] -= angle_x.cos() * 0.5;
                    }
                    Keycode::A => {
                        camera_position[0] -= angle_x.cos() * 0.5;
                        camera_position[2] -= angle_x.sin() * 0.5;
                    }
                    Keycode::D => {
                        camera_position[0] += angle_x.cos() * 0.5;
                        camera_position[2] += angle_x.sin() * 0.5;
                    }
                    Keycode::Left => {
                        angle_x += 0.05;
                    }
                    Keycode::Right => {
                        angle_x -= 0.05;
                    }
                    Keycode::Up => {
                        angle_y += 0.05;
                    }
                    Keycode::Down => {
                        angle_y -= 0.05;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        unsafe {
            gl::Uniform3f(
                gl::GetUniformLocation(program.id(), camera_position_name.as_ptr()),
                camera_position[0],
                camera_position[1],
                camera_position[2],
            );
            gl::Uniform2f(
                gl::GetUniformLocation(program.id(), resolution_name.as_ptr()),
                resolution[0],
                resolution[1],
            );
            gl::Uniform1f(
                gl::GetUniformLocation(program.id(), angle_x_name.as_ptr()),
                angle_x,
            );
            gl::Uniform1f(
                gl::GetUniformLocation(program.id(), angle_y_name.as_ptr()),
                angle_y,
            );
            gl::Uniform1f(
                gl::GetUniformLocation(program.id(), time_name.as_ptr()),
                time,
            );
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
