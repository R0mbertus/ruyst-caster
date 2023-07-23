use bevy::prelude::{Component, Resource};

use crate::{map::*, raycaster::*, WINDOW_WIDTH};

const STEP_SIZE: f32 = 0.035;
const ROTATION: f32 = 1.5;

#[derive(PartialEq, Debug)]
pub enum UpDown {
    Up,
    Down,
    None,
}

#[derive(PartialEq, Debug)]
pub enum LeftRight {
    Left,
    Right,
    None,
}

#[derive(Component, Debug, Resource)]
pub struct Gamestate {
    x: f32,
    y: f32,
    angle: f32,
    pub up_down: UpDown,
    pub left_right: LeftRight,
}

impl Gamestate {
    pub fn new() -> Gamestate {
        Gamestate {
            x: 4.0,
            y: 4.0,
            angle: 0.0,
            up_down: UpDown::None,
            left_right: LeftRight::None,
        }
    }

    pub fn get_view(&mut self) -> [Ray; WINDOW_WIDTH as usize] {
        raycaster(self.x, self.y, self.angle)
    }

    pub fn update(&mut self) {
        self.move_player();
    }

    fn move_player(&mut self) {
        let previous_position = (self.x, self.y);

        match self.up_down {
            UpDown::Up => {
                self.x += degree_to_radians(self.angle).cos() * STEP_SIZE;
                self.y += degree_to_radians(self.angle).sin() * STEP_SIZE;
            }
            UpDown::Down => {
                self.x -= degree_to_radians(self.angle).cos() * STEP_SIZE;
                self.y -= degree_to_radians(self.angle).sin() * STEP_SIZE;
            }
            UpDown::None => (),
        }

        match self.left_right {
            LeftRight::Left => self.angle -= ROTATION,
            LeftRight::Right => self.angle += ROTATION,
            LeftRight::None => (),
        }

        if wall_point(self.x, self.y) {
            (self.x, self.y) = previous_position
        }
    }
}

impl Default for Gamestate {
    fn default() -> Self {
        Gamestate::new()
    }
}
