use crate::map::*;

const STEP_SIZE: f64 = 0.02;

pub enum UpDown {
    Up,
    Down,
    None
}
  
pub enum LeftRight {
    Left,
    Right,
    None
}

pub struct Gamestate {
    x: f64,
    y: f64,
    angle: f64,
    up_down: UpDown,
    left_right: LeftRight,
}

impl Gamestate {
    pub fn new() -> Gamestate {
        Gamestate { 
            x: 4.0, 
            y: 4.0, 
            angle: 0.0, 
            up_down: UpDown::None, 
            left_right: LeftRight::None
        }
    }

    // try to eventually find a generic for these two
    pub fn set_up_down(&mut self, up_down: UpDown) {
        self.up_down = up_down;
    }

    pub fn set_left_right(&mut self, left_right: LeftRight) {
        self.left_right = left_right;
    }

    pub fn get_player_pos(&mut self, block_size: (f64, f64)) -> (f64, f64) {
        (self.x as f64 * block_size.1, self.y as f64 * block_size.0)
    }

    pub fn update(&mut self) {
        self.move_player();
    }

    fn move_player(&mut self) {
        let previous_position = (self.x, self.y);

        match self.up_down {
            UpDown::Up => {
                self.x += self.angle.cos() * STEP_SIZE;
                self.y += -self.angle.sin() * STEP_SIZE;
            },
            UpDown::Down => {
                self.x -= self.angle.cos() * STEP_SIZE;
                self.y -= -self.angle.sin() * STEP_SIZE;
            },
            UpDown::None => (),
        }

        match self.left_right {
            LeftRight::Left => self.angle += STEP_SIZE,
            LeftRight::Right => self.angle -= STEP_SIZE,
            LeftRight::None => (),
        }

        if wall_point(self.x as usize, self.y as usize) {
            (self.x, self.y) = previous_position
        }
    }
}
