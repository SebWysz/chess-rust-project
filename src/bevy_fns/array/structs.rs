use bevy::prelude::*; 

#[derive(Component)]
pub struct CurrentSelectedPiece;

#[derive(PartialEq, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
#[derive(Component)]
pub struct Redtile;

#[derive(Clone, Copy)]
pub enum PieceType {
    Pawn,
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
}

#[derive(Clone, Copy, Debug)]
pub enum PieceColour {
    White,
    Black,
}

#[derive(Component, Clone, Copy)]
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
    pub fn opposite(&self) -> PieceColour {
        match self {
            PieceColour::White => PieceColour::Black,
            PieceColour::Black => PieceColour::White,
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
