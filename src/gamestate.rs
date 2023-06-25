use crate::map::wall_point;

const STEP_SIZE: f32 = 0.05;

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
    x: f32,
    y: f32,
    angle: f32,
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

    pub fn update(&mut self) {
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
