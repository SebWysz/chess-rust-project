use bevy::prelude::*; 

#[derive(Resource)]
pub struct WhiteMove(pub bool);

#[derive(Resource)]
pub struct InCheck {
    pub black: bool,
    pub white: bool,
}

#[derive(Component)]
pub struct CurrentSelectedPiece;

#[derive(PartialEq, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
#[derive(Component)]
pub struct Redtile;

pub enum PieceType {
    Pawn,
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
}

pub enum PieceColour {
    White,
    Black,
}

#[derive(Component)]
pub struct Piece {
    pub colour: PieceColour,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(colour: PieceColour, piece_type: PieceType) -> Self {
        Piece {
            colour,
            piece_type,
        }
    }
}

impl PieceColour {
    pub fn is_white(&self) -> bool {
        match self {
            PieceColour::White => true,
            PieceColour::Black => false,
        }
    }
    pub fn is_different(&self, other: &PieceColour) -> bool {
        match (self, other) {
            (PieceColour::White, PieceColour::Black) => true,
            (PieceColour::Black, PieceColour::White) => true,
            _ => false,
        }
    }
}

impl PieceType {
    pub fn is_king(&self) -> bool {
        match self {
            PieceType::King => true,
            _ => false,
        }
    }
}
