//Activate Clippy, helps with proper formatting
#![deny(clippy::all)]

use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(setup_board)
        .add_system(mouse_click_system)
        .insert_resource(WhiteMove(true))
        .run();
}

const TILE_SIZE: f32 = 80.0;
const BOARD_SIZE: usize = 8;

#[derive(Resource)]
struct WhiteMove(bool);

#[derive(Component)]
struct CurrentSelectedPiece;

#[derive(Component)]
struct Piece {
    colour : PieceColour,
    piece_type : PieceType,
}

enum PieceColour {
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
            let tile_colour = if (row + col) % 2 == 0 {
                Color::rgb_u8(150, 77, 34)
            } else {
                Color::rgb_u8(218, 217, 181)
            };

            commands.spawn(SpriteBundle{
                sprite: Sprite{
                    color: tile_colour,
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
        Piece{colour : PieceColour::Black, piece_type : PieceType::Pawn},
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
        Piece{colour : PieceColour::White, piece_type : PieceType::Pawn},
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
        Piece{colour : PieceColour::White, piece_type : PieceType::King},
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
        Piece{colour : PieceColour::Black, piece_type : PieceType::King},
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
        Piece{colour : PieceColour::White, piece_type : PieceType::Queen},
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
        Piece{colour : PieceColour::Black, piece_type : PieceType::Queen},
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
            Piece{colour : PieceColour::Black, piece_type : PieceType::Bishop},
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
            Piece{colour : PieceColour::White, piece_type : PieceType::Bishop},
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
            Piece{colour : PieceColour::Black, piece_type : PieceType::Knight},
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
            Piece{colour : PieceColour::White, piece_type : PieceType::Knight},
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
            Piece{colour : PieceColour::Black, piece_type : PieceType::Rook},
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
            Piece{colour : PieceColour::White, piece_type : PieceType::Rook},
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
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    white_move : ResMut<WhiteMove>, 
    window_query: Query<&Window, With<PrimaryWindow>>,
    piece_query: Query<(Entity, &mut Position, &Piece), Without<CurrentSelectedPiece>>,
    mut curr_piece_query: Query<(Entity, &mut Transform, &mut Position, &Piece, &CurrentSelectedPiece)>,
    mut commands: Commands,
    red_tiles: Query<Entity, With<Redtile>>,
) {
    // Check for 'Escape' key to unselect the current piece
    if keyboard_input.just_pressed(KeyCode::Escape) {
        unselect_current_piece(curr_piece_query, commands, red_tiles);
        return;
    }

    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }
    
    let window = window_query.get_single().unwrap();
    let horiz_displacement = window.width() / 2. - TILE_SIZE * 3.5;
    let vert_displacement = window.height() / 2. - TILE_SIZE * 3.5;
    
    let mouse_tile = find_mouse_tile(window.cursor_position().unwrap(), window);
    
    // Deal with input not on the board by doing nothing
    if mouse_tile[0] >= BOARD_SIZE as f32 || mouse_tile[0] < 0. || mouse_tile[1] >= BOARD_SIZE as f32 && mouse_tile[1] < 0. {
        return;
    }
    // if selected piece is picked up, set it down at the tile
    // if piece is already picked up/invalid move square, do nothing (todo! or error noise? blinking red or something) 
    match curr_piece_query.get_single_mut() {
        // has piece picked up
        Ok((curr_entity, mut curr_trans, mut curr_pos, piece_qual, _curr_sel_piece)) => {
            if mouse_tile[0] == curr_pos.x && mouse_tile[1] == curr_pos.y {
                unselect_current_piece(curr_piece_query, commands, red_tiles);
                return;
            }

            if (white_move.0 == true && piece_qual.colour.is_different(&PieceColour::White)) 
                || (white_move.0 == false && piece_qual.colour.is_different(&PieceColour::Black)) {
                return;
            }
            
            if !(valid_tiles(curr_pos.x, curr_pos.y, piece_qual, &piece_query).contains(&mouse_tile)) {
                // insert error noise or blinking? to signal wrong move
                return;
            }
            // Then if it is a valid tile, move the piece there
            for (entity, position, _piece_qual) in piece_query.into_iter() {
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
            if white_move.0 {
                commands.insert_resource(WhiteMove(false));
            } else {
                commands.insert_resource(WhiteMove(true));
            }
        },
        // no piece picked up
        Err(_) => 
            // if piece occupies the square, pick piece "up"
            for (entity, position, piece_qual) in piece_query.into_iter() {
                if position.x != mouse_tile[0] || position.y != mouse_tile[1] { 
                    continue; 
                }
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
                for pos in valid_tiles(position.x, position.y, piece_qual, &piece_query) {
                    commands.spawn((
                        SpriteBundle{
                            sprite: Sprite{
                                color: Color::rgb_u8(244, 113, 116),
                                rect: Some(Rect::new(0.,0.,TILE_SIZE,TILE_SIZE)),
                                ..default()
                            },
                            transform: Transform::from_xyz(
                                pos.x * TILE_SIZE + horiz_displacement,
                                pos.y * TILE_SIZE + vert_displacement,
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

pub fn find_mouse_tile(input : Vec2, window : &Window) -> Vec2 {
    let horiz_displacement = window.width() / 2. - TILE_SIZE * 4.;
    let vert_displacement = window.height() / 2. - TILE_SIZE * 4.;
    Vec2::new(f32::floor((input[0] - horiz_displacement) / TILE_SIZE), 
              f32::floor((input[1] - vert_displacement) / TILE_SIZE))
    // return is 0 indexed!
}

// also have to somehow check that the king is not in check --- TBD
impl PieceColour {
    fn is_different(&self, other: &PieceColour) -> bool {
        match (self, other) {
            (PieceColour::White, PieceColour::Black) => true,
            (PieceColour::Black, PieceColour::White) => true,
            _ => false,
        }
    }
}
fn valid_moves_for_directions(
    x_curr: f32,
    y_curr: f32,
    directions: &[(f32, f32)],
    max_distance: usize,
    piece: &Piece,
    piece_query: &Query<(Entity, &mut Position, &Piece), Without<CurrentSelectedPiece>>,
) -> Vec<Vec2> {
    let mut to_return = vec![];

    for &(dx, dy) in directions {
        for i in 1..=max_distance {
            let x_new = x_curr + (i as f32) * dx;
            let y_new = y_curr + (i as f32) * dy;

            if x_new < 0. || x_new >= 8. || y_new < 0. || y_new >= 8. {
                break;
            }

            let mut piece_at_tile = false;
            for (_ent, pos, parse_piece) in piece_query.iter() {
                if pos.x != x_new || pos.y != y_new {
                    continue;
                }

                if piece.colour.is_different(&parse_piece.colour) {
                    to_return.push(Vec2::new(x_new, y_new));
                }
                piece_at_tile = true;
                break;
            }

            if !piece_at_tile {
                to_return.push(Vec2::new(x_new, y_new));
            } else {
                break;
            }
        }
    }

    to_return
}

fn valid_tiles(
    x_curr: f32,
    y_curr: f32,
    piece: &Piece,
    piece_query: &Query<(Entity, &mut Position, &Piece), Without<CurrentSelectedPiece>>,
) -> Vec<Vec2> {
    let mut to_return: Vec<Vec2> = vec![];

    match piece.piece_type {
        PieceType::King => {
            let directions = &[
                (1., 0.),
                (-1., 0.),
                (0., 1.),
                (0., -1.),
                (1., 1.),
                (1., -1.),
                (-1., 1.),
                (-1., -1.),
            ];
            to_return = valid_moves_for_directions(x_curr, y_curr, directions, 1, piece, piece_query);
        }
        PieceType::Queen => {
            let directions = &[
                (1., 0.),
                (-1., 0.),
                (0., 1.),
                (0., -1.),
                (1., 1.),
                (1., -1.),
                (-1., 1.),
                (-1., -1.),
            ];
            to_return = valid_moves_for_directions(x_curr, y_curr, directions, 7, piece, piece_query);
        }
        PieceType::Bishop => {
            let directions = &[(1., 1.), (1., -1.), (-1., 1.), (-1., -1.)];
            to_return = valid_moves_for_directions(x_curr, y_curr, directions, 7, piece, piece_query);
        }
        PieceType::Rook => {
            let directions = &[(1., 0.), (-1., 0.), (0., 1.), (0., -1.)];
            to_return = valid_moves_for_directions(x_curr, y_curr, directions, 7, piece, piece_query);
        }
        PieceType::Knight => {
            let knight_moves = &[
                (2., 1.),
                (2., -1.),
                (1., 2.),
                (1., -2.),
                (-2., 1.),
                (-2., -1.),
                (-1., 2.),
                (-1., -2.),
            ];

            for &(dx, dy) in knight_moves {
                let x_new = x_curr + dx;
                let y_new = y_curr + dy;

                if x_new < 0. || x_new >= 8. || y_new < 0. || y_new >= 8. {
                    continue;
                }

                let mut piece_at_tile = false;
                for (_ent, pos, parse_piece) in piece_query.iter() {
                    if pos.x != x_new || pos.y != y_new {
                        continue;
                    }

                    if piece.colour.is_different(&parse_piece.colour) {
                        to_return.push(Vec2::new(x_new, y_new));
                    }
                    piece_at_tile = true;
                    break;
                }

                if !piece_at_tile {
                    to_return.push(Vec2::new(x_new, y_new));
                }
            }
        }
        PieceType::Pawn => {
            let direction;
            let start_rank;
            match piece.colour {
                PieceColour::White => {
                    direction = 1.;
                    start_rank = 1.;
                }
                PieceColour::Black => {
                    direction = -1.;
                    start_rank = 6.;
                }
            }

            let y_new = y_curr + direction;
            if y_new >= 0. && y_new < 8. {
                let positions = &[(x_curr, y_new), (x_curr + 1., y_new), (x_curr - 1., y_new)];

                for &(x_new, y_new) in positions {
                    if x_new < 0. || x_new >= 8. {
                        continue;
                    }

                    let mut piece_at_tile = false;
                    for (_ent, pos, parse_piece) in piece_query.iter() {
                        if pos.x != x_new || pos.y != y_new {
                            continue;
                        }

                        if piece.colour.is_different(&parse_piece.colour) {
                            piece_at_tile = true;
                        } else {
                            to_return.push(Vec2::new(x_new, y_new));
                        }
                        break;
                    }

                    if !piece_at_tile && x_new == x_curr {
                        to_return.push(Vec2::new(x_new, y_new));
                    }
                }
            }

            if y_curr == start_rank {
                let y_new = y_curr + 2. * direction;
                if y_new >= 0. && y_new < 8. {
                    let mut piece_at_tile = false;
                    for (_ent, pos, _parse_piece) in piece_query.iter() {
                        if pos.x != x_curr || pos.y != y_new {
                            continue;
                        }
                        piece_at_tile = true;
                        break;
                    }
                    if !piece_at_tile {
                        to_return.push(Vec2::new(x_curr, y_new));
                    }
                }
            }
        }
    };

    return to_return;
}
fn unselect_current_piece(
    mut curr_piece_query: Query<(Entity, &mut Transform, &mut Position, &Piece, &CurrentSelectedPiece)>,
    mut commands: Commands,
    red_tiles: Query<Entity, With<Redtile>>,
) {
    match curr_piece_query.get_single_mut() {
        Ok((curr_entity, _curr_trans, _curr_pos, _piece_qual, _curr_sel_piece)) => {
            commands.entity(curr_entity).remove::<CurrentSelectedPiece>();
            if !red_tiles.is_empty() {
                for tile in red_tiles.into_iter() {
                    commands.entity(tile).despawn();
                }
            }
        },
        Err(_) => ()
    }
}