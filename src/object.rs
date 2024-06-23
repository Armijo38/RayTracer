use crate::shapes::shape::{Shape,IntersectionResult,NoneShape};
use crate::vec::Vec3;
use crate::matrix::Matrix33;
use serde::{Serialize,Deserialize};


#[derive(Serialize,Deserialize,Debug)]
pub struct Object {
    #[serde(default = "Vec3::new_default")]
    pub position: Vec3,
    #[serde(default = "Vec3::new_unit")]
    pub size: Vec3,
    #[serde(default)]
    pub rotation_angle: [i16; 3],
    #[serde(default)]
    pub rotation: Matrix33,
    #[serde(default)]
    pub reverse_rotation: Matrix33,
    #[serde(default)]
    pub specular: u32,
    #[serde(default)]
    pub reflection: f32,
    //pub transparency: f32,
    //pub refractive: f32,
    #[serde(default = "Object::default_color")]
    pub color: [f32; 3],
    pub shape: Box<dyn Shape>,
}

impl Object {
    #[allow(dead_code)]
    pub fn set_position(mut self, position: Vec3) -> Object {
        self.position = position;
        self
    }

    #[allow(dead_code)]
    pub fn set_size(mut self, size: Vec3) -> Object {
        self.size = size;
        self
    }

    pub fn calc_rotation(x_phi: f32, y_phi: f32, z_phi: f32) -> Matrix33 {
        let x_angle: f32 = x_phi / 180.0 * std::f32::consts::PI;
        let y_angle: f32 = y_phi / 180.0 * std::f32::consts::PI;
        let z_angle: f32 = z_phi / 180.0 * std::f32::consts::PI;

        let x_matrix = Matrix33::new([1.0, 0.0, 0.0,
                                      0.0, f32::cos(x_angle), -f32::sin(x_angle),
                                      0.0, f32::sin(x_angle), f32::cos(x_angle)]);
        let y_matrix = Matrix33::new([f32::cos(y_angle), 0.0, f32::sin(y_angle),
                                      0.0, 1.0, 0.0,
                                      -f32::sin(y_angle), 0.0, f32::cos(y_angle)]);
        let z_matrix = Matrix33::new([f32::cos(z_angle), -f32::sin(z_angle), 0.0,
                                      f32::sin(z_angle), f32::cos(z_angle), 0.0,
                                      0.0, 0.0, 1.0]);
        x_matrix * y_matrix * z_matrix
    }

    #[allow(dead_code)]
    pub fn set_rotation(mut self, x_phi: i32, y_phi: i32, z_phi: i32) -> Object {
        self.rotation = Object::calc_rotation(x_phi as f32, y_phi as f32, z_phi as f32);
        self.reverse_rotation = Object::calc_rotation(-x_phi as f32, y_phi as f32, z_phi as f32);
        self
    }

    #[allow(dead_code)]
    pub fn set_specular(mut self, specular: u32) -> Object {
        self.specular = specular;
        self
    }

    #[allow(dead_code)]
    pub fn set_reflection(mut self, reflection: f32) -> Object {
        self.reflection = reflection;
        self
    }

    #[allow(dead_code)]
    pub fn set_color(mut self, color: [f32; 3]) -> Object {
        self.color = color;
        self
    }

    pub fn default_color() -> [f32; 3] {
        [1.0, 1.0, 1.0]
    }

    pub fn new(shape: Box<dyn Shape>) -> Object {
        Object {
            position: Vec3::new_default(),
            rotation_angle: [0, 0, 0],
            rotation: Matrix33::new_default(),
            reverse_rotation: Matrix33::new_default(),
            size: Vec3::new(1.0, 1.0, 1.0),
            specular: 0,
            reflection: 0.0,
            color: [1.0, 1.0, 1.0],
            shape
        }
    }

    pub fn intersects(&self, start: &Vec3, ray: &Vec3) -> Option<(IntersectionResult, &Object)> {
        //let r2 = ray.clone();

        let start = start - &self.position;
        let start = &self.rotation * start;
        let start = start / &self.size;

        let ray = &self.rotation * ray;
        let ray = ray / &self.size;
        match self.shape.intersects(&start, &ray) {
            None => None,
            Some(mut intersection) => {
                intersection.norm = intersection.norm * &self.rotation;
                Some((intersection,
                      self))
            }
        }
    }

    pub fn init(&mut self) {
        self.rotation = Object::calc_rotation(self.rotation_angle[0] as f32, self.rotation_angle[1] as f32, self.rotation_angle[2] as f32);
        self.reverse_rotation = Object::calc_rotation(-self.rotation_angle[0] as f32, self.rotation_angle[1] as f32, self.rotation_angle[2] as f32);
        self.shape.init()
    }
}

impl Default for Object {
    fn default() -> Object {
        Object {
            position: Vec3::default(),
            size: Vec3::default(),
            rotation_angle: [0, 0, 0],
            rotation: Matrix33::new_default(),
            reverse_rotation: Matrix33::new_default(),
            specular: u32::default(),
            reflection: f32::default(),
            color: <[f32; 3]>::default(),
            shape: Box::new(NoneShape::new()),
        }
    }
}
