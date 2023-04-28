//Activate Clippy, helps with proper formatting
#![deny(clippy::all)] 

pub struct Board {
    pub board: Vec<Vec<Tile>>,
}

#[derive(PartialEq)]
pub enum PieceColour {
    White,
    Black,
}

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
pub struct Tile {
    pub has_piece: bool,
    pub colour: PieceColour,
    pub piece_type: PieceType,
}

struct Piece{
    piece_type: PieceType,
    colour: PieceColour,
}

struct Position{
    x: f32,
    y: f32,
}

