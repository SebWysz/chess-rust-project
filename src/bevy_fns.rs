use bevy::{prelude::*, window::PrimaryWindow};

pub mod array;
use array::structs::{Piece, PieceColour, PieceType, Position, CurrentSelectedPiece, Redtile};

use self::array::{valid_tiles, ArrayBoard, in_check_valid_tiles, can_take_king};

const TILE_SIZE: f32 = 80.0;
const BOARD_SIZE: usize = 8;

pub fn setup_board(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
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

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: tile_colour,
                    rect: Some(Rect::new(0., 0., TILE_SIZE, TILE_SIZE)),
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
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    col as f32 * TILE_SIZE + horiz_displacement,
                    6. * TILE_SIZE + vert_displacement,
                    0.0,
                )
                .with_scale(Vec3::new(0.25, 0.25, 1.)),
                texture: asset_server.load("sprites/Bpawn.png"),
                ..default()
            },
            Piece {
                colour: PieceColour::Black,
                piece_type: PieceType::Pawn,
            },
            Position {
                x: col as f32,
                y: 6.,
            },
        ));
        //white pawns
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    col as f32 * TILE_SIZE + horiz_displacement,
                    1. * TILE_SIZE + vert_displacement,
                    0.0,
                )
                .with_scale(Vec3::new(0.25, 0.25, 1.)),
                texture: asset_server.load("sprites/Wpawn.png"),
                ..default()
            },
            Piece {
                colour: PieceColour::White,
                piece_type: PieceType::Pawn,
            },
            Position {
                x: col as f32,
                y: 1.,
            },
        ));
    }

    //kings
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                4. * TILE_SIZE + horiz_displacement,
                0. * TILE_SIZE + vert_displacement,
                0.0,
            )
            .with_scale(Vec3::new(0.25, 0.25, 1.)),
            texture: asset_server.load("sprites/Wking.png"),
            ..default()
        },
        Piece {
            colour: PieceColour::White,
            piece_type: PieceType::King,
        },
        Position { x: 4., y: 0. },
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                4. * TILE_SIZE + horiz_displacement,
                7. * TILE_SIZE + vert_displacement,
                0.0,
            )
            .with_scale(Vec3::new(0.25, 0.25, 1.)),
            texture: asset_server.load("sprites/Bking.png"),
            ..default()
        },
        Piece {
            colour: PieceColour::Black,
            piece_type: PieceType::King,
        },
        Position { x: 4., y: 7. },
    ));

    //queens
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                3. * TILE_SIZE + horiz_displacement,
                0. * TILE_SIZE + vert_displacement,
                0.0,
            )
            .with_scale(Vec3::new(0.25, 0.25, 1.)),
            texture: asset_server.load("sprites/Wqueen.png"),
            ..default()
        },
        Piece {
            colour: PieceColour::White,
            piece_type: PieceType::Queen,
        },
        Position { x: 3., y: 0. },
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                3. * TILE_SIZE + horiz_displacement,
                7. * TILE_SIZE + vert_displacement,
                0.0,
            )
            .with_scale(Vec3::new(0.25, 0.25, 1.)),
            texture: asset_server.load("sprites/Bqueen.png"),
            ..default()
        },
        Piece {
            colour: PieceColour::Black,
            piece_type: PieceType::Queen,
        },
        Position { x: 3., y: 7. },
    ));

    //Bishops
    for col in 0..2 {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    (col as f32 * 3. + 2.) * TILE_SIZE + horiz_displacement,
                    7. * TILE_SIZE + vert_displacement,
                    0.0,
                )
                .with_scale(Vec3::new(0.25, 0.25, 1.)),
                texture: asset_server.load("sprites/Bbishop.png"),
                ..default()
            },
            Piece {
                colour: PieceColour::Black,
                piece_type: PieceType::Bishop,
            },
            Position {
                x: (col as f32 * 3. + 2.),
                y: 7.,
            },
        ));
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    (col as f32 * 3. + 2.) * TILE_SIZE + horiz_displacement,
                    0. * TILE_SIZE + vert_displacement,
                    0.0,
                )
                .with_scale(Vec3::new(0.25, 0.25, 1.)),
                texture: asset_server.load("sprites/Wbishop.png"),
                ..default()
            },
            Piece {
                colour: PieceColour::White,
                piece_type: PieceType::Bishop,
            },
            Position {
                x: (col as f32 * 3. + 2.),
                y: 0.,
            },
        ));
    }

    // Knights
    for col in 0..2 {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    (col as f32 * 5. + 1.) * TILE_SIZE + horiz_displacement,
                    7. * TILE_SIZE + vert_displacement,
                    0.0,
                )
                .with_scale(Vec3::new(0.25, 0.25, 1.)),
                texture: asset_server.load("sprites/Bhorse.png"),
                ..default()
            },
            Piece {
                colour: PieceColour::Black,
                piece_type: PieceType::Knight,
            },
            Position {
                x: (col as f32 * 5. + 1.),
                y: 7.,
            },
        ));
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    (col as f32 * 5. + 1.) * TILE_SIZE + horiz_displacement,
                    0. * TILE_SIZE + vert_displacement,
                    0.0,
                )
                .with_scale(Vec3::new(0.25, 0.25, 1.)),
                texture: asset_server.load("sprites/Whorse.png"),
                ..default()
            },
            Piece {
                colour: PieceColour::White,
                piece_type: PieceType::Knight,
            },
            Position {
                x: (col as f32 * 5. + 1.),
                y: 0.,
            },
        ));
    }

    // Rooks
    for col in 0..2 {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    (col as f32 * 7.) * TILE_SIZE + horiz_displacement,
                    7. * TILE_SIZE + vert_displacement,
                    0.0,
                )
                .with_scale(Vec3::new(0.25, 0.25, 1.)),
                texture: asset_server.load("sprites/Brook.png"),
                ..default()
            },
            Piece {
                colour: PieceColour::Black,
                piece_type: PieceType::Rook,
            },
            Position {
                x: (col as f32 * 7.),
                y: 7.,
            },
        ));
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    (col as f32 * 7.) * TILE_SIZE + horiz_displacement,
                    0. * TILE_SIZE + vert_displacement,
                    0.0,
                )
                .with_scale(Vec3::new(0.25, 0.25, 1.)),
                texture: asset_server.load("sprites/Wrook.png"),
                ..default()
            },
            Piece {
                colour: PieceColour::White,
                piece_type: PieceType::Rook,
            },
            Position {
                x: (col as f32 * 7.),
                y: 0.,
            },
        ));
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn mouse_click_system(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut array_board: ResMut<ArrayBoard>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    piece_query: Query<(Entity, &mut Position, &Piece), Without<CurrentSelectedPiece>>,
    mut curr_piece_query: Query<(
        Entity,
        &mut Transform,
        &mut Position,
        &Piece,
        &CurrentSelectedPiece,
    )>,
    red_tiles: Query<Entity, With<Redtile>>,
) {
    // Check for 'Escape' key to unselect the current piece
    if keyboard_input.just_pressed(KeyCode::Escape) {
        //Escape
        deselect_current_piece(curr_piece_query, commands, red_tiles);
        return;
    }

    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let window = window_query.get_single().unwrap();
    let horiz_displacement = (window.width() / 2.) - (TILE_SIZE * 3.5);
    let vert_displacement = (window.height() / 2.) - (TILE_SIZE * 3.5);

    let mouse_tile = find_mouse_tile(window.cursor_position().unwrap(), window);

    // Deal with input not on the board by doing nothing
    if mouse_tile[0] >= BOARD_SIZE as f32
        || mouse_tile[0] < 0.
        || mouse_tile[1] >= BOARD_SIZE as f32 && mouse_tile[1] < 0.
    {
        return;
    }
    // if selected piece is picked up, set it down at the tile
    // if piece is already picked up/invalid move square, do nothing (todo! or error noise? blinking red or something)
    match curr_piece_query.get_single_mut() {
        // has piece picked up
        Ok((curr_entity, mut curr_trans, mut curr_pos, piece_qual, _curr_sel_piece)) => {
            let curr_valid_tiles = if array_board.in_check.is_some() { in_check_valid_tiles(curr_pos.x, curr_pos.y, piece_qual, &array_board) } 
                else { valid_tiles(curr_pos.x, curr_pos.y, piece_qual, &array_board, true) };
            if mouse_tile[0] == curr_pos.x && mouse_tile[1] == curr_pos.y || !curr_valid_tiles.contains(&mouse_tile) {
                deselect_current_piece(curr_piece_query, commands, red_tiles);
                return;
            }

            if (array_board.turn.is_white() && !piece_qual.colour.is_white())
                || (!array_board.turn.is_white() && piece_qual.colour.is_white())
            {
                return;
            }

            if !(curr_valid_tiles.contains(&mouse_tile)) {
                // insert error noise or blinking? to signal wrong move
                return;
            }
            // Then if it is a valid tile, move the piece there
            for (entity, position, _piece_qual) in piece_query.into_iter() {
                if position.x == mouse_tile[0] && position.y == mouse_tile[1] {
                    commands.entity(entity).despawn();
                }
            }

            array_board.move_piece((curr_pos.x, curr_pos.y), (mouse_tile[0], mouse_tile[1]));

            let direction = Vec3::new(
                (mouse_tile[0] - curr_pos.x) * TILE_SIZE,
                (mouse_tile[1] - curr_pos.y) * TILE_SIZE,
                0.,
            );
            curr_pos.x = mouse_tile[0];
            curr_pos.y = mouse_tile[1];
            curr_trans.translation += direction;
            commands
                .entity(curr_entity)
                .remove::<CurrentSelectedPiece>();
            if !red_tiles.is_empty() {
                for tile in red_tiles.into_iter() {
                    commands.entity(tile).despawn();
                }
            }
            array_board.swap_turn(); 
 
            array_board.in_check = None;
            if can_take_king((curr_pos.x, curr_pos.y), (curr_pos.x, curr_pos.y), &piece_qual.colour.opposite(), &array_board) {
                array_board.in_check = Some(piece_qual.colour.opposite());
            }
        }
        // no piece picked up
        Err(_) => {
            // if piece occupies the square, pick piece "up"
            for (entity, position, piece_qual) in piece_query.into_iter() {
                if position.x != mouse_tile[0] || position.y != mouse_tile[1] {
                    continue;
                }
                commands.entity(entity).insert(CurrentSelectedPiece);

                // Show tiles able to move onto 
                spawn_red_tile(
                    &mut commands,
                    mouse_tile[0],
                    mouse_tile[1],
                    horiz_displacement,
                    vert_displacement,
                );
                let curr_valid_tiles = {
                    if array_board.in_check.is_some() && !array_board.in_check.unwrap().is_different(&piece_qual.colour) { 
                        in_check_valid_tiles(position.x, position.y, piece_qual, &array_board) 
                    } else {
                        valid_tiles(position.x, position.y, piece_qual, &array_board, true) 
                }};
                for valid_pos in curr_valid_tiles {
                    spawn_red_tile(
                        &mut commands,
                        valid_pos.x,
                        valid_pos.y,
                        horiz_displacement,
                        vert_displacement,
                    );
                }
            }
        }
    }
} 


fn deselect_current_piece(
    mut curr_piece_query: Query<(
        Entity,
        &mut Transform,
        &mut Position,
        &Piece,
        &CurrentSelectedPiece,
    )>,
    mut commands: Commands,
    red_tiles: Query<Entity, With<Redtile>>,
) {
    match curr_piece_query.get_single_mut() {
        Ok((curr_entity, _curr_trans, _curr_pos, _piece_qual, _curr_sel_piece)) => {
            commands
                .entity(curr_entity)
                .remove::<CurrentSelectedPiece>();
            if !red_tiles.is_empty() {
                for tile in red_tiles.into_iter() {
                    commands.entity(tile).despawn();
                }
            }
        }
        Err(_) => (),
    }
} 

pub fn find_mouse_tile(input: Vec2, window: &Window) -> Vec2 {
    let horiz_displacement = window.width() / 2. - TILE_SIZE * 4.;
    let vert_displacement = window.height() / 2. - TILE_SIZE * 4.;
    Vec2::new(
        f32::floor((input[0] - horiz_displacement) / TILE_SIZE),
        f32::floor((input[1] - vert_displacement) / TILE_SIZE),
    )
    // return is 0 indexed!
}

pub fn spawn_red_tile(
    commands: &mut Commands,
    pos_x: f32,
    pos_y: f32,
    horiz_displacement: f32,
    vert_displacement: f32, 
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(244, 113, 116),
                rect: Some(Rect::new(0., 0., TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(
                (pos_x * TILE_SIZE) + horiz_displacement,
                (pos_y * TILE_SIZE) + vert_displacement,
                0.,
            ),
            ..default()
        },
        Redtile,
    ));
}