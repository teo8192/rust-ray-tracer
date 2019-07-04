extern crate cgmath;

struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    pub fn intersection(&self, shapes: Shapes) -> f32 {
        let mut min_t = 100000.;
        for shape in shapes.shapes {
            min_t = match shape.intersection(self) {
                Ok(t) => {
                    if t < min_t {
                        t
                    }
                }
                Err(err) => min_t,
            }
        }

        min_t
    }

    pub 
}
