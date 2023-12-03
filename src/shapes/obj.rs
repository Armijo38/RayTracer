use super::shape::Shape;
use super::cube::Cube;
use super::shape::IntersectionResult;
use crate::object::Object;
use crate::vec::Vec3;

use tobj;


struct Triangle {
    pub edge1: Vec3,
    pub edge2: Vec3,
    pub v0: Vec3,
    pub norm: Vec3,
}

impl Triangle {
    pub fn new(v0: &Vec3, v1: &Vec3, v2: &Vec3) -> Triangle {
        Triangle {
            edge1: v1 - v0,
            edge2: v2 - v0,
            v0: v0.clone(),
            norm: (v1 - v0).cross(&(v2 - v0))
        }
    }
}

pub struct Obj {
    bbox: Object,
    //triangles: Vec<[Vec3; 3]>
    triangles: Vec<Triangle>
}

impl Obj {
    pub fn new(filepath: &str) -> Obj {
        let load_opts = tobj::LoadOptions{
            //merge_identical_points: false,
            //reorder_data: false,
            single_index: true,
            triangulate: true,
            ignore_points: false,
            ignore_lines: false,
        };
        let (models, materials) = tobj::load_obj(filepath, &load_opts).expect(&format!("Failed to load obj file {}", filepath));
        // TODO: materials

        let mut min_x: Option<f32> = None;
        let mut min_y: Option<f32> = None;
        let mut min_z: Option<f32> = None;

        let mut max_x: Option<f32> = None;
        let mut max_y: Option<f32> = None;
        let mut max_z: Option<f32> = None;

        let assign_min = |m: &mut Option<f32>, value: f32| {
            *m = match m {
                None => Some(value),
                Some(x) => if *x < value {
                    Some(*x)
                } else {
                    Some(value)
                }
            };
        };

        let assign_max = |m: &mut Option<f32>, value: f32| {
            *m = match m {
                None => Some(value),
                Some(x) => if *x > value {
                    Some(*x)
                } else {
                    Some(value)
                }
            };
        };

        let mut index_loader = |index: usize, mesh: &tobj::Mesh| -> Vec3 {
            let x = mesh.positions[index * 3];
            let y = mesh.positions[index * 3 + 1];
            let z = mesh.positions[index * 3 + 2];

            assign_min(&mut min_x, x);
            assign_min(&mut min_y, y);
            assign_min(&mut min_z, z);

            assign_max(&mut max_x, x);
            assign_max(&mut max_y, y);
            assign_max(&mut max_z, z);

            Vec3::new(x, y, z)
        };

        let mut triangles: Vec<[Vec3; 3]> = Vec::new();
        println!("There are {} meshes", models.len());
        for model in models {
            let mesh = &model.mesh;
            println!("{} triangles", mesh.indices.len() / 3);

            for i in 0 .. mesh.indices.len() / 3 {
                let i1 = mesh.indices[i * 3];
                let i2 = mesh.indices[i * 3 + 1];
                let i3 = mesh.indices[i * 3 + 2];

                let v1 = index_loader(i1 as usize, mesh);
                let v2 = index_loader(i2 as usize, mesh);
                let v3 = index_loader(i3 as usize, mesh);
                
                triangles.push([v1, v2, v3]);
            }
        }

        let x_scale = 1.0 / (max_x.unwrap() - min_x.unwrap());
        let y_scale = 1.0 / (max_y.unwrap() - min_y.unwrap());
        let z_scale = 1.0 / (max_z.unwrap() - min_z.unwrap());

        let scale = f32::min(f32::min(x_scale, y_scale), z_scale);

        let x_move = (max_x.unwrap() + min_x.unwrap()) / 2.0 * scale;
        let y_move = (max_y.unwrap() + min_y.unwrap()) / 2.0 * scale;
        let z_move = (max_z.unwrap() + min_z.unwrap()) / 2.0 * scale;

        let xyz_move = Vec3::new(x_move, y_move, z_move);

        let triangles = triangles.iter().map(|v: &[Vec3; 3]| -> Triangle {
            let v1 = &v[0] * scale - &xyz_move;
            let v2 = &v[1] * scale - &xyz_move;
            let v3 = &v[2] * scale - &xyz_move;
            Triangle::new(&v1, &v2, &v3)
        }).collect();

        Obj {
            triangles,
            bbox: Object::new(Box::new(Cube::new())).set_size(Vec3::new(scale / x_scale, scale / y_scale, scale / z_scale))
        }
    }

    fn triangles_intersects(&self, start: &Vec3, ray: &Vec3) -> Option<IntersectionResult> {
        let mut result: Option<IntersectionResult> = None;
        for triangle in &self.triangles {
            result = match Obj::triangle_intersects(triangle, start, ray) {
                None => result,
                Some(intersection) => {
                    match result {
                        None => Some(intersection),
                        Some(best_intersection) => if intersection.distance < best_intersection.distance {
                            Some(intersection)
                        } else {
                            Some(best_intersection)
                        }
                    }
                }
            }
        }
        result
    }

    fn triangle_intersects(triangle: &Triangle, start: &Vec3, ray: &Vec3) -> Option<IntersectionResult> {
        let pvec = ray.cross(&triangle.edge2);

        let det = triangle.edge1.dot(&pvec);
        
        /*
        if det < 1e-7 && det > -1e-7 {
            return None
        }
        */


        let tvec = start - &triangle.v0;
        let u = tvec.dot(&pvec);
        if u < 0.0 || u > det {
            return None;
        }

        let qvec = tvec.cross(&triangle.edge1);
        let v = ray.dot(&qvec);
        if v < 0.0 || u + v > det {
            return None;
        }

        let t = triangle.edge2.dot(&qvec);
        let inv_det = 1.0 / det;
        let t = t * inv_det;
        //let u = u * inv_det;
        //let v = v * inv_det;

        let tmp_norm = &triangle.norm;
        let norm = if tmp_norm.dot(&ray) > 0.0 {
            -tmp_norm.clone()
        } else {
            tmp_norm.clone()
        };
        Some(IntersectionResult::new(t, t, norm))
    }
}

impl Shape for Obj {
    fn intersects(&self, start: &Vec3, ray: &Vec3) -> Option<IntersectionResult> {
        /*
        match self.bbox.intersects(start, ray) {
            None => None,
            Some((intersection, _)) => Some(intersection)
        }
        */
        match self.bbox.intersects(start, ray) {
            None => None,
            Some(_) => self.triangles_intersects(start, ray)
        }
    }
}
