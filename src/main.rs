extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub mod render {
    use sdl2::video::WindowSurfaceRef;

    pub struct Pixels {
        pixels: Vec<u32>,
        w: usize,
        h: usize
    }

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

    impl Pixels {
        pub fn new(w: usize, h: usize) -> Pixels {
            let mut pixels: Vec<u32> = Vec::with_capacity(w * h);
            for _ in 0..(w * h) {
                pixels.push(0);
            }

            Pixels {
                pixels,
                w,
                h
            }
        }

        pub fn set_pixel(&mut self,
                         x: usize,
                         y: usize,
                         color: u32) -> Result<(), &str> {
            if x >= self.w || y > self.h {
                return Err("Out of bouds");
            }
            self.pixels[x + y * self.w] = color;

            Ok(())
        }

        pub fn copy_to_surface(&mut self, surface: &WindowSurfaceRef) -> () {
            let (w, h) = surface.size();
            unsafe {
                let raw_surface = *surface.raw();
                raw_surface.pixels
                    .copy_from(self.pixels.as_mut_ptr() as *mut std::ffi::c_void, w as usize * h as usize * 4);
            }
            ()
        }

        pub fn clear(&mut self) -> () {
            for x in 0..self.w {
                for y in 0..self.h {
                    self.pixels[x + self.w * y] = 0;
                }
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

    let mut pixels = render::Pixels::new(w, h);

    for x in 0..w {
        for y in 0..h {
            pixels.set_pixel(x, y, render::color(1., 0., 1.))?;
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

        pixels.copy_to_surface(&surface);
        surface.update_window().unwrap();

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
