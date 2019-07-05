//! Some basic mathematical shapes to be used with ray-tracing

extern crate cgmath;

use super::rays;
use cgmath::*;

/// If the point on the ray is behind the camera
/// or have values like NaN and inf
pub enum IntersectErr {
    IsBehind,
    Other,
}

fn abc(a: f32, b: f32, c: f32) -> Result<f32, IntersectErr> {
    let num = b * b - 4. * a * c;
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
        Err(IntersectErr::Other)
    }
}

/// A shape is something that may intersect a ray at some point in space
pub trait Shape {
    /// The intersection closest point between the shape and a ray, it it exists
    fn intersection(&self, ray: &rays::Ray) -> Result<Material, IntersectErr>;
}

/// Contains some shapes that a ray can intersect with.
/// This should be passed in to a ray, so the ray can calculate the color if the intersection
/// point.
pub struct Shapes<'a> {
    pub shapes: Vec<&'a Shape>,
}

impl<'a> Shapes<'a> {
    pub fn new() -> Shapes<'a> {
        let shapes = Vec::new();
        Shapes { shapes }
    }

    /// Add a shape the the collection of shapes
    pub fn add(&mut self, shape: &'a Shape) {
        self.shapes.push(shape);
    }
}

/// The material of a point
pub enum Material {
    Nothing,
    Spheroid(f32, Vector3<f32>),
    Hyperboloid(f32),
    Plane(f32, Vector3<f32>),
}

/// The plane is a flat 3-dimensional surface.
/// It is defined with a point on the plane (calling it the origin)
/// and a vector perpendicular to the plane.
pub struct Plane {
    normal: Vector3<f32>,
    origin: Point3<f32>,
}

impl Plane {
    /// Returns a plane with the properties you specify
    pub fn new(normal: Vector3<f32>, origin: Point3<f32>) -> Plane {
        Plane {
            normal: normal.normalize(),
            origin,
        }
    }
}

impl Shape for Plane {
    fn intersection(&self, ray: &rays::Ray) -> Result<Material, IntersectErr> {
        let origin = ray.origin - self.origin;
        let denom = self.normal.x * ray.direction.x
            + self.normal.y * ray.direction.y
            + self.normal.z * ray.direction.z;

        if denom == 0. {
            // Looking parallell to the plane
            Err(IntersectErr::Other)
        } else {
            let t =
                -(self.normal.x * origin.x + self.normal.y * origin.y + self.normal.x * origin.y)
                    / denom;
            if t <= 0. {
                // plane is behind
                Err(IntersectErr::Other)
            } else {
                Ok(Material::Plane(t, self.normal))
            }
        }
    }
}

/// The hyperboloid is a shape that looks like two cones stuck together.
/// If you have lambda equal to 0, the cones will bearly touch.
/// Positive lambda makes them fuse together and a negative gives a void between them
pub struct Hyperboloid {
    lambda: f32,
    origin: Point3<f32>,
    dimensions: Vector3<f32>,
}

impl Hyperboloid {
    /// Origin is the point everything is offset by.
    /// If lambda is 0, the origin is the point where the two cones meet.
    /// The dimensions is how ''squished'' the separate dimensions is.
    /// Pass it as (1, 1, 1) if you want it to look symetrical.
    /// A number > 1 will stretch out that dimension and a number < 1 will squish it
    pub fn new(lambda: f32, origin: Point3<f32>, dimensions: Vector3<f32>) -> Hyperboloid {
        Hyperboloid {
            lambda,
            origin,
            dimensions: Vector3::new(
                dimensions.x * dimensions.x,
                dimensions.y * dimensions.y,
                dimensions.z * dimensions.z,
            ),
        }
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
        let b =
            2. * origin.x * direction.x + 2. * origin.y * direction.y - 2. * origin.z * direction.z;
        let c = square(origin.x) + square(origin.y) - square(origin.z) - self.lambda;

        match abc(a, b, c) {
            Ok(t) => Ok(Material::Hyperboloid(t)),
            Err(err) => Err(err),
        }
    }
}

/// A spheriod is to a sphere what a square is to a rectangle.
/// Dimensions where all axis equal each other gives a sphere.
pub struct Spheroid {
    radius: f32,
    origin: Point3<f32>,
    dimensions: Vector3<f32>,
}

impl Spheroid {
    pub fn new(radius: f32, origin: Point3<f32>, dimensions: Vector3<f32>) -> Spheroid {
        Spheroid {
            radius,
            origin,
            dimensions: Vector3::new(
                dimensions.x * dimensions.x,
                dimensions.y * dimensions.y,
                dimensions.z * dimensions.z,
            ),
        }
    }
}

impl Shape for Spheroid {
    fn intersection(&self, ray: &rays::Ray) -> Result<Material, IntersectErr> {
        let mut origin = ray.origin - self.origin;
        origin.x /= self.dimensions.x;
        origin.y /= self.dimensions.y;
        origin.z /= self.dimensions.z;
        let mut direction = ray.direction;
        direction.x /= self.dimensions.x;
        direction.y /= self.dimensions.y;
        direction.z /= self.dimensions.z;

        let a = direction.magnitude2();
        let b =
            2. * origin.x * direction.x + 2. * origin.y * direction.y + 2. * origin.z * direction.z;
        let c = origin.magnitude2() - self.radius * self.radius;

        match abc(a, b, c) {
            Ok(t) => Ok(Material::Spheroid(t, -(origin + t * direction).normalize())),
            Err(err) => Err(err),
        }
    }
}
