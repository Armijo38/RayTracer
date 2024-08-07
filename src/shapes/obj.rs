use super::shape::Shape;
use super::cube::Cube;
use super::shape::IntersectionResult;
use crate::object::Object;
use crate::vec::Vec3;

use tobj;
use image::io::Reader as ImageReader;
use image::Rgb32FImage;
use serde::{Serialize,Deserialize};
use derivative::Derivative;
use std::cmp::{max,min};
use std::sync::Arc;


#[derive(Debug,Clone)]
struct Triangle {
    pub edge1: Vec3,
    pub edge2: Vec3,
    pub v0: Vec3,
    pub norm: Vec3,

    pub tex_v0: Vec3,
    pub tex_v1: Vec3,
    pub tex_v2: Vec3,
}

impl Triangle {
    pub fn new(v0: &Vec3, v1: &Vec3, v2: &Vec3,
               tex_v0: Vec3, tex_v1: Vec3, tex_v2: Vec3) -> Triangle {
        Triangle {
            edge1: v1 - v0,
            edge2: v2 - v0,
            v0: v0.clone(),
            norm: (v1 - v0).cross(&(v2 - v0)),
            tex_v0: tex_v0,
            tex_v1: tex_v1,
            tex_v2: tex_v2,
        }
    }

    pub fn min_x(&self) -> f32 {
        self.v0.x().min((&self.edge1 + &self.v0).x()).min((&self.edge2 + &self.v0).x())
    }

    pub fn max_x(&self) -> f32 {
        self.v0.x().max((&self.edge1 + &self.v0).x()).max((&self.edge2 + &self.v0).x())
    }

    pub fn min_y(&self) -> f32 {
        self.v0.y().min((&self.edge1 + &self.v0).y()).min((&self.edge2 + &self.v0).y())
    }

    pub fn max_y(&self) -> f32 {
        self.v0.y().max((&self.edge1 + &self.v0).y()).max((&self.edge2 + &self.v0).y())
    }

    pub fn min_z(&self) -> f32 {
        self.v0.z().min((&self.edge1 + &self.v0).z()).min((&self.edge2 + &self.v0).z())
    }

    pub fn max_z(&self) -> f32 {
        self.v0.z().max((&self.edge1 + &self.v0).z()).max((&self.edge2 + &self.v0).z())
    }
}

#[derive(Debug,Serialize,Deserialize)]
enum Split {
    Leaf {
        #[serde(skip_serializing,skip_deserializing)]
        triangles: Vec<Triangle>,
        #[serde(skip_serializing,skip_deserializing)]
        texture: Option<Arc<Rgb32FImage>>,
    },
    Node {
            left: Box<Split>,
            left_box: Cube, 
            right: Box<Split>,
            right_box: Cube, 
         }
}

impl Default for Split {
    fn default() -> Self {
        Split::Leaf{triangles: vec![], texture: None}
    }
}

impl Split {
    fn new_int(triangles: Vec<Triangle>, texture: Arc<Rgb32FImage>, skip_x: bool, skip_y: bool, skip_z: bool) -> Split {
        // TODO: const
        let leaf_limit = 256;
        if triangles.len() < leaf_limit {
            Split::Leaf{triangles: triangles, texture: Some(texture)}
        } else {
            let mut max_x: f32 = triangles[0].max_x();
            let mut max_y: f32 = triangles[0].max_y();
            let mut max_z: f32 = triangles[0].max_z();

            let mut min_x: f32 = triangles[0].min_x();
            let mut min_y: f32 = triangles[0].min_y();
            let mut min_z: f32 = triangles[0].min_z();

            for triangle in &triangles {
                max_x = max_x.max(triangle.max_x());
                max_y = max_z.max(triangle.max_y());
                max_z = max_y.max(triangle.max_z());

                min_x = min_x.min(triangle.min_x());
                min_y = min_y.min(triangle.min_y());
                min_z = min_z.min(triangle.min_z());
            }

            let dist_x = max_x - min_x;
            let dist_y = max_y - min_y;
            let dist_z = max_z - min_z;

            let triangles_len = triangles.len();

            if !skip_x && dist_x >= dist_y && dist_x >= dist_z {
                let mean_x = min_x + dist_x / 2.0;
                let left_cube = Cube::new(&Vec3::new(min_x, min_y, min_z), &Vec3::new(mean_x, max_y, max_z));
                let right_cube = Cube::new(&Vec3::new(mean_x, min_y, min_z), &Vec3::new(max_x, max_y, max_z));
                let mut left_triangles: Vec<Triangle> = vec![];
                let mut right_triangles: Vec<Triangle> = vec![];
                for triangle in triangles {
                    if triangle.max_x() < mean_x {
                        left_triangles.push(triangle);
                    } else if triangle.min_x() > mean_x {
                        right_triangles.push(triangle);
                    } else {
                        left_triangles.push(triangle.clone());
                        right_triangles.push(triangle);
                    }
                }
                //println!("{} {} {}", min_x, max_x, mean_x);
                //println!("{} {} x", left_triangles.len(), right_triangles.len());
                if left_triangles.len() == triangles_len {
                    Split::new_int(left_triangles, texture, true, skip_y, skip_z)
                } else if right_triangles.len() == triangles_len {
                    Split::new_int(right_triangles, texture, true, skip_y, skip_z)
                } else {
                    Split::Node{left: Box::new(Split::new(left_triangles, texture.clone())),
                                left_box: left_cube,
                                right: Box::new(Split::new(right_triangles, texture)),
                                right_box: right_cube}
                }
            } else if !skip_y && dist_y >= dist_x && dist_y >= dist_z {
                let mean_y = min_y + dist_y / 2.0;
                let left_cube = Cube::new(&Vec3::new(min_x, min_y, min_z), &Vec3::new(max_x, mean_y, max_z));
                let right_cube = Cube::new(&Vec3::new(min_x, mean_y, min_z), &Vec3::new(max_x, max_y, max_z));
                let mut left_triangles: Vec<Triangle> = vec![];
                let mut right_triangles: Vec<Triangle> = vec![];
                for triangle in triangles {
                    if triangle.max_y() < mean_y {
                        left_triangles.push(triangle);
                    } else if triangle.min_y() > mean_y {
                        right_triangles.push(triangle);
                    } else {
                        left_triangles.push(triangle.clone());
                        right_triangles.push(triangle);
                    }
                }
                //println!("{} {} y", left_triangles.len(), right_triangles.len());
                if left_triangles.len() == triangles_len {
                    Split::new_int(left_triangles, texture, skip_x, true, skip_z)
                } else if right_triangles.len() == triangles_len {
                    Split::new_int(right_triangles, texture, skip_x, true, skip_z)
                } else {
                    Split::Node{left: Box::new(Split::new(left_triangles, texture.clone())),
                                left_box: left_cube,
                                right: Box::new(Split::new(right_triangles, texture)),
                                right_box: right_cube}
                }
            } else if !skip_z {
                let mean_z = min_z + dist_z / 2.0;
                let left_cube = Cube::new(&Vec3::new(min_x, min_y, min_z), &Vec3::new(max_x, max_y, mean_z));
                let right_cube = Cube::new(&Vec3::new(min_x, min_y, mean_z), &Vec3::new(max_x, max_y, max_z));
                let mut left_triangles: Vec<Triangle> = vec![];
                let mut right_triangles: Vec<Triangle> = vec![];
                //println!("{}", triangles.len());
                for triangle in triangles {
                    if triangle.max_z() < mean_z {
                        left_triangles.push(triangle);
                    } else if triangle.min_z() > mean_z {
                        right_triangles.push(triangle);
                    } else {
                        left_triangles.push(triangle.clone());
                        right_triangles.push(triangle);
                    }
                }
                //println!("{} {} {}", min_z, max_z, mean_z);
                //println!("{} {} z", left_triangles.len(), right_triangles.len());
                if left_triangles.len() == triangles_len {
                    Split::new_int(left_triangles, texture, skip_x, skip_y, true)
                } else if right_triangles.len() == triangles_len {
                    Split::new_int(right_triangles, texture, skip_x, skip_y, true)
                } else {
                    Split::Node{left: Box::new(Split::new(left_triangles, texture.clone())),
                                left_box: left_cube,
                                right: Box::new(Split::new(right_triangles, texture)),
                                right_box: right_cube}
                }
            } else {
                Split::Leaf{triangles: triangles, texture: Some(texture)}
            }
        }
    }

    fn new(triangles: Vec<Triangle>, texture: Arc<Rgb32FImage>) -> Split {
        Split::new_int(triangles, texture, false, false, false)
    }

    fn triangles_intersects(triangles: &Vec<Triangle>, texture: &Rgb32FImage, start: &Vec3, ray: &Vec3) -> Option<IntersectionResult> {
        let mut result: Option<IntersectionResult> = None;
        for triangle in triangles {
            result = match Split::triangle_intersects(triangle, texture, start, ray) {
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

    fn triangle_intersects(triangle: &Triangle, texture: &Rgb32FImage, start: &Vec3, ray: &Vec3) -> Option<IntersectionResult> {
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
        let u = u * inv_det;
        let v = v * inv_det;

        let tmp_norm = &triangle.norm;
        let norm = if tmp_norm.dot(&ray) > 0.0 {
            -tmp_norm.clone()
        } else {
            tmp_norm.clone()
        };

        let tex_pos = (1.0 - u - v) * &triangle.tex_v0 + u * &triangle.tex_v1 + v * &triangle.tex_v2;
        let color = texture.get_pixel((texture.width() as f32 * tex_pos.x()) as u32,
                                      (texture.height() as f32 * tex_pos.y()) as u32);
        let color = [color[0], color[1], color[2]];
        Some(IntersectionResult::new(t, t, norm).set_color(color))
    }
}

#[typetag::serde(name="split")]
impl Shape for Split {
    fn intersects(&self, start: &Vec3, ray: &Vec3) -> Option<IntersectionResult> {
        match self {
            Split::Leaf{triangles, texture} => {
                Split::triangles_intersects(triangles, &texture.as_ref().unwrap(), start, ray)
            },
            Split::Node{left, left_box, right, right_box} => {
                let left_intersect: Option<IntersectionResult> = left_box.intersects(start, ray);
                let right_intersect: Option<IntersectionResult> = right_box.intersects(start, ray);
                match left_intersect {
                    None => match right_intersect {
                        None => None,
                        Some(_) => right.intersects(start, ray)
                    },
                    Some(left_result) => match right_intersect {
                        None => left.intersects(start, ray),
                        Some(right_result) => {
                            let (first, second) = if left_result.distance < right_result.distance {
                                (&left, &right)
                            } else {
                                (&right, &left)
                            };
                            match first.intersects(start, ray) {
                                None => second.intersects(start, ray),
                                Some(v) => Some(v)
                            }
                        }
                    }
                }
            }
        }
    }

    fn init(&mut self) {
    }
}

#[derive(Serialize,Deserialize,Default,Derivative,Debug)]
pub struct Obj {
    #[serde(skip_serializing,skip_deserializing)]
    #[derivative(Debug="ignore")]
    bbox: Object,
    //triangles: Vec<[Vec3; 3]>
    /*
    #[serde(skip_serializing,skip_deserializing)]
    #[derivative(Debug="ignore")]
    triangles: Vec<Triangle>,
    */
    #[serde(skip_serializing,skip_deserializing)]
    split: Split,

    #[serde(skip_serializing,skip_deserializing)]
    #[derivative(Debug="ignore")]
    texture: Arc<Rgb32FImage>,

    filepath: String,
    texture_path: String,
}

impl Obj {
    /*
    fn triangles_intersects(&self, start: &Vec3, ray: &Vec3) -> Option<IntersectionResult> {
        let mut result: Option<IntersectionResult> = None;
        for triangle in &self.triangles {
            result = match self.triangle_intersects(triangle, start, ray) {
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

    fn triangle_intersects(&self, triangle: &Triangle, start: &Vec3, ray: &Vec3) -> Option<IntersectionResult> {
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
        let u = u * inv_det;
        let v = v * inv_det;

        let tmp_norm = &triangle.norm;
        let norm = if tmp_norm.dot(&ray) > 0.0 {
            -tmp_norm.clone()
        } else {
            tmp_norm.clone()
        };

        let tex_pos = (1.0 - u - v) * &triangle.tex_v0 + u * &triangle.tex_v1 + v * &triangle.tex_v2;
        let color = self.texture.get_pixel((self.texture.width() as f32 * tex_pos.x()) as u32,
                                           (self.texture.height() as f32 * tex_pos.y()) as u32);
        let color = [color[0], color[1], color[2]];
        Some(IntersectionResult::new(t, t, norm).set_color(color))
    }
*/
}

#[typetag::serde(name="object")]
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
            //Some(_) => self.triangles_intersects(start, ray)
            Some(_) => self.split.intersects(start, ray)
        }
    }

    fn init(&mut self) {
        let load_opts = tobj::LoadOptions{
            //merge_identical_points: false,
            //reorder_data: false,
            single_index: true,
            triangulate: true,
            ignore_points: false,
            ignore_lines: false,
        };
        let (models, _materials) = tobj::load_obj(&self.filepath, &load_opts).expect(&format!("Failed to load obj file {}", self.filepath));
        self.texture = Arc::new(ImageReader::open(&self.texture_path).expect(&format!("Failed to open file {}", self.texture_path))
                                                    .decode().expect(&format!("Failed to decoded file {}", self.texture_path)).into_rgb32f());
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

        let mut index_loader = |index: usize, mesh: &tobj::Mesh| -> (Vec3, Vec3) {
            let x = mesh.positions[index * 3];
            let y = mesh.positions[index * 3 + 1];
            let z = mesh.positions[index * 3 + 2];

            let mut u: f32 = 0.0;
            let mut v: f32 = 0.0;
            if index * 2 + 1 < mesh.texcoords.len() {
                u = mesh.texcoords[index * 2].fract();
                if u < 0.0 {
                    u = 1.0 + u;
                }

                v = mesh.texcoords[index * 2 + 1].fract();
                if v < 0.0 {
                    v = 1.0 + v;
                }
            }

            assign_min(&mut min_x, x);
            assign_min(&mut min_y, y);
            assign_min(&mut min_z, z);

            assign_max(&mut max_x, x);
            assign_max(&mut max_y, y);
            assign_max(&mut max_z, z);

            (Vec3::new(x, y, z), Vec3::new(u, v, 0.0))
        };

        println!("There are {} meshes", models.len());
        let mut triangles: Vec<[Vec3; 6]> = Vec::new();
        for model in models {
            let mesh = &model.mesh;
            println!("{} triangles", mesh.indices.len() / 3);

            for i in 0 .. mesh.indices.len() / 3 {
                let i1 = mesh.indices[i * 3];
                let i2 = mesh.indices[i * 3 + 1];
                let i3 = mesh.indices[i * 3 + 2];

                let (v1, tex_v1) = index_loader(i1 as usize, mesh);
                let (v2, tex_v2) = index_loader(i2 as usize, mesh);
                let (v3, tex_v3) = index_loader(i3 as usize, mesh);
                
                triangles.push([v1, v2, v3, tex_v1, tex_v2, tex_v3]);
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

        let triangles = triangles.iter().map(|v: &[Vec3; 6]| -> Triangle {
            let v1 = &v[0] * scale - &xyz_move;
            let v2 = &v[1] * scale - &xyz_move;
            let v3 = &v[2] * scale - &xyz_move;
            let tex_v1 = &v[3];
            let tex_v2 = &v[4];
            let tex_v3 = &v[5];
            Triangle::new(&v1, &v2, &v3, tex_v1.clone(), tex_v2.clone(), tex_v3.clone())
        }).collect();

        self.bbox = Object::new(Box::new(Cube::default())).set_size(Vec3::new(scale / x_scale, scale / y_scale, scale / z_scale));
        self.split = Split::new(triangles, self.texture.clone());
    }
}
