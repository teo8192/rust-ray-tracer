extern crate cgmath;
extern crate sdl2;

use cgmath::Point3;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub mod rays;
pub mod render;
pub mod shapes;

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
    let mut scale = 8;

    let mut origin: Point3<f32> = Point3::new(0., 2., -10.);

    let mut camdir = rays::CamDir::new(origin, Point3::new(0., 0., 0.));

    //let sphere = shapes::Sphere::new(2.1, Point3::new(0., 0., 0.));
    //let shapes = shapes::Shapes::new();
    //shapes.add(sphere);
    let hyperboloid = shapes::Hyperboloid::new(-1., Point3::new(0., 0., 0.));

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
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    origin.z += 1.;
                    camdir.update(origin);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    origin.z -= 1.;
                    camdir.update(origin);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    origin.x += 1.;
                    camdir.update(origin);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    origin.x -= 1.;
                    camdir.update(origin);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    origin.y -= 1.;
                    camdir.update(origin);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::LShift),
                    ..
                } => {
                    origin.y += 1.;
                    camdir.update(origin);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    scale = 8
                }
                Event::KeyDown {
                    keycode: Some(Keycode::M),
                    ..
                } => {
                    scale = 1
                }
                _ => {}
            }
        }

        pixels.fill_color(render::color(1., 0., 1.));

        for x in 0..w/scale {
            for y in 0..h/scale {
                let uv = rays::CamDir::uv(x, y, w/scale, h/scale);
                let color = rays::Ray::from_camdir(&camdir, uv)
                    .color_single_intersection(&hyperboloid);
                for x1 in 0..scale {
                    for y1 in 0..scale {
                        pixels.set_pixel(x*scale + x1, y*scale + y1, color)?;
                    }
                }
            }
        }

        let surface = window.surface(&event_pump)?;

        pixels.copy_to_surface(&surface);
        surface.update_window().unwrap();

    }

    Ok(())
}
