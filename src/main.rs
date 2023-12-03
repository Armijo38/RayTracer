use image;

mod vec;
mod matrix;
mod shapes {
    pub mod shape;
    pub mod sphere;
    pub mod plane;
    pub mod cube;
    pub mod intersection;
    pub mod difference;
}
mod object;
mod lights;

use vec::Vec3;
use matrix::Matrix33;
use shapes::shape::{Shape,IntersectionResult};
use shapes::sphere::Sphere;
use shapes::plane::Plane;
use shapes::cube::Cube;
use shapes::intersection::Intersection;
use shapes::difference::Difference;
use object::Object;
use lights::{Light,PointLight,AmbientLight};

fn intersect<'a>(start: &Vec3, direction: &Vec3,
                 objects: &'a Vec<Object>,
                 t_min: Option<f32>, t_max: Option<f32>) -> Option<(IntersectionResult, &'a Object)> {
    let mut best_result: Option<(IntersectionResult, &Object)> = None;
    for object in objects {
        best_result = match object.intersects(start, direction) {
            None => best_result,
            Some((intersection, n_object)) => {
                if (t_min.is_none() || intersection.distance >= t_min.unwrap())
                    && (t_max.is_none() || intersection.distance <= t_max.unwrap())
                    && (best_result.is_none() || intersection.distance < best_result.as_ref().unwrap().0.distance) {
                        Some((intersection, n_object))
                } else {
                    best_result
                }
            }
        }
    }
    best_result
}

fn reflect_vec(ray: &Vec3, norm: &Vec3) -> Vec3 {
    norm * 2.0 * norm.dot(ray) - ray
}

fn ray_trace(start: &Vec3, direction: &Vec3,
             objects: &Vec<Object>,
             lights: &Vec<Box<dyn Light>>,
             t_min: Option<f32>, t_max: Option<f32>,
             depth: u16) -> Option<[f32; 3]> {

    if depth == 0 {
        return None;
    }

    let mut intensity = 0.0;
    let best_intersection  = intersect(&start, &direction, &objects, t_min, t_max);

    match best_intersection {
        Some((intersection, object)) => {
            let mut result_color = object.color;
            let point = start + direction * intersection.distance;

            for light in lights {
                let intersects = if let Some(light_point) = light.point() {
                    let to_light = light_point - &point;
                    match intersect(&point, &to_light, &objects, Some(1e-4), Some(1.0)) {
                        Some((light_intersection, _light_object)) => light_intersection.distance < 1.0,
                        None => false
                    }
                } else {
                    false
                };
                if !intersects {
                    intensity += light.intensity(&point, &intersection.norm);
                    if object.specular > 0 {
                        intensity += light.specular(&point, &intersection.norm, &direction, object.specular);
                    }
                }
            }
            result_color[0] = result_color[0] * intensity;
            result_color[1] = result_color[1] * intensity;
            result_color[2] = result_color[2] * intensity;

            let reflection = object.reflection;
            if reflection != 0.0 {
                result_color[0] = result_color[0] * (1.0 - reflection);
                result_color[1] = result_color[1] * (1.0 - reflection);
                result_color[2] = result_color[2] * (1.0 - reflection);

                let reflected = reflect_vec(&(-direction), &intersection.norm.norm()).norm();
                let reflect_color = ray_trace(&point, &reflected, objects, lights, Some(1e-4), None, depth-1);
                if let Some(reflected_color) = reflect_color {
                    result_color[0] += reflected_color[0] * reflection;
                    result_color[1] += reflected_color[1] * reflection;
                    result_color[2] += reflected_color[2] * reflection;
                }
            }
            Some(result_color)
        },
        None => None
    }
}

fn rotate_view(x_phi: f32, y_phi: f32, z_phi: f32, eye: vec::Vec3) -> vec::Vec3 {
    let x_matrix = Matrix33::new([1.0, 0.0, 0.0,
                                  0.0, f32::cos(x_phi), -f32::sin(x_phi),
                                  0.0, f32::sin(x_phi), f32::cos(x_phi)]);
    let y_matrix = Matrix33::new([f32::cos(y_phi), 0.0, f32::sin(y_phi),
                                  0.0, 1.0, 0.0,
                                  -f32::sin(y_phi), 0.0, f32::cos(y_phi)]);
    let z_matrix = Matrix33::new([f32::cos(z_phi), -f32::sin(z_phi), 0.0,
                                  f32::sin(z_phi), f32::cos(z_phi), 0.0,
                                  0.0, 0.0, 1.0]);
    eye * x_matrix * y_matrix * z_matrix
}

fn main() {
    let img_size: (i32, i32) = (512, 512);
    //let img_size: (i32, i32) = (2048, 2048);
    //let img_size: (i32, i32) = (8192, 8192);
    let viewport_size = (1.0, 1.0);
    let z_dist = 1.0;
    let mut img = image::RgbImage::new(img_size.0 as u32, img_size.1 as u32);

    /*
    let objects: Vec<Object> = vec![
        Object::new(Box::new(Sphere::new())).set_position(Vec3::new(-1.0, 1.0, 5.0))
                                            .set_color([0.0, 1.0, 1.0])
                                            .set_size(Vec3::new(0.5, 0.5, 0.5))
                                            .set_specular(100)
                                            .set_reflection(0.5),
        Object::new(Box::new(Sphere::new())).set_position(Vec3::new(-1.0, 1.0, 7.0))
                                            .set_color([1.0, 1.0, 0.0])
                                            .set_size(Vec3::new(1.0, 2.0, 1.0)),
        Object::new(Box::new(Cube::new())).set_position(Vec3::new(1.0, -1.0, 5.0))
                                          .set_color([1.0, 0.0, 0.0])
                                          .set_rotation(Vec3::new(0.0, 0.0, 0.5)),
        Object::new(Box::new(Plane::new_default())).set_position(Vec3::new(1.0, 1.0, 3.0))
                                                   .set_rotation(Vec3::new(0.0, 0.2, 0.0)),
    ];
    */
    let objects: Vec<Object> = vec![
        /*
        Object::new(Box::new(Plane::new_default())).set_position(Vec3::new(-1.0, 0.0, 3.5))
                                                   .set_rotation(0, 90, 0)
                                                   .set_size(Vec3::new(5.0, 2.0, 1.0))
                                                   .set_color([1.0, 1.0, 1.0])
                                                   .set_reflection(0.5)
                                                   .set_specular(10),

        Object::new(Box::new(Plane::new_default())).set_position(Vec3::new(1.0, 0.0, 3.5))
                                                   .set_rotation(0, -90, 0)
                                                   .set_size(Vec3::new(5.0, 2.0, 1.0))
                                                   .set_color([1.0, 1.0, 1.0])
                                                   .set_reflection(0.5)
                                                   .set_specular(10),
        Object::new(Box::new(Plane::new_default())).set_position(Vec3::new(0.0, 0.0, 5.0))
                                                   .set_rotation(0, 0, 0)
                                                   .set_size(Vec3::new(2.0, 2.0, 1.0))
                                                   .set_color([1.0, 1.0, 1.0])
                                                   .set_reflection(0.5)
                                                   .set_specular(10),
        Object::new(Box::new(Sphere::new())).set_position(Vec3::new(-0.5, -0.5, 2.0))
                                            .set_color([1.0, 0.0, 0.0])
                                            .set_specular(1000)
                                            .set_size(Vec3::new(0.35, 0.35, 0.35)),
        Object::new(Box::new(Sphere::new())).set_position(Vec3::new(0.5, 0.5, 2.5))
                                            .set_color([0.0, 0.0, 1.0])
                                            .set_reflection(0.5)
                                            .set_specular(10)
                                            .set_size(Vec3::new(0.35, 0.35, 0.35)),
                                            */
        //Object::new(Box::new(Sphere::new())).set_position(Vec3::new(1.0, 1.0, 3.0)).set_specular(100).set_rotation(0, 0, 0),
        //Object::new(Box::new(Sphere::new())).set_position(Vec3::new(-1.0, -1.0, 3.0))
        /*
        Object::new(Box::new(Cube::new())).set_position(Vec3::new(1.0, 1.0, 4.0))
                                          .set_rotation(0, 45, 0)
                                          .set_size(Vec3::new(1.0, 0.5, 1.0)),
        Object::new(Box::new(Cube::new())).set_position(Vec3::new(1.0, 1.0, 4.5))
                                          .set_size(Vec3::new(1.44, 1.0, 1.0))
                                          .set_color([1.0, 0.0, 0.0]),
        Object::new(Box::new(Cube::new())).set_position(Vec3::new(-1.0, -1.0, 4.0))
                                            //.set_size(Vec3::new(0.3, 0.3, 0.3))

                                            */
        Object::new(Box::new(Intersection::new(Object::new(Box::new(Sphere::new())),
                                               Object::new(Box::new(Sphere::new())).set_size(Vec3::new(0.5, 0.5, 0.5))
                                                                                   .set_position(Vec3::new(0.5, 0.0, 0.0))
                                               ))).set_position(Vec3::new(0.0, 0.0, 4.0))
                                                  .set_size(Vec3::new(0.8, 0.8, 0.8))
                                                  .set_rotation(0, -30, 0),

        Object::new(Box::new(Difference::new(Object::new(Box::new(Sphere::new())),
                                               Object::new(Box::new(Sphere::new())).set_size(Vec3::new(0.5, 0.5, 0.5))
                                                                                   .set_position(Vec3::new(0.5, 0.0, 0.0))
                                               ))).set_position(Vec3::new(0.5, 1.0, 4.0))
                                                  .set_size(Vec3::new(0.8, 0.8, 0.8))
                                                  .set_rotation(0, -70, 0),
        Object::new(Box::new(Plane::new_default())).set_position(Vec3::new(0.5, 1.0, 4.0))
                                                   .set_rotation(0, -70, 0)
                                                   .set_color([0.0, 1.0, 0.0])
        //Object::new(Box::new(Plane::new_default())).set_position(Vec3::new(1.0, -1.5,))
    ];
    /*
    let objects: Vec<Box<dyn Shape>> = vec![
        /*
        Box::new(shapes::plane::Plane::new(vec::Vec3::new(-1.0, 1.0, 1.5),
                                           vec::Vec3::new(0.0, 0.0, 6.0),
                                           vec::Vec3::new(0.0, -2.5, 0.0),
                                           [1.0, 1.0, 1.0], 10, 0.9)),
        Box::new(shapes::plane::Plane::new(vec::Vec3::new(1.0, -1.5, 1.5),
                                           vec::Vec3::new(0.0, 0.0, 6.0),
                                           vec::Vec3::new(0.0, 2.5, 0.0),
                                           [1.0, 1.0, 1.0], 10, 0.9)),
                                           */
        Box::new(Cube::new(Vec3::new(-0.8, -1.0, 3.0),
                           Vec3::new(-0.3, -0.5, 4.0),
                           [0.0, 1.0, 1.0], 1000, 0.5)),
        Box::new(Sphere::new(0.2,
                            Vec3::new(0.25, -0.75, 2.0),
                            [0.5, 0.5, 0.5], 100, 0.3)),
        Box::new(Sphere::new(0.2,
                            Vec3::new(0.5, 0.0, 5.0),
                            [1.0, 0.5, 0.0], 100, 0.4)),
        Box::new(Plane::new(Vec3::new(-1.0, 1.0, 1.5),
                            Vec3::new(0.0, 0.0, 6.0),
                            Vec3::new(0.0, -2.5, 0.0),
                            [1.0, 1.0, 0.5], 10, 0.3)),
    ];
    */

    let lights: Vec<Box<dyn Light>> = vec![
        //Box::new(PointLight::new(Vec3::new(-0.5, -0.5, 1.5), 0.6)),
        Box::new(PointLight::new(Vec3::new(0.0, 0.0, 0.0), 0.6)),
        /*
        Box::new(PointLight::new(Vec3::new(0.5, 1.5, 1.0), 0.2)),
        Box::new(PointLight::new(Vec3::new(0.0, 0.5, 8.0), 0.2)),
        */
        Box::new(AmbientLight::new(0.2)),
    ];

    let depth = 4;

    let start = Vec3::new(0.0, 0.0, 0.0);
    let x_phi = 0.0;
    let y_phi = 0.0;
    let z_phi = 0.0;

    for x in 0..img.width() as i32 {
        for y in 0..img.height() as i32 {
            let eye = Vec3::new(
                viewport_size.0 * ((x - img_size.0/2) as f32) / (img_size.0 as f32),
                viewport_size.1 * ((y - img_size.1/2) as f32) / (img_size.1 as f32),
                z_dist)
                .norm();

            let eye = rotate_view(x_phi, y_phi, z_phi, eye);
            let color = ray_trace(&start, &eye, &objects, &lights, Some(1.0), None, depth);
            match color {
                Some(real_color) => {
                    img.put_pixel(x as u32, y as u32,
                                  image::Rgb([(255.0 * real_color[0]) as u8,
                                              (255.0 * real_color[1]) as u8,
                                              (255.0 * real_color[2]) as u8]))
                },
                None => {
                    img.put_pixel(x as u32, y as u32,
                                  image::Rgb([0, 0, 0]));
                }
            }
        }
    }

    let path = "./img.png";
    img.save_with_format(path, image::ImageFormat::Png);
}
