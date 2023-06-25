extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod map;
mod gamestate;

// piston use
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{EventLoop, Button, PressEvent, ReleaseEvent};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, Key};
use piston::window::WindowSettings;

// project use
use crate::map::*;
use crate::gamestate::*;

const WINDOW_HEIGHT: f64 = 600.0;
const WINDOW_WIDTH: f64 = 800.0;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    gamestate: Gamestate,
    block_size: (f64, f64)
}

impl App {
    pub fn new() -> App {
        App { 
            gl: GlGraphics::new(OpenGL::V3_2),  
            gamestate: Gamestate::new(),
            block_size: map::block_size(WINDOW_HEIGHT, WINDOW_WIDTH)
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            for y in 0..map::HEIGHT {
                for x in 0..map::WIDTH {
                    if map::wall_point(x, y) {
                        Rectangle::new(WHITE).draw(
                            [0.0, 0.0, self.block_size.1, self.block_size.0], 
                            &DrawState::default(), 
                            c.transform.trans(x as f64 * self.block_size.1, y as f64 * self.block_size.0), 
                            gl
                        );
                    }
                }
            }

            let player_pos: (f64, f64) = self.gamestate.get_player_pos(self.block_size);
            Rectangle::new(GREEN).draw(
                [0.0, 0.0, self.block_size.1, self.block_size.0], 
                &DrawState::default(), 
                c.transform.trans(player_pos.1, player_pos.0), 
                gl
            );
        });
    }

    fn handle_key_press(&mut self, key: Key) {
        match key {
            Key::W => self.gamestate.set_up_down(UpDown::Up),
            Key::S => self.gamestate.set_up_down(UpDown::Down),
            Key::A => self.gamestate.set_left_right(LeftRight::Left),
            Key::D => self.gamestate.set_left_right(LeftRight::Right),
            _ => ()
        }
    }

    fn handle_key_release(&mut self, key: Key) {
        match key {
            Key::W | Key::S => self.gamestate.set_up_down(UpDown::None),
            Key::A | Key::D => self.gamestate.set_left_right(LeftRight::None),
            _ => ()
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.gamestate.update();
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new();

    let mut events = Events::new(EventSettings::new().max_fps(30));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.handle_key_press(key);
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            app.handle_key_release(key);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}