use crate::shapes::shape::{Shape,IntersectionResult};
use crate::vec::Vec3;
use crate::matrix::Matrix33;


pub struct Object {
    pub position: Vec3,
    pub size: Vec3,
    pub rotation: Matrix33,
    pub reverse_rotation: Matrix33,
    pub specular: u32,
    pub reflection: f32,
    //pub transparency: f32,
    //pub refractive: f32,
    pub color: [f32; 3],
    pub shape: Box<dyn Shape>,
}

impl Object {
    pub fn set_position(mut self, position: Vec3) -> Object {
        self.position = position;
        self
    }

    pub fn set_size(mut self, size: Vec3) -> Object {
        self.size = size;
        self
    }

    pub fn calc_rotation(x_phi: i32, y_phi: i32, z_phi: i32) -> Matrix33 {
        let x_angle: f32 = x_phi as f32 / 180.0 * std::f32::consts::PI;
        let y_angle: f32 = y_phi as f32 / 180.0 * std::f32::consts::PI;
        let z_angle: f32 = z_phi as f32 / 180.0 * std::f32::consts::PI;

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

    pub fn set_rotation(mut self, x_phi: i32, y_phi: i32, z_phi: i32) -> Object {
        self.rotation = Object::calc_rotation(x_phi, y_phi, z_phi);
        self.reverse_rotation = Object::calc_rotation(-x_phi, y_phi, z_phi);
        self
    }

    pub fn set_specular(mut self, specular: u32) -> Object {
        self.specular = specular;
        self
    }

    pub fn set_reflection(mut self, reflection: f32) -> Object {
        self.reflection = reflection;
        self
    }

    pub fn set_color(mut self, color: [f32; 3]) -> Object {
        self.color = color;
        self
    }

    pub fn new(shape: Box<dyn Shape>) -> Object {
        Object {
            position: Vec3::new_default(),
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
        let start = start - &self.position;
        let start = &self.rotation * start;
        let start = start / &self.size;

        let ray = &self.rotation * ray;
        let ray = ray / &self.size;
        match self.shape.intersects(&start, &ray) {
            None => None,
            Some(intersection) => {
                Some((IntersectionResult::new(intersection.distance,
                                              intersection.max_distance,
                                              intersection.norm * &self.rotation),
                    self))
            }
        }
    }
}
