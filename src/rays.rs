//! The rays shot from the camera through every pixel

extern crate cgmath;

use super::render;
use super::shapes;
use super::shapes::Material;
use cgmath::*;

/// A single ray from the camera through a pixel
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
    lights: Vec<Point3<f32>>,
}

/// Contains some variables common for all rays
pub struct CamDir {
    origin: Point3<f32>,
    target: Point3<f32>,
    cf: Vector3<f32>,
    cr: Vector3<f32>,
    cu: Vector3<f32>,
}

impl CamDir {
    /// Returns the uv, witch is needed to calculate whitch way a ray should go.
    pub fn uv(x: usize, y: usize, w: usize, h: usize) -> Vector2<f32> {
        let mut uv = Vector2::new(x as f32 / w as f32 - 0.5, y as f32 / h as f32 - 0.5);
        uv.x *= w as f32 / h as f32;
        uv.y *= -1.;
        uv
    }

    pub fn new(origin: Point3<f32>, target: Point3<f32>) -> CamDir {
        let dir: Vector3<f32> = target - origin;
        let cf = dir.normalize();
        let cr = Vector3::new(0., 1., 0.).cross(cf).normalize();
        let cu = cf.cross(cr).normalize();

        CamDir {
            origin,
            target,
            cf,
            cr,
            cu,
        }
    }

    /// updates the camera to a new origin
    pub fn update(&mut self, origin: Point3<f32>) {
        let dir: Vector3<f32> = self.target - origin;
        self.cf = dir.normalize();
        self.cr = Vector3::new(0., 1., 0.).cross(self.cf).normalize();
        self.cu = self.cf.cross(self.cr).normalize();

        self.origin = origin;
    }
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: Vector3<f32>) -> Ray {
        let direction = direction.normalize();
        let mut lights: Vec<Point3<f32>> = Vec::new();
        lights.push(Point3::new(0., 1000., 0.));
        lights.push(Point3::new(10., 10., 0.));

        Ray {
            origin,
            direction,
            lights,
        }
    }

    /// Returns a new ray calculated from the variables in the Camera
    pub fn from_camdir(camdir: &CamDir, uv: Vector2<f32>) -> Ray {
        Ray::new(
            camdir.origin,
            (uv.x * camdir.cr + uv.y * camdir.cu + 2. * camdir.cf).normalize(),
        )
    }

    fn closest_material_helper(
        materials: &mut Vec<Option<Material>>,
        material: Material,
    ) -> Material {
        match materials.pop() {
            None => material,
            Some(new_material) => match new_material {
                None => material,
                Some(new_material) => {
                    if new_material.t < material.t {
                        Ray::closest_material_helper(materials, new_material)
                    } else {
                        material
                    }
                }
            },
        }
    }

    fn closest_material(&self, materials: &mut Vec<Option<Material>>) -> Option<Material> {
        match materials.pop() {
            Some(material) => match material {
                Some(mat) => Some(Ray::closest_material_helper(materials, mat)),
                None => None,
            },
            None => None,
        }
    }

    fn bounce(&self, shapes: &shapes::Shapes, point: Point3<f32>) -> (f32, Option<Point3<f32>>) {
        let get_light = |light_source: Point3<f32>| -> (f32, Option<Point3<f32>>) {
            match self.closest_material(
                &mut shapes.shapes(&Ray::new(point, (light_source - point).normalize())),
            ) {
                Some(material) => (
                    0.3,
                    Some(point + material.t * (light_source - point).normalize()),
                ),
                None => (1., None),
            }
        };
        let mut avg = 0.;
        let mut num = 0.;
        for light in &self.lights {
            let (light, _) = get_light(*light);
            num += 1.;
            avg += light;
        }

        (avg / num, None)
    }

    /// Find the closest intersection point to the ray origin, an return a color in HTML notation.
    pub fn intersection(&self, shapes: &shapes::Shapes) -> u32 {
        let (r, g, b, p) = self.col(self.closest_material(&mut shapes.shapes(&self)));

        let l = match p {
            Some(p) => {
                let (light, _) = self.bounce(&shapes, p);
                light
            }
            None => 1.,
        };

        render::color(r * l, g * l, b * l)
    }

    /// Return the color of a single intersection with a shape
    pub fn single_intersection<S: shapes::Shape>(&self, shape: &S) -> u32 {
        let (r, g, b, _) = self.col(shape.intersection(&self));
        render::color(r, g, b)
    }

    fn light(&self, normal: Vector3<f32>, point: Point3<f32>) -> f32 {
        let mut avg = 0.;
        let mut num = 0.;
        for light in &self.lights {
            avg += normal.dot((light - point).normalize());
            num += 1.;
        }
        let c = avg / num;
        if c < 0. {
            0.
        } else {
            c
        }
    }

    /// Returns the color of a material
    pub fn col(&self, material: Option<Material>) -> (f32, f32, f32, Option<Point3<f32>>) {
        match material {
            Some(material) => match material.normal {
                Some(normal) => {
                    let p = self.origin + material.t * self.direction;
                    let c = self.light(normal, p);

                    (
                        p.x.fract().abs() * c,
                        p.y.fract().abs() * c,
                        p.z.fract().abs() * c,
                        Some(p),
                    )
                }
                None => {
                    let p = self.origin + material.t * self.direction;

                    (
                        p.x.fract().abs(),
                        p.y.fract().abs(),
                        p.z.fract().abs(),
                        Some(p),
                    )
                }
            },
            None => (0., 0., 0., None),
        }
    }
}
