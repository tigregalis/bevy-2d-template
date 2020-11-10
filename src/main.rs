#![feature(external_doc)]
#![doc(include = "../README.md")]

use bevy::{
    app::AppExit, diagnostic::FrameTimeDiagnosticsPlugin, diagnostic::PrintDiagnosticsPlugin,
    prelude::*,
};

mod display_diagnostic;
use display_diagnostic::DisplayDiagnosticsPlugin;

mod font;
use font::{fonts, FontMap};

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(DisplayDiagnosticsPlugin::default())
        .add_resource(GameState::Starting)
        .add_resource(ClearColor(Color::BLACK)) // the window's background colour
        .init_resource::<FontMap>()
        .add_startup_system(setup.system())
        .add_startup_system(start_game_system.system())
        .add_system(test_change_text_system_a.system())
        .add_system(test_change_text_system_b.system())
        .add_system(end_game_system.system())
        .add_system(start_pause_game_system.system())
        .add_system(exit_on_esc_system.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut font_map: ResMut<FontMap>) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        // .spawn(TextComponents {
        //     text: Text {
        //         font: asset_server.load(fonts::JETBRAINSMONO_MEDIUM),
        //         value: "This text wraps".to_string(),
        //         style: TextStyle {
        //             color: Color::RED * [1.0, 1.0, 1.0, 0.5],
        //             font_size: 20.0,
        //         },
        //     },
        //     style: Style {
        //         position_type: PositionType::Absolute,
        //         position: Rect {
        //             top: Val::Px(5.0),
        //             right: Val::Px(5.0),
        //             ..Default::default()
        //         },
        //         ..Default::default()
        //     },
        //     ..Default::default()
        // })
        .spawn(TextComponents {
            text: Text {
                font: asset_server.load(fonts::JETBRAINSMONO_MEDIUM),
                value: "Thistextwraps".to_string(),
                style: TextStyle {
                    color: Color::RED * [1.0, 1.0, 1.0, 0.5],
                    font_size: 40.0,
                },
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .spawn(TextComponents {
            text: Text {
                font: asset_server.load(fonts::JETBRAINSMONO_MEDIUM),
                value: "This\ntext\nwraps".to_string(),
                style: TextStyle {
                    color: Color::WHITE * [1.0, 1.0, 1.0, 0.5],
                    font_size: 40.0,
                },
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        // .spawn(TextComponents {
        //     text: Text {
        //         font: asset_server.load(fonts::JETBRAINSMONO_MEDIUM),
        //         value: "This \n text \nwraps".to_string(),
        //         style: TextStyle {
        //             color: Color::WHITE * [1.0, 1.0, 1.0, 0.5],
        //             font_size: 20.0,
        //         },
        //     },
        //     style: Style {
        //         position_type: PositionType::Absolute,
        //         position: Rect {
        //             top: Val::Px(5.0),
        //             right: Val::Px(5.0),
        //             ..Default::default()
        //         },
        //         ..Default::default()
        //     },
        //     ..Default::default()
        // })
        ;
}

struct A;

fn test_change_text_system_a(mut text_query: Query<(&mut Text, &A)>) {
    for (mut text, _a) in &mut text_query.iter() {
        text.value = format!("this is not\nwhat it was\nbefore");
    }
}

struct B;

fn test_change_text_system_b(mut text_query: Query<(&mut Text, &B)>) {
    for (mut text, _b) in &mut text_query.iter() {
        text.value = format!("this is notwhat it wasbefore");
    }
}

fn end_game_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut despawn_query: Query<(Entity, &DespawnOnEnd)>,
    // color_material_handle_query: Query<&Handle<ColorMaterial>>,
) {
    if *game_state == GameState::Restarting {
        for (entity, _) in &mut despawn_query.iter() {
            commands.despawn(entity);
        }
        start_game_system(commands, materials);
        *game_state = GameState::Starting;
    }
}

fn start_game_system(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        // ball
        .spawn(SpriteComponents {
            material: materials.add(Color::WHITE.into()),
            transform: Transform {
                translation: Vec3::new(0.0, -0.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(DespawnOnEnd);
}

fn start_pause_game_system(mut game_state: ResMut<GameState>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_released(KeyCode::Space) {
        *game_state = match *game_state {
            GameState::Starting => GameState::Playing,
            GameState::Restarting => GameState::Restarting,
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
            GameState::Win => GameState::Restarting,
            GameState::Lose => GameState::Restarting,
        }
    } else if keyboard_input.just_released(KeyCode::R) {
        *game_state = GameState::Restarting;
    }
}

struct DespawnOnEnd;

#[derive(PartialEq, Eq)]
#[allow(dead_code)]
enum GameState {
    Starting,
    Restarting,
    Playing,
    Paused,
    Win,
    Lose,
}

fn exit_on_esc_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        app_exit_events.send(AppExit);
    }
}

use std::collections::HashMap;
use std::ops::Add;

pub struct Controllable;

pub struct Direction(Point);

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct TileType {
    blocking: bool,
}

pub struct Array2<T>(HashMap<Point, T>);

pub fn move_controllables(
    map: Res<Array2<Entity>>,
    mut controllable_query: Query<(&Controllable, &mut Direction, &mut Point)>,
    tile_type_query: Query<(Entity, &TileType)>,
) {
    for (_, mut direction, mut position) in &mut controllable_query.iter() {

        let mut direction = &mut direction.0;
        
        let new_pos: Point = *position + *direction;
        
        if new_pos.x < 80           // Check that the new position isn't offscreen.
        && new_pos.x >= 0
        && new_pos.y < 50
        && new_pos.y >= 0
        {
            position.x = new_pos.x;
            position.y = new_pos.y;
            direction.x = 0;
            direction.y = 0;
        }

        if let Some(&entity) = map.0.get(&new_pos) {
            if let Ok(tile_type) = tile_type_query.get::<TileType>(entity) {
                if tile_type.blocking {
                    // do whatever
                }
            }
        }
    }
}