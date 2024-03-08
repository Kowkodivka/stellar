use std::{f32::consts::PI, ffi::CString};

use object::{create_program, IBO, VAO, VBO};
use sdl2::{event::Event, keyboard::Keycode};

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

    let mut time: f32 = 0.0;

    let mut angle_y: f32 = 0.0;
    let mut angle_x: f32 = 0.0;

    let mut camera_position = vec![0.0, 1.0, 0.0];
    let resolution = vec![800.0, 600.0];

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
                    Keycode::Left => {
                        angle_x += 0.1;
                    }
                    Keycode::Right => {
                        angle_x -= 0.1;
                    }
                    Keycode::Up => {
                        angle_y += 0.1;
                    }
                    Keycode::Down => {
                        angle_y -= 0.1;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        unsafe {
            let camera_uniform_location =
                gl::GetUniformLocation(program.id(), camera_position_name.as_ptr());
            gl::Uniform3f(
                camera_uniform_location,
                camera_position[0],
                camera_position[1],
                camera_position[2],
            );
            let resolution_uniform_location =
                gl::GetUniformLocation(program.id(), resolution_name.as_ptr());
            gl::Uniform2f(resolution_uniform_location, resolution[0], resolution[1]);
            let angle_x_uniform_location =
                gl::GetUniformLocation(program.id(), angle_x_name.as_ptr());
            gl::Uniform1f(angle_x_uniform_location, angle_x);
            let angle_y_uniform_location =
                gl::GetUniformLocation(program.id(), angle_y_name.as_ptr());
            gl::Uniform1f(angle_y_uniform_location, angle_y);
            let time_uniform_location = gl::GetUniformLocation(program.id(), time_name.as_ptr());
            gl::Uniform1f(time_uniform_location, time);
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
