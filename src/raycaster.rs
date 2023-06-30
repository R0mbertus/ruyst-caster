use std::f64::consts::PI;

use crate::{map::wall_point, HALF_WINDOW_HEIGHT, PRECISION, WINDOW_WIDTH};

const FOV: f64 = 60.0;
const HALF_FOV: f64 = FOV / 2.0;
const RAY_ANGLE_INCREMENT: f64 = FOV / WINDOW_WIDTH;

pub fn degree_to_radians(degree: f64) -> f64 {
    degree * (PI / 180.0)
}

fn distance(x: f64, y: f64) -> f64 {
    ((x * x) + (y * y)).sqrt()
}

pub fn raycaster(x: f64, y: f64, player_angle: f64) -> [f64; WINDOW_WIDTH as usize] {
    let mut ray_angle: f64 = player_angle - HALF_FOV;

    let mut rays = [0.0; WINDOW_WIDTH as usize];

    for (_ray, wall_height) in rays.iter_mut().enumerate() {
        let mut ray_x: f64 = x;
        let mut ray_y: f64 = y;

        let ray_cos = degree_to_radians(ray_angle).cos() / PRECISION;
        let ray_sin = degree_to_radians(ray_angle).sin() / PRECISION;

        while !wall_point(ray_x, ray_y) {
            ray_x += ray_cos;
            ray_y += ray_sin;
        }

        let distance =
            distance(x - ray_x, y - ray_y) * degree_to_radians(ray_angle - player_angle).cos();

        *wall_height = HALF_WINDOW_HEIGHT / distance;

        ray_angle += RAY_ANGLE_INCREMENT;
    }

    rays
}
