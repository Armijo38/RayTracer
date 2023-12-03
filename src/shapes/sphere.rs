use super::shape::Shape;
use super::shape::IntersectionResult;
use crate::vec::Vec3;


pub struct Sphere {
}

impl Shape for Sphere {
    fn intersects(&self, start: &Vec3, ray: &Vec3) -> Option<IntersectionResult> {
        let oc = start; // - 0.0,0.0,0.0 (start pos)
        let k1 = ray.dot(ray);
        let k2 = 2. * &oc.dot(&ray);
        let radius = 0.5;
        let k3 = &oc.dot(&oc) - radius * radius;
        let d = k2 * k2 - 4. * k1 * k3;
        if d < 0.0 {
            return None;
        } else {
            let t1 = (-k2 + d.sqrt()) / (2.0 * k1);
            let t2 = (-k2 - d.sqrt()) / (2.0 * k1);

            let min_t = f32::min(t1, t2);
            let max_t = f32::max(t1, t2);
            let point = start + ray * min_t;
            let norm = point.norm();
            return Some(IntersectionResult::new(min_t, max_t, norm));
        }
    }
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere{}
    }
}
