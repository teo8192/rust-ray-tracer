extern crate cgmath;
extern crate sdl2;

use cgmath::Point3;
use cgmath::Vector3;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub mod rays;
pub mod render;
pub mod shapes;

fn check_keyboard(
    event_pump: &mut sdl2::EventPump,
    running: &mut bool,
    origin: &mut Point3<f32>,
    camdir: &mut rays::CamDir,
    scale: &mut usize,
) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                *running = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                origin.z += 1.;
                camdir.update(*origin);
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                origin.z -= 1.;
                camdir.update(*origin);
            }
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                origin.x += 1.;
                camdir.update(*origin);
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                origin.x -= 1.;
                camdir.update(*origin);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                origin.y -= 1.;
                camdir.update(*origin);
            }
            Event::KeyDown {
                keycode: Some(Keycode::LShift),
                ..
            } => {
                origin.y += 1.;
                camdir.update(*origin);
            }
            Event::KeyDown {
                keycode: Some(Keycode::P),
                ..
            } => *scale = 8,
            Event::KeyDown {
                keycode: Some(Keycode::M),
                ..
            } => *scale = 1,
            _ => {}
        }
    }
}

fn color_to_rgb(color: u32) -> Color {
    let b = color % 256;
    let g = (color >> 8) % 256;
    let r = (color >> 16) % 256;

    Color::RGB(r as u8, g as u8, b as u8)
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

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;
    //let mut pixels = render::Pixels::new(w, h);
    let mut scale = 8;

    let mut origin: Point3<f32> = Point3::new(0., 2., -10.);

    let mut camdir = rays::CamDir::new(origin, Point3::new(0., 0., 0.));

    let hyperboloid =
        shapes::Hyperboloid::new(-1., Point3::new(0., 0., 0.), Vector3::new(1., 1., 1.));
    let sphere = shapes::Sphere::new(2.1, Point3::new(0., 0., 0.));
    let mut objects = shapes::Shapes::new();
    objects.add(&sphere);
    objects.add(&hyperboloid);

    let mut running = true;
    while running {
        check_keyboard(
            &mut event_pump,
            &mut running,
            &mut origin,
            &mut camdir,
            &mut scale,
        );

        let get_color = |x, y| -> u32 {
            rays::Ray::from_camdir(&camdir, rays::CamDir::uv(x, y, w / scale, h / scale))
                .intersection(&objects)
        };

        for x in 0..w / scale {
            for y in 0..h / scale {
                let color = get_color(x, y);
                canvas.set_draw_color(color_to_rgb(color));
                canvas.fill_rect(Rect::new(
                    (x * scale) as i32,
                    (y * scale) as i32,
                    scale as u32,
                    scale as u32,
                ))?;
            }
        }

        canvas.present();
    }

    Ok(())
}
