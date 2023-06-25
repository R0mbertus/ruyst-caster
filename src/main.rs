extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod map;
mod gamestate;

// piston use
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{EventLoop, Button, PressEvent};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, Key};
use piston::window::WindowSettings;

// project use
use crate::gamestate::*;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    gamestate: Gamestate
}

impl App {
    pub fn new() -> App {
        App { 
            gl: GlGraphics::new(OpenGL::V3_2),  
            gamestate: Gamestate::new()
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
        });
    }

    fn handle_key_press(&mut self, key: Key) {
        match key {
            Key::W => self.gamestate.setUpDown(UpDown::Up),
            Key::S => self.gamestate.setUpDown(UpDown::Down),
            Key::A => self.gamestate.setLeftRight(LeftRight::Left),
            Key::D => self.gamestate.setLeftRight(LeftRight::Right),
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
    let mut window: Window = WindowSettings::new("spinning-square", [800, 600])
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

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}