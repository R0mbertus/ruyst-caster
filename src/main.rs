mod gamestate;
mod map;
mod raycaster;

use crate::gamestate::*;
use crate::map::*;
use crate::raycaster::RAYS_AMOUNT;
use bevy::prelude::*;
use bevy::render::render_resource::Texture;
use bevy::asset::Handle;

// constants
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const HALF_WINDOW_WIDTH: f32 = WINDOW_WIDTH / 2.0;
const HALF_WINDOW_HEIGHT: f32 = WINDOW_HEIGHT / 2.0;
const WINDOW: (f32, f32) = (WINDOW_WIDTH, WINDOW_HEIGHT);
const PRECISION: f32 = 64.0;
const LINE_WIDTH: f32 = WINDOW_WIDTH / RAYS_AMOUNT as f32;
const SKY_BLUE: [f32; 4] = [0.40, 0.7, 0.95, 1.0];

#[derive(Component, Resource)]
struct TextureHandles {
    wall: Handle<Image>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { 
                title: "Ruyst-caster".to_string(),
                resizable: false,
                resolution: WINDOW.into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Gamestate::new())
        .add_systems(Startup, setup)
        .add_systems(Startup, load_sprites)
        .add_systems(Update, keyboard_input_system)
        .run();
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    // spawn camera
    commands.spawn(Camera2dBundle::default());

    // Get initial gamestate
    commands.spawn(Gamestate::default());

    // Store texture handles
    commands.insert_resource(TextureHandles {
        wall: asset_server.load("wall.png")
    });    
}

fn load_sprites(
    mut commands: Commands, 
    texture_handles: Res<TextureHandles>
) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            //color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            rect: Some(Rect {
                min: Vec2::new(0.0, 0.0), 
                max: Vec2::new(1.0, 15.0)
            }),
            ..default()
        },
        texture: texture_handles.wall.clone(),
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    });
}

fn keyboard_input_system(
    input: Res<Input<KeyCode>>,
    mut gamestate: ResMut<Gamestate>
) {
    // handle up down
    if input.pressed(KeyCode::W) {
        gamestate.up_down = UpDown::Up;
    } else if input.pressed(KeyCode::S) {
        gamestate.up_down = UpDown::Down;
    } else if input.just_released(KeyCode::S) && gamestate.up_down != UpDown::Up
        || input.just_released(KeyCode::W) && gamestate.up_down != UpDown::Down
    {
        gamestate.up_down = UpDown::None;
    }

    // handle left right
    if input.pressed(KeyCode::A) {
        gamestate.left_right = LeftRight::Left;
    } else if input.pressed(KeyCode::D) {
        gamestate.left_right = LeftRight::Right;
    } else if input.just_released(KeyCode::A) && gamestate.left_right != LeftRight::Right
        || input.just_released(KeyCode::D) && gamestate.left_right != LeftRight::Left
    {
        gamestate.left_right = LeftRight::None;
    }
}

fn render_scene() {

}

// pub struct App {
//     gl: GlGraphics, // OpenGL drawing backend.
//     gamestate: Gamestate,
//     wall: Texture,  // OpenGL texture
// }

// impl Default for App {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl App {
//     pub fn new() -> App {
//         App {
//             gl: GlGraphics::new(OpenGL::V3_2),
//             gamestate: Gamestate::new(),
//             wall: Texture::from_path(
//                 "assets/wall.png",
//                 &TextureSettings::new()
//             ).expect("[ERROR]: Failed to load wall texture")
//         }
//     }

//     fn render(&mut self, args: &RenderArgs) {
//         use graphics::*;

//         self.gl.draw(args.viewport(), |c, gl| {
//             // Clear the screen.
//             clear(GREEN, gl);

//             // Draw sky
//             Rectangle::new(SKY_BLUE).draw(
//                 [0.0, 0.0, WINDOW_WIDTH, HALF_WINDOW_HEIGHT],
//                 &DrawState::default(),
//                 c.transform,
//                 gl,
//             );

//             for (ray, distance) in self.gamestate.get_view().iter().enumerate() {
//                 Line::new(color_distance(RED, *distance as f32), LINE_WIDTH).draw(
//                     [
//                         ray as f32,
//                         HALF_WINDOW_HEIGHT - (HALF_WINDOW_HEIGHT / distance) / 2.0,
//                         ray as f32,
//                         HALF_WINDOW_HEIGHT + (HALF_WINDOW_HEIGHT / distance) / 2.0,
//                     ],
//                     &DrawState::default(),
//                     c.transform,
//                     gl,
//                 );
//             }
//         });
//     }

//     fn update(&mut self, _args: &UpdateArgs) {
//         self.gamestate.update();
//     }
// }

// fn main() {
//     // Change this to OpenGL::V2_1 if not working.
//     let opengl = OpenGL::V3_2;

//     // Create a Glutin window.
//     let mut window: Window =
//         WindowSettings::new("ruyst-caster", [WINDOW_WIDTH - 20.0, WINDOW_HEIGHT])
//             .graphics_api(opengl)
//             .exit_on_esc(true)
//             .build()
//             .unwrap();

//     // Create a new game and run it.
//     let mut app = App::new();

//     let mut events = Events::new(EventSettings::new().max_fps(30));
//     while let Some(e) = events.next(&mut window) {
//         if let Some(args) = e.render_args() {
//             app.render(&args);
//         }

//         if let Some(Button::Keyboard(key)) = e.press_args() {
//             app.handle_key_press(key);
//         }

//         if let Some(Button::Keyboard(key)) = e.release_args() {
//             app.handle_key_release(key);
//         }

//         if let Some(args) = e.update_args() {
//             app.update(&args);
//         }
//     }
// }
