//! Stuff to do with rendering to screen

use sdl2::video::WindowSurfaceRef;

pub struct Pixels {
    pixels: Vec<u32>,
    w: usize,
    h: usize,
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

/// Rurn rgb (float between 0 and 1) to a unsigned interger kind of like HTML notation
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

        Pixels { pixels, w, h }
    }

    /// Sets a pixel in the pixel array to a color
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) -> Result<(), &str> {
        if x >= self.w || y > self.h {
            return Err("Out of bouds");
        }
        self.pixels[x + y * self.w] = color;

        Ok(())
    }

    /// Copies the pixel array to a surface
    pub fn copy_to_surface(&mut self, surface: &WindowSurfaceRef) -> () {
        let (w, h) = surface.size();
        unsafe {
            let raw_surface = *surface.raw();
            raw_surface.pixels.copy_from(
                self.pixels.as_mut_ptr() as *mut std::ffi::c_void,
                w as usize * h as usize * 4,
            );
        }
        ()
    }

    /// Sets the color of all pixels to black
    pub fn clear(&mut self) -> () {
        self.fill_color(0);
    }

    /// Sets the color of all pixels to the input color
    pub fn fill_color(&mut self, color: u32) -> () {
        for x in 0..self.w {
            for y in 0..self.h {
                self.pixels[x + self.w * y] = color;
            }
        }
    }
}
