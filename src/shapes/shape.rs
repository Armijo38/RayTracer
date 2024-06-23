use crate::vec::Vec3;
use serde::{Serialize,Deserialize};

#[derive(Clone)]
pub struct IntersectionResult {
    pub distance: f32,
    pub max_distance: f32,
    pub norm: Vec3,
    pub color: Option<[f32; 3]>,
}


impl IntersectionResult {
    pub fn new(distance: f32, max_distance: f32, norm: Vec3) -> IntersectionResult {
        IntersectionResult{distance, max_distance, norm, color: None}
    }

    pub fn set_color(mut self, color: [f32; 3]) -> IntersectionResult {
        self.color = Some(color);
        self
    }
}


#[typetag::serde(tag="type")]
pub trait Shape: std::fmt::Debug + Sync + Send {
    fn intersects(&self, start: &Vec3, ray: &Vec3) -> Option<IntersectionResult>;
    fn init(&mut self) {
        // empty
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct NoneShape {
}

impl NoneShape {
    pub fn new() -> NoneShape {
        NoneShape{}
    }
}

#[typetag::serde(name="none")]
impl Shape for NoneShape {
    fn intersects(&self, _start: &Vec3, _ray: &Vec3) -> Option<IntersectionResult> {
        None
    }
}
