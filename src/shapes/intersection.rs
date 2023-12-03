use super::shape::{Shape,IntersectionResult};
use crate::object::Object;
use crate::vec::Vec3;

pub struct Intersection {
    shape1: Object,
    shape2: Object,
}

impl Shape for Intersection {
    fn intersects(&self, start: &Vec3, ray: &Vec3) -> Option<IntersectionResult> {
        match self.shape1.intersects(start, ray) {
            None => None,
            Some((intersection1, _object1)) => {
                match self.shape2.intersects(start, ray) {
                    None => None,
                    Some((intersection2, _object2)) => {
                        if intersection1.distance < intersection2.distance {
                            if intersection1.max_distance < intersection2.distance {
                                None
                            } else {
                                Some(IntersectionResult::new(intersection2.distance,
                                                             intersection1.max_distance,
                                                             intersection2.norm))
                            }
                        } else {
                            if intersection2.max_distance < intersection1.distance {
                                None
                            } else {
                                Some(IntersectionResult::new(intersection1.distance,
                                                             intersection2.max_distance,
                                                             intersection1.norm))
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Intersection {
    pub fn new(shape1: Object, shape2: Object) -> Intersection {
        Intersection {
            shape1,
            shape2,
        }
    }
}
