use super::shape::Shape;
use super::shape::IntersectionResult;
use crate::vec::Vec3;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Plane {
    //#[serde(default)]
    //point: Vec3,
    //#[serde(default)]
    //direction1: Vec3,
    //#[serde(default)]
    //direction2: Vec3,
    #[serde(default)]
    norm: Vec3,
    #[serde(default)]
    d: f32,
    //#[serde(default)]
    //point2: Vec3,
    #[serde(default)]
    min_point: Vec3,
    #[serde(default)]
    max_point: Vec3,
}

#[typetag::serde(name="plane")]
impl Shape for Plane {
    fn intersects(&self, start: &Vec3, ray: &Vec3) -> Option<IntersectionResult> {
        let c = ray.dot(&self.norm);
        if c == 0.0 {
            return None;
        }
        let t = -(self.norm.dot(start) + self.d) / c;
        if t < 0.0 {
            return None;
        }

        let point = start + ray * t;

        //let point2 = &self.point + &self.direction1 + &self.direction2;
        let eps = 1e-5;
        /*
        if point.x() < f32::min(self.point.x(), point2.x()) - eps
            || point.x() > f32::max(self.point.x(), point2.x()) + eps
            || point.y() < f32::min(self.point.y(), point2.y()) - eps
            || point.y() > f32::max(self.point.y(), point2.y()) + eps
            || point.z() < f32::min(self.point.z(), point2.z()) - eps
            || point.z() > f32::max(self.point.z(), point2.z()) + eps {
        */
        if point.x() < self.min_point.x()
            || point.x() > self.max_point.x()
            || point.y() < self.min_point.y()
            || point.y() > self.max_point.y()
            || point.z() < self.min_point.z()
            || point.z() > self.max_point.z()
        {
                return None;
        }
        Some(IntersectionResult::new(t, t, self.norm.clone()))
    }
}

impl Plane {
    #[allow(dead_code)]
    pub fn new(point: Vec3, direction1: Vec3, direction2: Vec3) -> Plane {
        let norm = direction1.cross(&direction2).norm();
        let d = -norm.x() * point.x()
                -norm.y() * point.y()
                -norm.z() * point.z();
        let point2 = &point + &direction1 + &direction2;
        let eps = 1e-5;
        let min_point: Vec3 = Vec3::new(f32::min(point.x(), point2.x() - eps),
                                        f32::min(point.y(), point2.y() - eps),
                                        f32::min(point.z(), point2.z() - eps));
        let max_point: Vec3 = Vec3::new(f32::max(point.x(), point2.x() + eps),
                                        f32::max(point.y(), point2.y() + eps),
                                        f32::max(point.z(), point2.z() + eps));
        //Plane{point, direction1, direction2, norm, d, point2, min_point, max_point}
        Plane{norm, d, min_point, max_point}
    }
}

impl Default for Plane {
    fn default() -> Plane {
        Plane::new(Vec3::new(0.5, -0.5, 0.0),
                   Vec3::new(-1.0, 0.0, 0.0),
                   Vec3::new(0.0, 1.0, 0.0))
    }
}
