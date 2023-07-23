mod gamestate;
mod map;
mod raycaster;

use crate::gamestate::*;
use crate::map::*;
use crate::raycaster::RAYS_AMOUNT;
use bevy::prelude::*;
use bevy::asset::LoadState;

// constants
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const HALF_WINDOW_WIDTH: f32 = WINDOW_WIDTH / 2.0;
const HALF_WINDOW_HEIGHT: f32 = WINDOW_HEIGHT / 2.0;
const WINDOW: (f32, f32) = (WINDOW_WIDTH, WINDOW_HEIGHT);
const PRECISION: f32 = 64.0;
const SPRITE_WIDTH: f32 = WINDOW_WIDTH / RAYS_AMOUNT as f32;

#[derive(Resource)]
struct AssetsLoading(Vec<HandleUntyped>);

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
        .add_systems(Startup, (setup, load_sprites))
        .add_systems(Update, (keyboard_input_system, update))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) {
    // spawn camera
    commands.spawn(Camera2dBundle::default());

    let wall_handle: Handle<Image> = asset_server.load("wall.png");

    loading.0.push(wall_handle.clone_untyped());
}

fn load_sprites(mut commands: Commands, asset_server: Res<AssetServer>, loading: Res<AssetsLoading>) {
    match asset_server.get_group_load_state(loading.0.iter().map(|h| h.id())) {
        LoadState::Failed => {
            panic!("[ERROR]: Failed to load asset");
        }
        LoadState::Loaded => {

        }
        _ => {}
    }

    // spawn wall
    for x in 0..RAYS_AMOUNT {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(sprite_width, HALF_WINDOW_HEIGHT)),
                rect: Some(Rect {
                    min: Vec2::new(0., 0.),
                    max: Vec2::new(1., 15.0),
                }),
                ..default()
            },
            texture: wall_handle.clone(),
            transform: Transform::from_translation(Vec3::new(
                -HALF_WINDOW_WIDTH + x as f32,
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
        sprite.custom_size.as_mut().unwrap().y = HALF_WINDOW_HEIGHT / rays[i].distance as f32;

        // let sprite_rect = sprite.rect.as_mut().unwrap();
        // sprite_rect.min.x = rays[i].get_texture_x(16.0);
        // sprite_rect.max.y = sprite_rect.min.x + 1.;

        //sprite.color = color_distance(Color::RED, rays[i].distance);
    }
}
