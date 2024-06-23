use super::shape::{Shape,IntersectionResult};
use crate::object::Object;
use crate::vec::Vec3;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Difference {
    shape1: Object,
    shape2: Object,
}

#[typetag::serde(name="difference")]
impl Shape for Difference {
    fn intersects(&self, start: &Vec3, ray: &Vec3) -> Option<IntersectionResult> {
        match self.shape1.intersects(start, ray) {
            None => None,
            Some((intersection1, _object1)) => {
                match self.shape2.intersects(start, ray) {
                    None => Some(intersection1),
                    Some((intersection2, _object2)) => {
                        if intersection1.distance < intersection2.distance {
                            Some(IntersectionResult::new(intersection1.distance,
                                                         f32::min(intersection1.max_distance,
                                                                  intersection2.distance),
                                                         intersection1.norm))
                        } else {
                            if intersection2.max_distance < intersection1.distance {
                                Some(intersection1)
                            } else {
                                Some(IntersectionResult::new(intersection2.max_distance,
                                                             intersection1.max_distance,
                                                             intersection2.norm))
                            }
                        }
                    }
                }
            }
        }
    }

    fn init(&mut self) {
        self.shape1.init();
        self.shape2.init();
    }
}

impl Difference {
    #[allow(dead_code)]
    pub fn new(shape1: Object, shape2: Object) -> Difference {
        Difference {
            shape1,
            shape2,
        }
    }
}
