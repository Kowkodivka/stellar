use sdl2::event::Event;

use crate::winsdl::Winsdl;

mod winsdl;

fn main() {
    let mut winsdl = Winsdl::new("Window", 800, 600).unwrap();
    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
    }
}
