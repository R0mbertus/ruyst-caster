use std::f32::consts::PI;

use crate::{map::wall_point, PRECISION, WINDOW_WIDTH};

pub const RAYS_AMOUNT: usize = WINDOW_WIDTH as usize;
const FOV: f32 = 60.0;
const HALF_FOV: f32 = FOV / 2.0;
const RAY_ANGLE_INCREMENT: f32 = FOV / RAYS_AMOUNT as f32;

#[derive(Default, Copy, Clone)]
pub struct Ray {
    x: f32,
    y: f32,
    pub distance: f32,
}

impl Ray {
    pub fn get_texture_x(self, texture_width: f32) -> f32 {
        (texture_width * (self.x + self.y)).floor() % texture_width
    }
}

pub fn degree_to_radians(degree: f32) -> f32 {
    degree * (PI / 180.0)
}

fn get_distance(x: f32, y: f32) -> f32 {
    ((x * x) + (y * y)).sqrt()
}

pub fn raycaster(x: f32, y: f32, player_angle: f32) -> [Ray; RAYS_AMOUNT] {
    let mut ray_angle: f32 = player_angle - HALF_FOV;

    let mut rays = [Ray::default(); RAYS_AMOUNT];

    for (_count, ray) in rays.iter_mut().enumerate() {
        let mut ray_x: f32 = x;
        let mut ray_y: f32 = y;

        let ray_cos = degree_to_radians(ray_angle).cos() / PRECISION;
        let ray_sin = degree_to_radians(ray_angle).sin() / PRECISION;

        while !wall_point(ray_x, ray_y) {
            ray_x += ray_cos;
            ray_y += ray_sin;
        }

        *ray = Ray {
            x: ray_x,
            y: ray_y,
            distance: degree_to_radians(ray_angle - player_angle).cos() * get_distance(x - ray_x, y - ray_y),
        };

        ray_angle += RAY_ANGLE_INCREMENT;
    }

    rays
}
