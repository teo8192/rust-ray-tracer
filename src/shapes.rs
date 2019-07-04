extern crate cgmath;

use cgmath::*;
use super::rays;

pub enum IntersectErr {
    IsBehind, Other
}

fn min_g0(a: f32, b: f32) -> Result<f32, IntersectErr> {
    if a < 0. && b < 0. {
        return Err(IntersectErr::IsBehind);
    }
    if a < 0. || b <= a {
        return Ok(b);
    }
    if b < 0. || a <= b {
        return Ok(a);
    }
    println!("Something is wrong, {} {}", a, b);
    return Err(IntersectErr::Other)
}

pub trait Shape {
    fn intersection(&self, ray: &rays::Ray) -> Result<Material, IntersectErr>;
}

pub struct Shapes {
    shapes: i32,
}

impl Shapes {
    pub fn new() -> Shapes {
        //let shapes: Vec<Shape> = Vec::new();
        //Shapes { shapes }
        let shapes = 0;
        Shapes { shapes } 
    }

    pub fn add(&mut self, shape: &Shape) {
        self.shapes = 0;
    }
}

pub enum Materials {
    NONE,
    SPHERE,
}

pub struct Material {
    pub t: f32,
    pub material: Materials,
}

impl Material {
    pub fn new() -> Material {
        let t = 0.;
        let material = Materials::NONE;
        Material { t, material }
    }
}

pub struct Sphere {
    radius: f32,
    origin: Point3<f32>,
}

impl Sphere {
    pub fn new(radius: f32, origin: Point3<f32>) -> Sphere {
        Sphere { radius, origin }
    }
}

impl Shape for Sphere {
    fn intersection(&self, ray: &rays::Ray) -> Result<Material, IntersectErr> {
        let square = |num: f32| -> f32 { num * num };

        let origin = ray.origin - self.origin;
        //let a = square(ray.direction.x) + square(ray.direction.y) + square(ray.direction.z);
        let a = ray.direction.magnitude2();
        let b = 2. * origin.x * ray.direction.x
            + 2. * origin.y * ray.direction.y
            + 2. * origin.z * ray.direction.z;
        //let c = square(origin.x) + square(origin.y) + square(origin.z);
        let c = origin.magnitude2() - self.radius * self.radius;

        let num = square(b) - 4. * a * c;
        if num < 0. {
            return Err(IntersectErr::IsBehind);
        }

        let t1 = (-b + num.sqrt()) / (2. * a);
        let t2 = (-b - num.sqrt()) / (2. * a);

        //let t1 = 10.;

        let material = Materials::SPHERE;

        match min_g0(t1, t2) {
            Ok(t) => Ok(Material { t, material }),
            Err(err) => Err(err),
        }
    }
}
