//Activate Clippy, helps with proper formatting
#![deny(clippy::all)]

use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(setup_board)
        .run();
}

const TILE_SIZE: f32 = 80.0;
const BOARD_SIZE: usize = 8;

fn setup_board(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    let window = window_query.get_single().unwrap();
    let horiz_displacement = window.width() / 2. - TILE_SIZE * 3.5;
    let vert_displacement = window.height() / 2. - TILE_SIZE * 3.5;
    // Create the checkered board
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            let tile_color = if (row + col) % 2 == 0 {
                Color::rgb(0.345098, 0.09412, 0.270588)
            } else {
                Color::rgb(0.949, 0.8235, 0.741176)
            };

            commands.spawn(SpriteBundle {
                sprite: Sprite{
                    color: tile_color,
                    rect: Some(Rect::new(0.,0.,TILE_SIZE,TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    col as f32 * TILE_SIZE + horiz_displacement,
                    row as f32 * TILE_SIZE + vert_displacement,
                    0.0,
                ),
                //texture: asset_server.load("sprites/chess-piece-sprites.png"),
                ..default()
            });
        }
    }
    // Put the pieces on the board
    // First pawns
    for col in 0..BOARD_SIZE {
        //black pawns
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(
                col as f32 * TILE_SIZE + horiz_displacement,
                6. * TILE_SIZE + vert_displacement,
                0.0,
            ).with_scale(Vec3::new(0.25,0.25,1.)),
            texture: asset_server.load("sprites/Bpawn.png"),
            ..default()
        });
        //white pawns
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(
                col as f32 * TILE_SIZE + horiz_displacement,
                1. * TILE_SIZE + vert_displacement,
                0.0,
            ).with_scale(Vec3::new(0.25,0.25,1.)),
            texture: asset_server.load("sprites/Wpawn.png"),
            ..default()
        });
    }
    //kings
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(
            4. * TILE_SIZE + horiz_displacement,
            0. * TILE_SIZE + vert_displacement,
            0.0,
        ).with_scale(Vec3::new(0.25,0.25,1.)),
        texture: asset_server.load("sprites/Wking.png"),
        ..default()
    });

    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(
            4. * TILE_SIZE + horiz_displacement,
            7. * TILE_SIZE + vert_displacement,
            0.0,
        ).with_scale(Vec3::new(0.25,0.25,1.)),
        texture: asset_server.load("sprites/Bking.png"),
        ..default()
    });
    //queens
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(
            3. * TILE_SIZE + horiz_displacement,
            0. * TILE_SIZE + vert_displacement,
            0.0,
        ).with_scale(Vec3::new(0.25,0.25,1.)),
        texture: asset_server.load("sprites/Wqueen.png"),
        ..default()
    });
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(
            3. * TILE_SIZE + horiz_displacement,
            7. * TILE_SIZE + vert_displacement,
            0.0,
        ).with_scale(Vec3::new(0.25,0.25,1.)),
        texture: asset_server.load("sprites/Bqueen.png"),
        ..default()
    });
    //Bishops
    for col in 0..2 {
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(
                (col as f32 * 3. + 2.) * TILE_SIZE + horiz_displacement,
                7. * TILE_SIZE + vert_displacement,
                0.0,
            ).with_scale(Vec3::new(0.25,0.25,1.)),
            texture: asset_server.load("sprites/Bbishop.png"),
            ..default()
        });
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(
                (col as f32 * 3. + 2.) * TILE_SIZE + horiz_displacement,
                0. * TILE_SIZE + vert_displacement,
                0.0,
            ).with_scale(Vec3::new(0.25,0.25,1.)),
            texture: asset_server.load("sprites/Wbishop.png"),
            ..default()
        });
    }
    // Knights
    for col in 0..2 {
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(
                (col as f32 * 5. + 1.) * TILE_SIZE + horiz_displacement,
                7. * TILE_SIZE + vert_displacement,
                0.0,
            ).with_scale(Vec3::new(0.25,0.25,1.)),
            texture: asset_server.load("sprites/Bhorse.png"),
            ..default()
        });
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(
                (col as f32 * 5. + 1.) * TILE_SIZE + horiz_displacement,
                0. * TILE_SIZE + vert_displacement,
                0.0,
            ).with_scale(Vec3::new(0.25,0.25,1.)),
            texture: asset_server.load("sprites/Whorse.png"),
            ..default()
        });
    }
    // Rooks
    for col in 0..2 {
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(
                (col as f32 * 7.) * TILE_SIZE + horiz_displacement,
                7. * TILE_SIZE + vert_displacement,
                0.0,
            ).with_scale(Vec3::new(0.25,0.25,1.)),
            texture: asset_server.load("sprites/Brook.png"),
            ..default()
        });
        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(
                (col as f32 * 7.) * TILE_SIZE + horiz_displacement,
                0. * TILE_SIZE + vert_displacement,
                0.0,
            ).with_scale(Vec3::new(0.25,0.25,1.)),
            texture: asset_server.load("sprites/Wrook.png"),
            ..default()
        });
    }

}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}