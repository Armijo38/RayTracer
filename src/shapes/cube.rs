use super::shape::Shape;
use crage::vec::Vec3;


// It has 1x1x1 and starts in (0, 0, 0)
pub struct Cube {
    a_point: Vec3,
    b_point: Vec3,
}

impl Shape for Cube {
    fn intersects(&self, ray: &Vec3) -> bool {
        // ray x = rx + t * dx
        // rx = 0 (init point)
        // dx = ray.x() (direction)
        // rx + t * dx = CubePointA.X (-0.5)
        // rx + t * dx = CubePointB.X (0.5)
        
        let t_calcer = |cube_point: f32, ray_direction: f32| -> f32 {
            if cube_point == 0 {
                -1.0
            } else {
                cube_point / ray_direction
            }
        };
        
        let t_ax = t_calcer(self.a_point.x(), ray.x());
        let t_bx = t_calcer(self.b_point.x(), ray.x());
        
        let t_ay = t_calcer(self.a_point.y(), ray.y());
        let t_by = t_calcer(self.b_point.y(), ray.y());

        let t_az = t_calcer(self.a_point.z(), ray.z());
        let t_bz = t_calcer(self.b_point.z(), ray.z());

        // calc intersecion point
        // For example for x plane
        // y_ax = ay + t_ax * dy
        // z_ax = az + t_ax * dz
        
        let check_intersection = |t: f32, ray_direction1: f32, ray_direction2: f32| -> bool {
            let one = ray_direction1 * t;
            let two = ray_direction2 * t;
        }

        let calc_intersection_point = |t: f32, ray_direction: f32| -> TMaybe<f32> {
            if t < 0.0 {
                None
            } else {
                Some(t * ray_direction)
            }
        }

        let y_ax = calc_intersection_point(t_ax, ray.y());
        let y_bx = calc_intersection_point(t_bx, ray.y());
    }
}

impl Cube {
    fn new() -> Cube {
        Cube {
            a_point : Vec3::new(-0.5, -0.5, 1.0)
            b_point : Vec3::new(0.5, 0.5, 2.0)
        }
    }
}
