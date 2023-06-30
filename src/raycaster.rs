use std::f64::consts::PI;

use crate::{WINDOW_WIDTH, PRECISION};

const FOV: f64 = 60.0;
const HALF_FOV: f64 = FOV / 2.0;
const RAY_ANGLE_INCREMENT: f64 = FOV / WINDOW_WIDTH;

fn degree_to_radians(degree: f64) -> f64 {
    degree * (PI / 180.0)
}

fn distance(x: f64, y: f64) -> f64 {
    ((x * x) + (y * y)).sqrt()
}

pub fn raycaster(player_angle: f64) -> [f64; WINDOW_WIDTH as usize] {
    let mut ray_angle = player_angle - HALF_FOV;

    for ray in 0..WINDOW_WIDTH {

        ray_angle += RAY_ANGLE_INCREMENT;
    }
}
