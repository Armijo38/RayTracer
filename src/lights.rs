use crate::vec::Vec3;
use serde::{Serialize,Deserialize};

#[typetag::serde(tag="type")]
pub trait Light {
    fn intensity(&self, point: &Vec3, norm: &Vec3) -> f32;
    fn specular(&self, point: &Vec3, norm: &Vec3, eye: &Vec3, s: u32) -> f32;
    fn point(&self) -> Option<&Vec3> {
        None
    }
}

#[derive(Serialize,Deserialize)]
pub struct PointLight {
    position: Vec3,
    intensity: f32,
}

#[typetag::serde(name="point")]
impl Light for PointLight {
    fn intensity(&self, point: &Vec3, norm: &Vec3) -> f32 {
        let direction = &self.position - point;
        if (Vec3::new(0.0, 0.0, 3.5) - point).length() < 1e-4 {
            println!("{} {}", direction, &direction.cos(&norm) * self.intensity);
        }
        f32::max(direction.cos(&norm), 0.0) * self.intensity
    }

    fn specular(&self, point: &Vec3, norm: &Vec3, eye: &Vec3, s: u32) -> f32 {
        let light_direction = &self.position - point;
        let norm_dot_direction = norm.norm().dot(&light_direction);

        let r = norm.norm() * 2.0 * norm_dot_direction - light_direction;

        let eye_cos_r = -eye.cos(&r);
        if eye_cos_r <= 0.0 {
            return 0.0;
        }
        self.intensity * eye_cos_r.powf(s as f32)
    }

    fn point(&self) -> Option<&Vec3> {
        Some(&self.position)
    }
}

impl PointLight {
    #[allow(dead_code)]
    pub fn new(position: Vec3, intensity: f32) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}

#[derive(Serialize,Deserialize)]
pub struct AmbientLight {
    intensity: f32,
}

#[typetag::serde(name="ambient")]
impl Light for AmbientLight {
    fn intensity(&self, _point: &Vec3, _norm: &Vec3) -> f32 {
        self.intensity
    }

    fn specular(&self, _point: &Vec3, _norm: &Vec3, _eye: &Vec3, _s: u32) -> f32 {
        0.0
    }
}

impl AmbientLight {
    #[allow(dead_code)]
    pub fn new(intensity: f32) -> AmbientLight {
        AmbientLight{intensity}
    }
}
