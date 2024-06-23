use super::shape::Shape;
use super::shape::IntersectionResult;
use super::plane::Plane;
use crate::vec::Vec3;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Cube {
    //point1: Vec3,
    //point2: Vec3,

    #[serde(skip_serializing,skip_deserializing)]
    planes: [Plane; 6],
}

#[typetag::serde(name="cube")]
impl Shape for Cube {
    fn init(&mut self) {
        let point1 = Vec3::new(-0.5, -0.5, -0.5);
        let point2 = Vec3::new(0.5, 0.5, 0.5);

        self.planes = [
            Plane::new(Vec3::new(point1.x(), point2.y(), point2.z()),
                       Vec3::new(0.0, 0.0, point1.z() - point2.z()),
                       Vec3::new(0.0, point1.y() - point2.y(), 0.0)), // left
            Plane::new(Vec3::new(point2.x(), point2.y(), point1.z()),
                       Vec3::new(0.0, 0.0, point2.z() - point1.z()),
                       Vec3::new(0.0, point1.y() - point2.y(), 0.0)), // right
            Plane::new(Vec3::new(point2.x(), point2.y(), point1.z()),
                       Vec3::new(0.0, point1.y() - point2.y(), 0.0),
                       Vec3::new(point1.x() - point2.x(), 0.0, 0.0)), // front
            Plane::new(Vec3::new(point2.x(), point2.y(), point2.z()),
                       Vec3::new(point1.x() - point2.x(), 0.0, 0.0),
                       Vec3::new(0.0, point1.y() - point2.y(), 0.0)), //back
            Plane::new(Vec3::new(point2.x(), point1.y(), point2.z()),
                       Vec3::new(point1.x() - point2.x(), 0.0, 0.0),
                       Vec3::new(0.0, 0.0, point1.z() - point2.z())), // up
            Plane::new(Vec3::new(point2.x(), point2.y(), point1.z()),
                       Vec3::new(point1.x() - point2.x(), 0.0, 0.0),
                       Vec3::new(0.0, 0.0, point2.z() - point1.z())), // down
        ]
    }

    fn intersects(&self, start: &Vec3, ray: &Vec3) -> Option<IntersectionResult> {
        let mut result: Option<IntersectionResult> = None;
        for plane in &self.planes {
            result = match plane.intersects(start, ray) {
                None => result,
                Some(mut intersection) => {
                    match result {
                        None => Some(intersection),
                        Some(mut best_intersection) => {
                            if intersection.distance < best_intersection.distance {
                                intersection.max_distance = best_intersection.distance;
                                Some(intersection)
                            } else {
                                best_intersection.max_distance = intersection.distance;
                                Some(best_intersection)
                            }
                        }
                    }
                }
            };
        }

        return result;
    }
}

impl Cube {
    #[allow(dead_code)]
    pub fn new() -> Cube {
        //point1 - left up
        //point2 - right down

        let point1 = Vec3::new(-0.5, -0.5, -0.5);
        let point2 = Vec3::new(0.5, 0.5, 0.5);

        Cube {
            planes: [
                Plane::new(Vec3::new(point1.x(), point2.y(), point2.z()),
                           Vec3::new(0.0, 0.0, point1.z() - point2.z()),
                           Vec3::new(0.0, point1.y() - point2.y(), 0.0)), // left
                Plane::new(Vec3::new(point2.x(), point2.y(), point1.z()),
                           Vec3::new(0.0, 0.0, point2.z() - point1.z()),
                           Vec3::new(0.0, point1.y() - point2.y(), 0.0)), // right
                Plane::new(Vec3::new(point2.x(), point2.y(), point1.z()),
                           Vec3::new(0.0, point1.y() - point2.y(), 0.0),
                           Vec3::new(point1.x() - point2.x(), 0.0, 0.0)), // front
                Plane::new(Vec3::new(point2.x(), point2.y(), point2.z()),
                           Vec3::new(point1.x() - point2.x(), 0.0, 0.0),
                           Vec3::new(0.0, point1.y() - point2.y(), 0.0)), //back
                Plane::new(Vec3::new(point2.x(), point1.y(), point2.z()),
                           Vec3::new(point1.x() - point2.x(), 0.0, 0.0),
                           Vec3::new(0.0, 0.0, point1.z() - point2.z())), // up
                Plane::new(Vec3::new(point2.x(), point2.y(), point1.z()),
                           Vec3::new(point1.x() - point2.x(), 0.0, 0.0),
                           Vec3::new(0.0, 0.0, point2.z() - point1.z())), // down
            ]
        }
    }
}
