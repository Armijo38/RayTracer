use super::shape::Shape;
use crate::vec::Vec3;


pub struct Sphere {
    radius: i32,
}

impl Shape for Sphere {
    fn intersects(&self, ray: &Vec3) -> bool {
        return ray.x().powf(2.0) + ray.y().powf(2.0) <= (self.radius.pow(2) as f32);
    }
}

impl Sphere {
    fn new(radius: i32) -> Sphere {
        Sphere{radius}
    }
}
