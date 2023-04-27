//Activate Clippy, helps with proper formatting
#![deny(clippy::all)]

use bevy::{prelude::*, window::PrimaryWindow, ecs::entity, transform};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(setup_board)
        .add_system(mouse_click_system)
        .run();
}

const TILE_SIZE: f32 = 80.0;
const BOARD_SIZE: usize = 8;

#[derive(Component)]
struct CurrentSelectedPiece;

#[derive(Component)]
struct Piece {
    color : PieceColor,
    piece_type : PieceType,
}

enum PieceColor {
    White,
    Black,
}

enum PieceType {
    Pawn,
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
}

#[derive(PartialEq, Component)]
struct Position {
    x : f32,
    y : f32,
}

#[derive(Component)]
struct Redtile;


fn setup_board(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    let window = window_query.get_single().unwrap();
    let horiz_displacement = window.width() / 2. - TILE_SIZE * 3.5;
    let vert_displacement = window.height() / 2. - TILE_SIZE * 3.5;
    // Create the checkered board
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            let tile_color = if (row + col) % 2 == 0 {
                Color::rgb_u8(150, 77, 34)
            } else {
                Color::rgb_u8(218, 217, 181)
            };

            commands.spawn(SpriteBundle{
                sprite: Sprite{
                    color: tile_color,
                    rect: Some(Rect::new(0.,0.,TILE_SIZE,TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    col as f32 * TILE_SIZE + horiz_displacement,
                    row as f32 * TILE_SIZE + vert_displacement,
                    -1.0,
                ),
                ..default()
            });
        }
    }
    // Put the pieces on the board
    // First pawns
    for col in 0..BOARD_SIZE {
        //black pawns
        commands.spawn((SpriteBundle {
            transform: Transform::from_xyz(
                col as f32 * TILE_SIZE + horiz_displacement,
                6. * TILE_SIZE + vert_displacement,
                0.0,
            ).with_scale(Vec3::new(0.25,0.25,1.)),
            texture: asset_server.load("sprites/Bpawn.png"),
            ..default()
        },
        Piece{color : PieceColor::Black, piece_type : PieceType::Pawn},
        Position {x: col as f32, y: 6.}
    ));
        //white pawns
        commands.spawn((SpriteBundle {
            transform: Transform::from_xyz(
                col as f32 * TILE_SIZE + horiz_displacement,
                1. * TILE_SIZE + vert_displacement,
                0.0,
            ).with_scale(Vec3::new(0.25,0.25,1.)),
            texture: asset_server.load("sprites/Wpawn.png"),
            ..default()
        },
        Piece{color : PieceColor::White, piece_type : PieceType::Pawn},
        Position {x: col as f32, y: 1.},
    ));
    }

    //kings
    commands.spawn((SpriteBundle {
        transform: Transform::from_xyz(
            4. * TILE_SIZE + horiz_displacement,
            0. * TILE_SIZE + vert_displacement,
            0.0,
        ).with_scale(Vec3::new(0.25,0.25,1.)),
        texture: asset_server.load("sprites/Wking.png"),
        ..default()
        },
        Piece{color : PieceColor::White, piece_type : PieceType::King},
        Position {x: 4., y: 0.},
    ));
    commands.spawn((SpriteBundle {
        transform: Transform::from_xyz(
            4. * TILE_SIZE + horiz_displacement,
            7. * TILE_SIZE + vert_displacement,
            0.0,
        ).with_scale(Vec3::new(0.25,0.25,1.)),
        texture: asset_server.load("sprites/Bking.png"),
        ..default()
        },
        Piece{color : PieceColor::Black, piece_type : PieceType::King},
        Position {x: 4., y: 7.},
    ));

    //queens
    commands.spawn((SpriteBundle {
        transform: Transform::from_xyz(
            3. * TILE_SIZE + horiz_displacement,
            0. * TILE_SIZE + vert_displacement,
            0.0,
        ).with_scale(Vec3::new(0.25,0.25,1.)),
        texture: asset_server.load("sprites/Wqueen.png"),
        ..default()
        },
        Piece{color : PieceColor::White, piece_type : PieceType::Queen},
        Position {x: 3., y: 0.},
    ));
    commands.spawn((SpriteBundle {
        transform: Transform::from_xyz(
            3. * TILE_SIZE + horiz_displacement,
            7. * TILE_SIZE + vert_displacement,
            0.0,
        ).with_scale(Vec3::new(0.25,0.25,1.)),
        texture: asset_server.load("sprites/Bqueen.png"),
        ..default()
        },
        Piece{color : PieceColor::Black, piece_type : PieceType::Queen},
        Position {x: 3., y: 7.},
    ));

    //Bishops
    for col in 0..2 {
        commands.spawn((SpriteBundle {
            transform: Transform::from_xyz(
                (col as f32 * 3. + 2.) * TILE_SIZE + horiz_displacement,
                7. * TILE_SIZE + vert_displacement,
                0.0,
            ).with_scale(Vec3::new(0.25,0.25,1.)),
            texture: asset_server.load("sprites/Bbishop.png"),
            ..default()
            },
            Piece{color : PieceColor::Black, piece_type : PieceType::Bishop},
            Position {x: (col as f32 * 3. + 2.), y: 7.},
        ));
        commands.spawn((SpriteBundle {
            transform: Transform::from_xyz(
                (col as f32 * 3. + 2.) * TILE_SIZE + horiz_displacement,
                0. * TILE_SIZE + vert_displacement,
                0.0,
            ).with_scale(Vec3::new(0.25,0.25,1.)),
            texture: asset_server.load("sprites/Wbishop.png"),
            ..default()
            },
            Piece{color : PieceColor::White, piece_type : PieceType::Bishop},
            Position {x: (col as f32 * 3. + 2.), y: 0.},
        ));
    }

    // Knights
    for col in 0..2 {
        commands.spawn((SpriteBundle {
            transform: Transform::from_xyz(
                (col as f32 * 5. + 1.) * TILE_SIZE + horiz_displacement,
                7. * TILE_SIZE + vert_displacement,
                0.0,
            ).with_scale(Vec3::new(0.25,0.25,1.)),
            texture: asset_server.load("sprites/Bhorse.png"),
            ..default()
            },
            Piece{color : PieceColor::Black, piece_type : PieceType::Knight},
            Position {x: (col as f32 * 5. + 1.), y: 7.},
        ));
        commands.spawn((SpriteBundle {
            transform: Transform::from_xyz(
                (col as f32 * 5. + 1.) * TILE_SIZE + horiz_displacement,
                0. * TILE_SIZE + vert_displacement,
                0.0,
            ).with_scale(Vec3::new(0.25,0.25,1.)),
            texture: asset_server.load("sprites/Whorse.png"),
            ..default()
            },
            Piece{color : PieceColor::White, piece_type : PieceType::Knight},
            Position {x: (col as f32 * 5. + 1.), y: 0.},
        ));
    }

    // Rooks
    for col in 0..2 {
        commands.spawn((SpriteBundle {
            transform: Transform::from_xyz(
                (col as f32 * 7.) * TILE_SIZE + horiz_displacement,
                7. * TILE_SIZE + vert_displacement,
                0.0,
            ).with_scale(Vec3::new(0.25,0.25,1.)),
            texture: asset_server.load("sprites/Brook.png"),
            ..default()
            },
            Piece{color : PieceColor::Black, piece_type : PieceType::Rook},
            Position {x: (col as f32 * 7.), y: 7.},
        ));
        commands.spawn((SpriteBundle {
            transform: Transform::from_xyz(
                (col as f32 * 7.) * TILE_SIZE + horiz_displacement,
                0. * TILE_SIZE + vert_displacement,
                0.0,
            ).with_scale(Vec3::new(0.25,0.25,1.)),
            texture: asset_server.load("sprites/Wrook.png"),
            ..default()
            },
            Piece{color : PieceColor::White, piece_type : PieceType::Rook},
            Position {x: (col as f32 * 7.), y: 0.},
        ));
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    piece_query: Query<(Entity, &mut Transform, &mut Position, &Piece), Without<CurrentSelectedPiece>>,
    mut curr_piece_query: Query<(Entity, &mut Transform, &mut Position, &Piece, &CurrentSelectedPiece)>,
    mut commands: Commands,
    red_tiles: Query<Entity, With<Redtile>>,
) {
    let window = window_query.get_single().unwrap();
    let horiz_displacement = window.width() / 2. - TILE_SIZE * 3.5;
    let vert_displacement = window.height() / 2. - TILE_SIZE * 3.5;
    
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mouse_tile = find_mouse_tile(window.cursor_position().unwrap(), window);
        
        // Deal with input not on the board by doing nothing
        if mouse_tile[0] < BOARD_SIZE as f32 && mouse_tile[0] >= 0. && mouse_tile[1] < BOARD_SIZE as f32 && mouse_tile[1] >= 0. {
            // if selected piece is picked up, set it down at the tile
            // if piece is already picked up/invalid move square, do nothing (todo! or error noise? blinking red or something) 
            match curr_piece_query.get_single_mut() {
                // has piece picked up
                Ok((curr_entity, mut curr_trans, mut curr_pos, piece_qual, _curr_sel_piece)) => {
                    // if !(valid_tiles(curr_pos.x, curr_pos.y, piece_qual, &piece_query).contains(&mouse_tile)) {
                    //     // insert error noise or blinking? to signal wrong move
                    //     return;
                    // }
                    // Then if it is a valid tile, move the piece there
                    for (entity, _transform, position, piece_qual) in piece_query.into_iter() {
                        if position.x == mouse_tile[0] && position.y == mouse_tile[1] {
                            commands.entity(entity).despawn();
                        }
                    }
                    
                    let direction = Vec3::new((mouse_tile[0] - curr_pos.x) * TILE_SIZE,
                                                    (mouse_tile[1] - curr_pos.y) * TILE_SIZE, 
                                                    0.);
                    curr_pos.x = mouse_tile[0];
                    curr_pos.y = mouse_tile[1];
                    curr_trans.translation += direction;
                    commands.entity(curr_entity).remove::<CurrentSelectedPiece>();
                    if !red_tiles.is_empty() {
                        for tile in red_tiles.into_iter() {
                            commands.entity(tile).despawn();
                        }
                    }
                },
                // no piece picked up
                Err(_) => 
                    // if piece occupies the square, pick piece "up"
                    for (entity, _transform, position, piece_qual) in piece_query.into_iter() {
                        if position.x == mouse_tile[0] && position.y == mouse_tile[1] {
                            commands.entity(entity).insert(CurrentSelectedPiece);
                            commands.spawn((
                                SpriteBundle{
                                    sprite: Sprite{
                                        color: Color::rgb_u8(244, 113, 116),
                                        rect: Some(Rect::new(0.,0.,TILE_SIZE,TILE_SIZE)),
                                        ..default()
                                    },
                                    transform: Transform::from_xyz(
                                        mouse_tile[0] * TILE_SIZE + horiz_displacement,
                                        mouse_tile[1] * TILE_SIZE + vert_displacement,
                                        0.,
                                    ),
                                    ..default()
                                },
                                Redtile,
                            ));
                        }
                    }
            }
        }
    }
}

pub fn find_mouse_tile(input : Vec2, window : &Window) -> Vec2 {
    let horiz_displacement = window.width() / 2. - TILE_SIZE * 4.;
    let vert_displacement = window.height() / 2. - TILE_SIZE * 4.;
    Vec2::new(f32::floor((input[0] - horiz_displacement) / TILE_SIZE), 
              f32::floor((input[1] - vert_displacement) / TILE_SIZE))
    // return is 0 indexed!
}

// also have to somehow check that the king is not in check --- TBD
// fn valid_tiles(x_curr : f32, y_curr : f32, piece : &Piece, 
//                     piece_query: &Query<(Entity, &mut Transform, &mut Position, &Piece), Without<CurrentSelectedPiece>>
//                 ) -> Vec<Vec2> {
//     let mut to_return: Vec<Vec2>;
//     // Now fill to_return with all possible moves
//     // piece_query is all the pieces on the board besides the one we are currently parsing
//     // Look at code above to see 
    
//     // Make sure the tile is not occupied with another piece; if it is: same color = not valid tile, different color = valid tile
//     // Try to do basic moves first like King, Rook, bishop, or queen, then Knight, then pawn (with double move if it's on its home rank)
//     // Then do castling and En passant last (text me with questions)

//     // I suggest creating a helper function so that you could do (bishop moves + rook moves == Queen Moves) or something like that
//     match piece.piece_type {
//         PieceType::King   =>  todo!(),
//         PieceType::Queen  =>  todo!(),
//         PieceType::Bishop =>  todo!(),
//         PieceType::Rook   =>  todo!(),
//         PieceType::Knight =>  todo!(),
//         PieceType::Pawn   =>  todo!(),
//     };
//     return to_return;
// }