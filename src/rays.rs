extern crate cgmath;

use cgmath::*;
use super::shapes;
use super::render;

pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

pub struct CamDir {
    origin: Point3<f32>,
    target: Point3<f32>,
    cf: Vector3<f32>,
    cr: Vector3<f32>,
    cu: Vector3<f32>,
}

impl CamDir {
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
            cu
        }
    }

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

    pub fn from_camdir(camdir: &CamDir, uv: Vector2<f32>) -> Ray {
        let direction = (uv.x * camdir.cr + uv.y * camdir.cu + 2. * camdir.cf).normalize();
        let origin = camdir.origin;
        Ray { origin, direction }
    }

    pub fn intersection(&self, shaps: shapes::Shapes) -> shapes::Material {
        let min_t = shapes::Material::new();
        //for shape in shaps.shapes {
            //match shape.intersection(self) {
                //Ok(t) => {
                    //if t.t < min_t.t || min_t.material == shapes::Materials::NONE {
                        //min_t = t
                    //}
                //}
                //Err(err) => {}
            //}
        //}

        min_t
    }

    pub fn single_intersection(self, shape: &shapes::Shape) -> Result<shapes::Material, shapes::IntersectErr> {
        shape.intersection(&self)
    }

    pub fn color_single_intersection(self, shape: &shapes::Shape) -> u32 {
        let material = match shape.intersection(&self) {
            Ok(mat) => {
                mat
            },
            Err(_) => {
                shapes::Material::new()
            }
        };

        self.col(material)
    }

    fn light(normal: Vector3<f32>) -> f32 {
        let l: Vector3<f32> = Vector3::new(-1., -2., -3.).normalize();
        let c = normal.dot(l);
        if c < 0. {
            0.
        } else {
            c
        }
    }

    pub fn col(&self, material: shapes::Material) -> u32 {
        match material.material {
            shapes::Materials::NONE => {
                0
            },
            shapes::Materials::SPHERE => {
                let n = material.normal;
                let c = Ray::light(n);

                render::color(c, c, c)
            },
            _ => {
                let mut p = material.t * self.direction;
                p.x += self.origin.x;
                p.y += self.origin.y;
                p.z += self.origin.z;

                render::color(p.x.fract().abs(), p.y.fract().abs(), p.z.fract().abs())
            }
        }
    }
}
