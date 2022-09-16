use crate::vec::Vec3;


pub trait Shape {
    fn intersects(&self, ray: &Vec3) -> bool;
}
