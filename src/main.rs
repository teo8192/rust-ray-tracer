extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub mod render {
    use sdl2::video::WindowSurfaceRef;

    fn cap(c: f32, min: f32, max: f32) -> f32 {
        if c > max {
            return max;
        }
        if c < min {
            return min;
        }

        c
    }

    pub fn color(r: f32, g: f32, b: f32) -> u32 {
        let r = cap(r, 0., 1.);
        let g = cap(g, 0., 1.);
        let b = cap(b, 0., 1.);

        let mut color: u32 = (r * 255.) as u32;
        color *= 256;
        color += (g * 255.) as u32;
        color *= 256;
        color += (b * 255.) as u32;

        color
    }

    pub fn set_pixel(pixels: &mut Vec<u32>, w: usize, h: usize, x: usize, y: usize, color: u32) -> () {
        if x >= w || y > h {
            return ();
        }
        pixels[x + y * w] = color;

        ()
    }

    pub fn copy_to_surface(surface: &WindowSurfaceRef, pixels: &mut Vec<u32>) -> () {
        let (w, h) = surface.size();
        unsafe {
            let raw_surface = *surface.raw();
            raw_surface.pixels
                .copy_from(pixels.as_mut_ptr() as *mut std::ffi::c_void, w as usize * h as usize * 4);
        }
        ()
    }

    pub fn generate_pixels(w: usize, h: usize) -> Vec<u32> {
        let mut pixles: Vec<u32> = Vec::with_capacity(w * h);
        for _ in 0..(w * h) {
            pixles.push(0);
        }

        pixles
    }

    pub fn clear(pixels: &mut Vec<u32>, w: usize, h: usize) -> () {
        for x in  0..w {
            for y in 0..h {
                pixels[x + w * y] = 0;
            }
        }
    }
}

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

    let mut pixles = render::generate_pixels(w, h);

    for x in 0..w {
        for y in 0..h {
            render::set_pixel(&mut pixles, w, h, x, y, render::color(1., 0., 1.));
        }
    }

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

        render::copy_to_surface(&surface, &mut pixles);
        surface.update_window().unwrap();

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
