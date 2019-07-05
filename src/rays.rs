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

        Ray { origin, direction }
    }

    /// Returns a new ray calculated from the variables in the Camera
    pub fn from_camdir(camdir: &CamDir, uv: Vector2<f32>) -> Ray {
        let direction = (uv.x * camdir.cr + uv.y * camdir.cu + 2. * camdir.cf).normalize();
        let origin = camdir.origin;
        Ray { origin, direction }
    }

    fn closest_material_helper(
        &self,
        materials: &mut Vec<Material>,
        material: Material,
        t: f32,
    ) -> Material {
        let new_material_maybe = materials.pop();
        match new_material_maybe {
            None => material,
            Some(new_material) => match new_material {
                Material::Nothing => self.closest_material_helper(materials, material, t),
                Material::Spheroid(t1, n) => {
                    if t1 < t {
                        self.closest_material_helper(materials, Material::Spheroid(t1, n), t1)
                    } else {
                        self.closest_material_helper(materials, material, t)
                    }
                }
                Material::Plane(t1, n) => {
                    if t1 < t {
                        self.closest_material_helper(materials, Material::Plane(t1, n), t1)
                    } else {
                        self.closest_material_helper(materials, material, t)
                    }
                }
                Material::Hyperboloid(t1) => {
                    if t1 < t {
                        self.closest_material_helper(materials, Material::Hyperboloid(t1), t1)
                    } else {
                        self.closest_material_helper(materials, material, t)
                    }
                }
            },
        }
    }

    fn closest_material(&self, materials: &mut Vec<Material>) -> Material {
        let material = materials.pop();
        match material {
            None => Material::Nothing,
            Some(material) => match material {
                Material::Nothing => self.closest_material(materials),
                Material::Spheroid(t, n) => {
                    self.closest_material_helper(materials, Material::Spheroid(t, n), t)
                }
                Material::Hyperboloid(t) => {
                    self.closest_material_helper(materials, Material::Hyperboloid(t), t)
                }
                Material::Plane(t, n) => {
                    self.closest_material_helper(materials, Material::Plane(t, n), t)
                }
            },
        }
    }

    /// Find the closest intersection point to the ray origin, an return a color in HTML notation.
    pub fn intersection(&self, shapes: &shapes::Shapes) -> u32 {
        let mut materials: Vec<Material> = shapes
            .shapes
            .iter()
            .map(|x| -> Material {
                match x.intersection(&self) {
                    Ok(material) => material,
                    Err(_) => Material::Nothing,
                }
            })
            .filter(|x| -> bool {
                match x {
                    Material::Nothing => false,
                    _ => true,
                }
            })
            .collect();

        self.col(self.closest_material(&mut materials))
    }

    /// Return the color of a single intersection with a shape
    pub fn single_intersection(self, shape: &shapes::Shape) -> u32 {
        let material = match shape.intersection(&self) {
            Ok(mat) => mat,
            Err(_) => Material::Nothing,
        };

        self.col(material)
    }

    fn light(normal: Vector3<f32>) -> f32 {
        let l: Vector3<f32> = Vector3::new(-1., 2., -3.).normalize();
        let c = normal.dot(l);
        if c < 0. {
            0.
        } else {
            c
        }
    }

    /// Returns the color of a material
    pub fn col(&self, material: Material) -> u32 {
        match material {
            Material::Spheroid(_t, n) => {
                let c = Ray::light(n);

                render::color(c, c, c)
            }
            Material::Hyperboloid(t) => {
                let p = self.origin + t * self.direction;

                render::color(p.x.fract().abs(), p.y.fract().abs(), p.z.fract().abs())
            }
            Material::Plane(t, n) => {
                let p = self.origin + t * self.direction;
                let c = Ray::light(n);

                render::color(
                    p.x.fract().abs() * c,
                    p.y.fract().abs() * c,
                    p.z.fract().abs() * c,
                )
            }
            Material::Nothing => 0,
        }
    }
}
