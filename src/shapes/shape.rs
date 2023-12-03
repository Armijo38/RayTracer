use crate::vec::Vec3;

#[derive(Clone)]
pub struct IntersectionResult {
    pub distance: f32,
    pub max_distance: f32,
    pub norm: Vec3,
}


impl IntersectionResult {
    pub fn new(distance: f32, max_distance: f32, norm: Vec3) -> IntersectionResult {
        IntersectionResult{distance, max_distance, norm}
    }
}


pub trait Shape {
    fn intersects(&self, start: &Vec3, ray: &Vec3) -> Option<IntersectionResult>;
}
