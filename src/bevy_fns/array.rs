pub mod structs;


use bevy::prelude::{Vec2, Resource};
use structs::{Piece, PieceColour, PieceType};

use crate::bevy_fns::array;

#[derive(Resource, Clone)]
pub struct ArrayBoard {
    pub turn : PieceColour,
    pub in_check : Option<PieceColour>,
    pub board : Vec<Vec<Option<Piece>>>,
}

impl ArrayBoard {
    pub fn new() -> Self {
        ArrayBoard {
            turn: PieceColour::White,
            in_check: None,
            board: vec![
                vec![
                    Some(Piece::new(PieceColour::White, PieceType::Rook)),
                    Some(Piece::new(PieceColour::White, PieceType::Pawn)),
                    None,
                    None,
                    None,
                    None,
                    Some(Piece::new(PieceColour::Black, PieceType::Pawn)),
                    Some(Piece::new(PieceColour::Black, PieceType::Rook)),
                ],
                vec![
                    Some(Piece::new(PieceColour::White, PieceType::Knight)),
                    Some(Piece::new(PieceColour::White, PieceType::Pawn)),
                    None,
                    None,
                    None,
                    None,
                    Some(Piece::new(PieceColour::Black, PieceType::Pawn)),
                    Some(Piece::new(PieceColour::Black, PieceType::Knight)),
                ],
                vec![
                    Some(Piece::new(PieceColour::White, PieceType::Bishop)), 
                    Some(Piece::new(PieceColour::White, PieceType::Pawn)), 
                    None, 
                    None, 
                    None, 
                    None, 
                    Some(Piece::new(PieceColour::Black, PieceType::Pawn)), 
                    Some(Piece::new(PieceColour::Black, PieceType::Bishop))
                ],
                vec![
                    Some(Piece::new(PieceColour::White, PieceType::Queen)), 
                    Some(Piece::new(PieceColour::White, PieceType::Pawn)), 
                    None, 
                    None, 
                    None, 
                    None, 
                    Some(Piece::new(PieceColour::Black, PieceType::Pawn)), 
                    Some(Piece::new(PieceColour::Black, PieceType::Queen))
                ],
                vec![
                    Some(Piece::new(PieceColour::White, PieceType::King)), 
                    Some(Piece::new(PieceColour::White, PieceType::Pawn)), 
                    None, 
                    None, 
                    None, 
                    None, 
                    Some(Piece::new(PieceColour::Black, PieceType::Pawn)), 
                    Some(Piece::new(PieceColour::Black, PieceType::King))
                ],
                vec![
                    Some(Piece::new(PieceColour::White, PieceType::Bishop)), 
                    Some(Piece::new(PieceColour::White, PieceType::Pawn)), 
                    None, 
                    None, 
                    None, 
                    None, 
                    Some(Piece::new(PieceColour::Black, PieceType::Pawn)), 
                    Some(Piece::new(PieceColour::Black, PieceType::Bishop))
                ],
                vec![
                    Some(Piece::new(PieceColour::White, PieceType::Knight)),
                    Some(Piece::new(PieceColour::White, PieceType::Pawn)),
                    None,
                    None,
                    None,
                    None,
                    Some(Piece::new(PieceColour::Black, PieceType::Pawn)),
                    Some(Piece::new(PieceColour::Black, PieceType::Knight)),
                ],
                vec![
                    Some(Piece::new(PieceColour::White, PieceType::Rook)),
                    Some(Piece::new(PieceColour::White, PieceType::Pawn)),
                    None,
                    None,
                    None,
                    None,
                    Some(Piece::new(PieceColour::Black, PieceType::Pawn)),
                    Some(Piece::new(PieceColour::Black, PieceType::Rook)),
                ],
            ],
        }
    }
    pub fn swap_turn(&mut self) {
        if self.turn.is_white() {
            self.turn = PieceColour::Black;
        } else {
            self.turn = PieceColour::White;
        } 
    }
    pub fn move_piece(&mut self, (x_curr, y_curr) : (f32, f32), (x_new, y_new) : (f32, f32)) {
        self.board[x_new as usize][y_new as usize] = self.board[x_curr as usize][y_curr as usize].take();
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
    array_board: &ArrayBoard,
) -> Vec<Vec2> {
    let mut to_return = vec![];

    let curr_piece = array_board.board[x_curr as usize][y_curr as usize].as_ref().unwrap();

    for &(dx, dy) in directions {
        for i in 1..=max_distance {
            let x_new = x_curr + (i as f32) * dx;
            let y_new = y_curr + (i as f32) * dy;

            if x_new < 0. || x_new >= 8. || y_new < 0. || y_new >= 8. {
                break;
            }

            match array_board.board[x_new as usize][y_new as usize].as_ref() {
                Some(piece) if piece.colour.is_different(&curr_piece.colour) => {
                    to_return.push(Vec2::new(x_new, y_new));
                    break;
                },
                Some(_) => break,
                None => to_return.push(Vec2::new(x_new, y_new)),
            }
        }
    }

    to_return
}

pub fn valid_tiles(
    x_curr: f32,
    y_curr: f32,
    piece: &Piece,
    array_board: &ArrayBoard,
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
                valid_moves_for_directions(x_curr, y_curr, directions, 1, array_board);
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
                valid_moves_for_directions(x_curr, y_curr, directions, 7, array_board);
        }
        PieceType::Bishop => {
            let directions = &[(1., 1.), (1., -1.), (-1., 1.), (-1., -1.)];
            to_return =
                valid_moves_for_directions(x_curr, y_curr, directions, 7, array_board);
        }
        PieceType::Rook => {
            let directions = &[(1., 0.), (-1., 0.), (0., 1.), (0., -1.)];
            to_return =
                valid_moves_for_directions(x_curr, y_curr, directions, 7, array_board);
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
            let curr_piece = array_board.board[x_curr as usize][y_curr as usize].as_ref().unwrap();
            for &(dx, dy) in knight_moves {
                let x_new = x_curr + dx;
                let y_new = y_curr + dy;

                if x_new < 0. || x_new >= 8. || y_new < 0. || y_new >= 8. {
                    continue;
                }

                match array_board.board[x_new as usize][y_new as usize].as_ref() {
                    Some(piece) if piece.colour.is_different(&curr_piece.colour) => {
                        to_return.push(Vec2::new(x_new, y_new));
                        continue;
                    },
                    Some(_) => continue,
                    None => to_return.push(Vec2::new(x_new, y_new)),
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

            let curr_piece = array_board.board[x_curr as usize][y_curr as usize].as_ref().unwrap();
            let y_new = y_curr + direction;
            
            if y_new >= 0. && y_new < 8. {
                let positions = &[(x_curr, y_new), (x_curr + 1., y_new), (x_curr - 1., y_new)];
 
                for &(x_new, y_new) in positions {
                    if x_new < 0. || x_new >= 8. {
                        continue;
                    }
                    
                    match array_board.board[x_new as usize][y_new as usize].as_ref() {
                        Some(piece) if piece.colour.is_different(&curr_piece.colour) => {
                            if x_new == x_curr {  continue; }
                            to_return.push(Vec2::new(x_new, y_new));
                        },
                        Some(_) => (),
                        None => {
                            if x_new != x_curr { continue; }
                            to_return.push(Vec2::new(x_new, y_new));
                        },
                    };
                }
            }

            if y_curr == start_rank {
                let y_new = y_curr + 2. * direction;
                if y_new >= 0. && y_new < 8. { 
                    match array_board.board[x_curr as usize][y_new as usize] {
                        Some(_) => (),
                        None => {
                            if to_return.contains(&Vec2::new(x_curr, y_curr + direction)) {
                               to_return.push(Vec2::new(x_curr, y_new));
                            }
                        },
                    };    
                }
            }
        }
    };

    return to_return;
}

pub fn in_check_valid_tiles(
    x_curr: f32,
    y_curr: f32,
    piece: &Piece,
    array_board: &ArrayBoard,
) -> Vec<Vec2> {
        let mut to_return : Vec<Vec2> = vec![];
        let colour_in_check = array_board.in_check.unwrap(); 
        let mut king_tile : Vec2 = Vec2::new(-1., -1.);
        for (x, file) in array_board.board.iter().enumerate() {
            for (y, tile) in file.into_iter().enumerate() {
                if tile.is_some_and(|tile_piece| (!tile_piece.colour.is_different(&colour_in_check) && tile_piece.piece_type.is_king())) {
                    king_tile = Vec2::new(x as f32, y as f32);
                }
            }
        }
        for tile in valid_tiles(x_curr, y_curr, piece, array_board) {
            let mut temp_board = array_board.clone();
            temp_board.move_piece((x_curr, y_curr), (tile.x, tile.y));
            let king_tile2 = if array_board.board[x_curr as usize][y_curr as usize].unwrap().piece_type.is_king() { Vec2::new(tile.x as f32, tile.y as f32)} else { king_tile };
            //test piece moved by colour in check
            //see if a move can take colour in check's king
            let mut can_take_king = false;
            for (x, file) in temp_board.board.iter().enumerate() {
                for (y, curr_piece) in file.into_iter().enumerate() {
                    if curr_piece.is_none() { continue; }
                    if !curr_piece.as_ref().unwrap().colour.is_different(&colour_in_check) {
                        continue;
                    }
                    if valid_tiles(x as f32, y as f32, &curr_piece.unwrap(), &temp_board).contains(&king_tile2) {
                            can_take_king = true;
                    }
                }
            }
            if !can_take_king {
                to_return.push(Vec2::new(tile.x as f32, tile.y as f32));
            }
        }
        return to_return;
}
// this fn don't work