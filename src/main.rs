extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub mod render;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let w: usize = 640;
    let h: usize = 480;

    let window = video_subsystem
        .window("SDL2", w as u32, h as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut pixels = render::Pixels::new(w, h);

    pixels.fill_color(render::color(1., 0., 1.));

    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }
                _ => {}
            }
        }

        let surface = window.surface(&event_pump)?;

        pixels.copy_to_surface(&surface);
        surface.update_window().unwrap();

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
