//! A simple ray-tracer implemented in rust using SDL2 for video output and cgmath for vector
//! operations. Planning to integrate ArrayFire for faster calculation on the GPU.

extern crate cgmath;
extern crate sdl2;

use cgmath::Point3;
use cgmath::Vector3;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub mod keyboard;
pub mod rays;
pub mod render;
pub mod shapes;

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

    let mut scale = 8;

    let mut origin: Point3<f32> = Point3::new(0., 2., -10.);

    let mut camdir = rays::CamDir::new(origin, Point3::new(0., 0., 0.));

    //let hyperboloid =
    //shapes::Hyperboloid::new(-1., Point3::new(0., 0., 0.), Vector3::new(1., 0.5, 1.));
    let sphere = shapes::Spheroid::new(2.1, Point3::new(0., 0., 0.), Vector3::new(1., 1., 0.7));
    let plane = shapes::Plane::new(Vector3::new(0., 1., 0.), Point3::new(0., 0., 0.));
    let mut objects = shapes::Shapes::new();
    objects.add(&sphere);
    //objects.add(&hyperboloid);
    objects.add(&plane);

    let mut running = true;
    while running {
        keyboard::check_keyboard(
            &mut event_pump,
            &mut running,
            &mut origin,
            &mut camdir,
            &mut scale,
        );

        let (w, h) = canvas.output_size()?;

        println!("{} {}", w, h);

        let get_color = |x, y| -> u32 {
            rays::Ray::from_camdir(
                &camdir,
                rays::CamDir::uv(x, y, w as usize / scale, h as usize / scale),
            )
            .intersection(&objects)
        };

        for x in 0..w as usize / scale {
            for y in 0..h as usize / scale {
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
