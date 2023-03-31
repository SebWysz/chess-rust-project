//Activate Clippy, helps with proper formatting
#![deny(clippy::all)] 

pub struct Board {
    pub board: Vec<Vec<Tile>>,
}

pub enum Colour {
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
    pub colour: Colour,
    pub piece_type: PieceType,
}