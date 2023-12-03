use super::shape::Shape;
use super::shape::IntersectionResult;
use crate::vec::Vec3;

pub struct Plane {
    point: Vec3,
    direction1: Vec3,
    direction2: Vec3,
}

impl Shape for Plane {
    fn intersects(&self, start: &Vec3, ray: &Vec3) -> Option<IntersectionResult> {
        let norm = self.direction1.cross(&self.direction2).norm();
        let d = -norm.x() * self.point.x()
                -norm.y() * self.point.y()
                -norm.z() * self.point.z();
        let c = ray.dot(&norm);
        if c == 0.0 {
            return None;
        }
        let t = -(norm.dot(start) + d) / c;
        if t < 0.0 {
            return None;
        }

        let point = start + ray * t;

        let point2 = &self.point + &self.direction1 + &self.direction2;
        let eps = 1e-5;
        if point.x() < f32::min(self.point.x(), point2.x()) - eps
            || point.x() > f32::max(self.point.x(), point2.x()) + eps
            || point.y() < f32::min(self.point.y(), point2.y()) - eps
            || point.y() > f32::max(self.point.y(), point2.y()) + eps
            || point.z() < f32::min(self.point.z(), point2.z()) - eps
            || point.z() > f32::max(self.point.z(), point2.z()) + eps {
                return None;
        }
        Some(IntersectionResult::new(t, t, norm))
    }
}

impl Plane {
    pub fn new(point: Vec3, direction1: Vec3, direction2: Vec3) -> Plane {
        Plane{point, direction1, direction2}
    }

    pub fn new_default() -> Plane {
        Plane::new(Vec3::new(0.5, -0.5, 0.0),
                   Vec3::new(-1.0, 0.0, 0.0),
                   Vec3::new(0.0, 1.0, 0.0))
    }
}
