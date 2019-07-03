extern crate sdl2;
use std::path::Path;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("SDL2", 640, 480)
        .position_centered().build().map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .accelerated().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));

    let mut timer = sdl_context.timer()?;

    let mut event_pump = sdl_context.event_pump()?;

    // animation sheet and extras are available from
    // https://opengameart.org/content/a-platformer-in-the-forest
    //let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/characters.bmp"))?;

    let points = [Point::new(100,100), Point::new(10, 10)];

    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },
                _ => {}
            }
        }

        //let ticks = timer.ticks() as i32;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        // copy the frame to the canvas

        canvas.set_draw_color(Color::RGB(255, 0, 255));
        canvas.draw_points(&points[..]);

        canvas.present();

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
