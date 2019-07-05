//! Read keyboard events

extern crate cgmath;
extern crate sdl2;

use cgmath::Point3;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use super::rays;

/// Check for events and update variables depending on the events
pub fn check_keyboard(
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
                origin.y += 1.;
                camdir.update(*origin);
            }
            Event::KeyDown {
                keycode: Some(Keycode::LShift),
                ..
            } => {
                origin.y -= 1.;
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
