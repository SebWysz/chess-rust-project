fn setup_board(
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
