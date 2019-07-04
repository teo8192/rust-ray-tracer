extern crate cgmath;

use cgmath::*;
use super::rays;

pub enum IntersectErr {
    IsBehind, Other
}

fn abc(a: f32, b: f32, c: f32) -> Result<f32, IntersectErr> {
    let num = square(b) - 4. * a * c;
    if num < 0. {
        return Err(IntersectErr::IsBehind);
    }

    let t1 = (-b + num.sqrt()) / (2. * a);
    let t2 = (-b - num.sqrt()) / (2. * a);

    min_g0(t1, t2)
}

fn min_g0(a: f32, b: f32) -> Result<f32, IntersectErr> {
    if a < 0. && b < 0. {
        Err(IntersectErr::IsBehind)
    } else if a < 0. || b <= a {
        Ok(b)
    } else if b < 0. || a <= b {
        Ok(a)
    } else {
        println!("{} {}", a, b);
        Err(IntersectErr::Other)
    }
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

pub enum Material {
    Nothing,
    Sphere(f32, Vector3<f32>),
    Hyperboloid(f32)
}

pub struct Hyperboloid {
    lambda: f32,
    origin: Point3<f32>,
    dimensions: Vector3<f32>,
}

impl Hyperboloid {
    pub fn new(lambda: f32, origin: Point3<f32>, dimensions: Vector3<f32>) -> Hyperboloid {
        Hyperboloid { lambda, origin, dimensions }
    }
}

impl Shape for Hyperboloid {
    fn intersection(&self, ray: &rays::Ray) -> Result<Material, IntersectErr> {
        let square = |num: f32| -> f32 { num * num };

        let mut origin = ray.origin - self.origin;
        origin.x /= self.dimensions.x;
        origin.y /= self.dimensions.y;
        origin.z /= self.dimensions.z;
        let mut direction = ray.direction;
        direction.x /= self.dimensions.x;
        direction.y /= self.dimensions.y;
        direction.z /= self.dimensions.z;

        let a = square(direction.x) + square(direction.y) - square(direction.z);
        let b = 2. * origin.x * direction.x
            + 2. * origin.y * direction.y
            - 2. * origin.z * direction.z;
        let c =  square(origin.x) + square(origin.y) - square(origin.z)- self.lambda;

        match abc(a, b, c) {
            Ok(t) => {
                Ok(Material::Hyperboloid(t))
            },
            Err(err) => Err(err),
        }
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

        let a = ray.direction.magnitude2();
        let b = 2. * origin.x * ray.direction.x
            + 2. * origin.y * ray.direction.y
            + 2. * origin.z * ray.direction.z;
        let c = origin.magnitude2() - self.radius * self.radius;

        match abc(a, b, c) {
            Ok(t) => {
                let mut p = t * ray.direction;
                p.x += origin.x;
                p.y += origin.y;
                p.z += origin.z;
                let normal = p.normalize();

                Ok(Material::Sphere(t, normal))
            },
            Err(err) => Err(err),
        }
    }
}
