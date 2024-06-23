use image;

use std::fs;

mod vec;
mod matrix;
mod shapes {
    pub mod shape;
    pub mod sphere;
    pub mod plane;
    pub mod cube;
    pub mod intersection;
    pub mod difference;
    pub mod obj;
}
mod object;
mod lights;

use vec::Vec3;
use matrix::Matrix33;
use shapes::shape::IntersectionResult;
use object::Object;
use lights::Light;

use indicatif::ProgressBar;
use clap::Parser;
use serde::{Serialize,Deserialize};

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
            let mut result_color = match intersection.color {
                Some(color) => color,
                None => object.color
            };
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

#[derive(Parser,Debug)]
struct Args {
    /// Path to config file
    #[arg(short, long)]
    config: String,

    /// Path to output file
    #[arg(short, long)]
    output_file: String,

    /// Print objects info
    #[arg(short, long, default_value_t = false)]
    print_debug_objects: bool,
}

#[derive(Serialize,Deserialize)]
struct Config {
    img_size: (i32, i32),
    reflection_depth: u16,
    #[serde(default)]
    start: Vec3,
    #[serde(default)]
    // TODO: convert from radians
    view_angle: Vec3,

    // TODO: Union shape?
    objects: Vec<Object>,
    lights: Vec<Box<dyn Light>>,
}

fn main() {
    let args = Args::parse();

    let viewport_size = (1.0, 1.0);
    let z_dist = 1.0;

    let config_file_raw: String = fs::read_to_string(&args.config).expect("Should have been able to read config file");
    let config: Config = serde_json::from_str(&config_file_raw).expect("Should have been able to parse config file");

    let mut img = image::RgbImage::new(config.img_size.0 as u32, config.img_size.1 as u32);

    let mut objects = config.objects;
    for object in objects.iter_mut() {
        object.init();
    }

    if args.print_debug_objects {
        println!("{:?}", objects);
    }

    let lights = config.lights;


    let bar = ProgressBar::new((img.width() * img.height()) as u64);

    for x in 0..img.width() as i32 {
        for y in 0..img.height() as i32 {
            let eye = Vec3::new(
                viewport_size.0 * ((x - config.img_size.0/2) as f32) / (config.img_size.0 as f32),
                viewport_size.1 * ((y - config.img_size.1/2) as f32) / (config.img_size.1 as f32),
                z_dist)
                .norm();

            let eye = rotate_view(config.view_angle.x(), config.view_angle.y(), config.view_angle.z(), eye);
            let color = ray_trace(&config.start, &eye, &objects, &lights, Some(1.0), None, config.reflection_depth);
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
            bar.inc(1);
        }
    }
    bar.finish();

    img.save_with_format(args.output_file, image::ImageFormat::Png).expect("Can not save result image");
}
