extern crate cgmath;

enum IntersectErr {
    IsBehind,
}

fn min_g0(a: f32, b: f32) -> Result<f32, IntersectErr> {
    if a < 0 && b < 0 {
        return Err(IsBehind);
    }
    if a < 0 || b < a {
        return Ok(b);
    }
    if b < 0 || a < b {
        return Ok(a);
    }
}

trait Shape {
    fn intersection(&self, ray: Ray) -> Result(f32, IntersectErr);
}

pub struct Shapes {
    shapes: Vec<Shape>,
}

impl Shapes {
    pub fn new() -> Shapes {
        let shapes: Vec<Shape> = Vec::new();
        Shapes { shapes }
    }
}

struct Sphere {
    radius: f32,
    origin: Point3,
}

impl Sphere {
    pub fn new(radius: f32, origin: Point3) -> Sphere {
        Sphere { radius, origin }
    }
}

impl Shape for Sphere {
    pub fn intersection(&self, ray: Ray) -> Result(f32, IntersectErr) {
        let square = |num: f32| -> f32 { num * num };

        let origin = ray.origin - self.origin;
        let a = square(ray.direction.x) + square(ray.direction.y) + square(ray.direction.z);
        let b = 2. * origin.x * ray.direction.x
            + 2. * origin.y * ray.direction.y
            + 2. * origin.z * ray.direction.z;
        let c = squre(origin.x) + square(origin.y) + square(origin.z);

        let num = square(b) - 4. * a * c;
        if num < 0 {
            return Err(IsBehind);
        }

        let t1 = (-b + num.sqrt()) / (2. * a);
        let t2 = (-b - num.sqrt()) / (2. * a);

        min_g0(t1, t2)
    }
}
