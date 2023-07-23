mod gamestate;
mod map;
mod raycaster;

use crate::gamestate::*;
use crate::map::*;
use crate::raycaster::RAYS_AMOUNT;
use bevy::prelude::*;

// constants
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const HALF_WINDOW_WIDTH: f32 = WINDOW_WIDTH / 2.0;
const HALF_WINDOW_HEIGHT: f32 = WINDOW_HEIGHT / 2.0;
const WINDOW: (f32, f32) = (WINDOW_WIDTH, WINDOW_HEIGHT);
const PRECISION: f32 = 64.0;
const SPRITE_WIDTH: f32 = WINDOW_WIDTH / RAYS_AMOUNT as f32;

fn main() {
    App::new()
        .insert_resource(Gamestate::new())
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Ruyst-caster".to_string(),
                        resizable: false,
                        resolution: WINDOW.into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (keyboard_input_system, update))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, images: Res<Assets<Image>>) {
    // spawn camera
    commands.spawn(Camera2dBundle::default());

    let wall_handle: Handle<Image> = asset_server.load("wall.png");

    if let Some(_wall) = images.get(&wall_handle) {
        info!("[SETUP]: textures loaded");
    }

    for x in 0..RAYS_AMOUNT {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SPRITE_WIDTH, HALF_WINDOW_HEIGHT)),
                rect: Some(Rect {
                    min: Vec2::new((x % 16) as f32 / 15., 0.),
                    max: Vec2::new(((x + 1) % 16) as f32 / 15., 15.0),
                }),
                ..default()
            },
            texture: wall_handle.clone(),
            transform: Transform::from_translation(Vec3::new(
                -HALF_WINDOW_WIDTH + x as f32 * SPRITE_WIDTH,
                0.,
                0.,
            )),
            ..default()
        });
    }
}

fn keyboard_input_system(input: Res<Input<KeyCode>>, mut gamestate: ResMut<Gamestate>) {
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

fn update(mut gamestate: ResMut<Gamestate>, mut query: Query<&mut Sprite>) {
    gamestate.update();

    let rays = gamestate.get_view();

    for (i, mut sprite) in query.iter_mut().enumerate() {
        sprite.custom_size.as_mut().unwrap().y = HALF_WINDOW_HEIGHT / rays[i].distance;

        let sprite_rect = sprite.rect.as_mut().unwrap();
        sprite_rect.min.x = rays[i].get_texture_x(16.0);
        sprite_rect.max.x = sprite_rect.min.x + 1.;

        sprite.color = color_distance(sprite.color, rays[i].distance)
    }
}
