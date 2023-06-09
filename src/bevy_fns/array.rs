pub mod structs;
use bevy::prelude::Vec2;
use structs::{Piece, PieceColour, PieceType};

use std::collections::HashMap;

struct ArrayBoard {
    board : Vec<Vec<Option<Piece>>>,
    turn : PieceColour,
}

impl ArrayBoard {
    fn new() -> Self {
        ArrayBoard {
            board: vec![
                vec![
                    Some(Piece::new(PieceColour::White, PieceType::Rook)),
                    Some(Piece::new(PieceColour::White, PieceType::Knight)),
                    Some(Piece::new(PieceColour::White, PieceType::Bishop)),
                    Some(Piece::new(PieceColour::White, PieceType::Queen)),
                    Some(Piece::new(PieceColour::White, PieceType::King)),
                    Some(Piece::new(PieceColour::White, PieceType::Bishop)),
                    Some(Piece::new(PieceColour::White, PieceType::Knight)),
                    Some(Piece::new(PieceColour::White, PieceType::Rook)),
                ],
                vec![
                    Some(Piece::new(PieceColour::White, PieceType::Pawn)),
                    Some(Piece::new(PieceColour::White, PieceType::Pawn)),
                    Some(Piece::new(PieceColour::White, PieceType::Pawn)),
                    Some(Piece::new(PieceColour::White, PieceType::Pawn)),
                    Some(Piece::new(PieceColour::White, PieceType::Pawn)),
                    Some(Piece::new(PieceColour::White, PieceType::Pawn)),
                    Some(Piece::new(PieceColour::White, PieceType::Pawn)),
                    Some(Piece::new(PieceColour::White, PieceType::Pawn)),
                ],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![
                    Some(Piece::new(PieceColour::Black, PieceType::Pawn)),
                    Some(Piece::new(PieceColour::Black, PieceType::Pawn)),
                    Some(Piece::new(PieceColour::Black, PieceType::Pawn)),
                    Some(Piece::new(PieceColour::Black, PieceType::Pawn)),
                    Some(Piece::new(PieceColour::Black, PieceType::Pawn)),
                    Some(Piece::new(PieceColour::Black, PieceType::Pawn)),
                    Some(Piece::new(PieceColour::Black, PieceType::Pawn)),
                    Some(Piece::new(PieceColour::Black, PieceType::Pawn)),
                ],
                vec![
                    Some(Piece::new(PieceColour::Black, PieceType::Rook)),
                    Some(Piece::new(PieceColour::Black, PieceType::Knight)),
                    Some(Piece::new(PieceColour::Black, PieceType::Bishop)),
                    Some(Piece::new(PieceColour::Black, PieceType::Queen)),
                    Some(Piece::new(PieceColour::Black, PieceType::King)),
                    Some(Piece::new(PieceColour::Black, PieceType::Bishop)),
                    Some(Piece::new(PieceColour::Black, PieceType::Knight)),
                    Some(Piece::new(PieceColour::Black, PieceType::Rook)),
                ],
            ],
            turn: PieceColour::White
        }
    }
}

impl Default for ArrayBoard {
    fn default() -> Self {
        Self::new()
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

    // Create a hashmap where key is the position and value is the color of the piece at that position
    let mut positions = HashMap::new();
    for (_ent, pos, piece) in piece_query.iter() {
        positions.insert((pos.x as usize, pos.y as usize), &piece.colour);
    }

    for &(dx, dy) in directions {
        for i in 1..=max_distance {
            let x_new = x_curr + (i as f32) * dx;
            let y_new = y_curr + (i as f32) * dy;

            if x_new < 0. || x_new >= 8. || y_new < 0. || y_new >= 8. {
                break;
            }

            let x_new_usize = x_new as usize;
            let y_new_usize = y_new as usize;

            match positions.get(&(x_new_usize, y_new_usize)) {
                Some(colour) if *colour == &piece.colour => {
                    // If there is a piece of the same color at the position, stop checking in this direction
                    break;
                }
                Some(_) => {
                    // If there is a piece of a different color at the position, add it to valid moves
                    to_return.push(Vec2::new(x_new, y_new));
                    break;
                }
                None => {
                    // If there is no piece at the position, add it to valid moves
                    to_return.push(Vec2::new(x_new, y_new));
                }
            }
        }
    }

    to_return
}

pub fn valid_tiles(
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
            to_return =
                valid_moves_for_directions(x_curr, y_curr, directions, 1, piece, piece_query);
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
            to_return =
                valid_moves_for_directions(x_curr, y_curr, directions, 7, piece, piece_query);
        }
        PieceType::Bishop => {
            let directions = &[(1., 1.), (1., -1.), (-1., 1.), (-1., -1.)];
            to_return =
                valid_moves_for_directions(x_curr, y_curr, directions, 7, piece, piece_query);
        }
        PieceType::Rook => {
            let directions = &[(1., 0.), (-1., 0.), (0., 1.), (0., -1.)];
            to_return =
                valid_moves_for_directions(x_curr, y_curr, directions, 7, piece, piece_query);
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
                        // Check for diagonal capture moves
                        if x_new != x_curr && piece.colour.is_different(&parse_piece.colour) {
                            to_return.push(Vec2::new(x_new, y_new));
                        }
                        piece_at_tile = true;
                        break;
                    }

                    // Check for vertical non-capture moves
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
