//! Some basic mathematical shapes to be used with ray-tracing

extern crate cgmath;
extern crate roots;

use super::rays;
use cgmath::*;
use roots::find_roots_quartic;
use roots::Roots;

const MIN_T: f32 = 0.01;

/// If the point on the ray is behind the camera
/// or have values like NaN and inf
fn abc(a: f32, b: f32, c: f32) -> Option<f32> {
    let num = b * b - 4. * a * c;

    if num < 0. {
        None
    } else {
        let t1 = (-b + num.sqrt()) / (2. * a);
        let t2 = (-b - num.sqrt()) / (2. * a);

        min_g0(t1, t2)
    }
}

fn min_g0(a: f32, b: f32) -> Option<f32> {
    if a < MIN_T && b < MIN_T {
        None
    } else if a < MIN_T || b <= a {
        Some(b)
    } else if b < MIN_T || a <= b {
        Some(a)
    } else {
        None
    }
}

/// A shape is something that may intersect a ray at some point in space
pub trait Shape {
    /// The intersection closest point between the shape and a ray, it it exists
    fn intersection(&self, ray: &rays::Ray) -> Option<Material>;
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
pub struct Material {
    pub t: f32,
    pub normal: Option<Vector3<f32>>,
}

/// An infinetly long tube
pub struct Cylinder {
    radius: f32,
    origin: Point3<f32>,
}

impl Cylinder {
    /// Origin is some point inside the tube. Since the tube is infinite in the z-direction, only
    /// the xy components of the origin vector is relevant.
    pub fn new(radius: f32, origin: Point3<f32>) -> Cylinder {
        Cylinder { radius, origin }
    }
}

impl Shape for Cylinder {
    fn intersection(&self, ray: &rays::Ray) -> Option<Material> {
        let origin = ray.origin - self.origin;
        let sq = |x| -> f32 { x * x };

        let a = sq(ray.direction.x) + sq(ray.direction.y);
        let b = 2. * (origin.x * ray.direction.x + origin.y * ray.direction.y);
        let c = sq(origin.x) + sq(origin.y) - sq(self.radius);

        match abc(a, b, c) {
            Some(t) => {
                let origin = Vector3::new(ray.origin.x, ray.origin.y, ray.origin.z);
                let mut normal = origin + ray.direction * t;
                normal.z = 0.;
                let normal = Some(normal);

                Some(Material { t, normal })
            }
            None => None,
        }
    }
}

// {{{ TORUS
pub struct Torus {
    inner_radius: f32,
    tube_radius: f32,
    origin: Point3<f32>,
}

impl Torus {
    pub fn new(inner_radius: f32, tube_radius: f32, origin: Point3<f32>) -> Torus {
        Torus {
            inner_radius,
            tube_radius,
            origin,
        }
    }
}

impl Shape for Torus {
    fn intersection(&self, ray: &rays::Ray) -> Option<Material> {
        let sq = |x| -> f32 { x * x };

        let Rsq = sq(self.inner_radius);
        let rsq = sq(self.tube_radius);

        let origin = ray.origin - self.origin;

        let a1 = ray.direction.magnitude2();
        let b1 = 2.
            * (origin.x * ray.direction.x
                + origin.y * ray.direction.y
                + origin.z * ray.direction.z);
        let c11 = origin.magnitude2();
        let c12 = Rsq - rsq;

        let a = sq(a1);
        let b = 2. * a1 * b1;
        let c = 2. * a1 * (c11 + c12) + sq(b1) - 4. * Rsq * (c11 - sq(origin.z));
        let d = 2. * b1 * (c11 + c12) - 4. * Rsq * (b1 - 2. * origin.z * ray.direction.z);
        let e = sq(c11 + c12) - 4. * Rsq * (c11 - sq(origin.z));

        let t = match find_roots_quartic(a, b, c, d, e) {
            Roots::Four(roots) => {
                let mut min_root = roots[0];
                if roots[1] < min_root && roots[1] > MIN_T {
                    min_root = roots[1];
                }
                if roots[2] < min_root && roots[2] > MIN_T {
                    min_root = roots[2];
                }
                if roots[3] < min_root && roots[3] > MIN_T {
                    min_root = roots[3];
                }

                min_root
            }
            Roots::Three(roots) => {
                let mut min_root = roots[0];
                if roots[1] < min_root && roots[1] > MIN_T {
                    min_root = roots[1];
                }
                if roots[2] < min_root && roots[2] > MIN_T {
                    min_root = roots[2];
                }

                min_root
            }
            Roots::Two(roots) => {
                let mut min_root = roots[0];
                if roots[1] < min_root && roots[1] > MIN_T {
                    min_root = roots[1];
                }

                min_root
            }
            Roots::One(roots) => roots[0],
            _ => -1.,
        };

        if t < 0. {
            None
        } else {
            Some(Material { t, normal: None })
        }
    }
}
// }}}

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
    fn intersection(&self, ray: &rays::Ray) -> Option<Material> {
        let origin = ray.origin - self.origin;
        let denom = self.normal.x * ray.direction.x
            + self.normal.y * ray.direction.y
            + self.normal.z * ray.direction.z;

        if denom == 0. {
            // Looking parallell to the plane
            None
        } else {
            let t =
                -(self.normal.x * origin.x + self.normal.y * origin.y + self.normal.x * origin.y)
                    / denom;
            if t <= MIN_T {
                // plane is behind
                None
            } else {
                Some(Material {
                    t,
                    normal: Some(self.normal),
                })
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
    fn intersection(&self, ray: &rays::Ray) -> Option<Material> {
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
            Some(t) => Some(Material { t, normal: None }),
            None => None,
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
    fn intersection(&self, ray: &rays::Ray) -> Option<Material> {
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
            Some(t) => Some(Material {
                t,
                normal: Some((origin + t * direction).normalize()),
            }),
            None => None,
        }
    }
}
