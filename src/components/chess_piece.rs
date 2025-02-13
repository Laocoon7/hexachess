use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Serialize, Deserialize, Reflect, Component, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub enum ChessPiece {
    White(PieceType),
    Black(PieceType),
}

impl ChessPiece {
    pub fn pawn(black: bool) -> Self {
        if black {
            ChessPiece::Black(PieceType::Pawn)
        } else {
            ChessPiece::White(PieceType::Pawn)
        }
    }
    pub fn knight(black: bool) -> Self {
        if black {
            ChessPiece::Black(PieceType::Knight)
        } else {
            ChessPiece::White(PieceType::Knight)
        }
    }
    pub fn bishop(black: bool) -> Self {
        if black {
            ChessPiece::Black(PieceType::Bishop)
        } else {
            ChessPiece::White(PieceType::Bishop)
        }
    }
    pub fn rook(black: bool) -> Self {
        if black {
            ChessPiece::Black(PieceType::Rook)
        } else {
            ChessPiece::White(PieceType::Rook)
        }
    }
    pub fn queen(black: bool) -> Self {
        if black {
            ChessPiece::Black(PieceType::Queen)
        } else {
            ChessPiece::White(PieceType::Queen)
        }
    }
    pub fn king(black: bool) -> Self {
        if black {
            ChessPiece::Black(PieceType::King)
        } else {
            ChessPiece::White(PieceType::King)
        }
    }
}

impl ChessPiece {
    pub fn piece_type(self) -> PieceType {
        match self {
            ChessPiece::White(piece) => piece,
            ChessPiece::Black(piece) => piece,
        }
    }
    pub fn is_black(self) -> bool {
        matches!(self, ChessPiece::Black(_))
    }
    pub fn is_white(self) -> bool {
        matches!(self, ChessPiece::White(_))
    }
    pub fn value(self) -> i32 {
        match self.piece_type() {
            PieceType::Pawn => 2,
            PieceType::Knight => 6,
            PieceType::Bishop => 7,
            PieceType::Rook => 10,
            PieceType::Queen => 18,
            PieceType::King => i32::MAX,
        }
    }
    pub fn image_index(self) -> usize {
        let index = match self.piece_type() {
            PieceType::Pawn => 0,
            PieceType::Knight => 1,
            PieceType::Bishop => 2,
            PieceType::Rook => 3,
            PieceType::Queen => 4,
            PieceType::King => 5,
        };

        if self.is_black() {
            index + 6
        } else {
            index
        }
    }
}
