use bevy::prelude::*;
use hexx::Hex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Reflect, Component, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct ChessPiece(u8);

bitflags::bitflags! {
    impl ChessPiece: u8 {
        const PAWN      = 1;
        const KNIGHT    = 2;
        const BISHOP    = 3;
        const ROOK      = 4;
        const QUEEN     = 5;
        const KING      = 6;

        const WHITE     = 8;
        const BLACK     = 16;
    }
}

impl ChessPiece {
    pub fn pawn(black: bool) -> Self {
        if black {
            ChessPiece::PAWN | ChessPiece::BLACK
        } else {
            ChessPiece::PAWN | ChessPiece::WHITE
        }
    }
    pub fn knight(black: bool) -> Self {
        if black {
            ChessPiece::KNIGHT | ChessPiece::BLACK
        } else {
            ChessPiece::KNIGHT | ChessPiece::WHITE
        }
    }
    pub fn bishop(black: bool) -> Self {
        if black {
            ChessPiece::BISHOP | ChessPiece::BLACK
        } else {
            ChessPiece::BISHOP | ChessPiece::WHITE
        }
    }
    pub fn rook(black: bool) -> Self {
        if black {
            ChessPiece::ROOK | ChessPiece::BLACK
        } else {
            ChessPiece::ROOK | ChessPiece::WHITE
        }
    }
    pub fn queen(black: bool) -> Self {
        if black {
            ChessPiece::QUEEN | ChessPiece::BLACK
        } else {
            ChessPiece::QUEEN | ChessPiece::WHITE
        }
    }
    pub fn king(black: bool) -> Self {
        if black {
            ChessPiece::KING | ChessPiece::BLACK
        } else {
            ChessPiece::KING | ChessPiece::WHITE
        }
    }
}

impl ChessPiece {
    pub fn piece(self) -> Self {
        Self(self.0 & 0b0000_0111)
    }
    pub fn color(self) -> Self {
        Self(self.0 & 0b0001_1000)
    }
    pub fn value(self) -> i32 {
        if self.contains(ChessPiece::PAWN) {
            2
        } else if self.contains(ChessPiece::KNIGHT) {
            6
        } else if self.contains(ChessPiece::BISHOP) {
            7
        } else if self.contains(ChessPiece::ROOK) {
            10
        } else if self.contains(ChessPiece::QUEEN) {
            18
        } else if self.contains(ChessPiece::KING) {
            i32::MAX
        } else {
            0
        }
    }

    pub fn index(self) -> usize {
        let piece = self.piece();
        let mut index = if piece == ChessPiece::PAWN {
            0
        } else if piece == ChessPiece::KNIGHT {
            1
        } else if piece == ChessPiece::BISHOP {
            2
        } else if piece == ChessPiece::ROOK {
            3
        } else if piece == ChessPiece::QUEEN {
            4
        } else if piece == ChessPiece::KING {
            5
        } else {
            0
        };

        if self.contains(ChessPiece::BLACK) {
            index += 6
        }

        index
    }

    pub fn successors(self, position: Hex) -> Vec<Hex> {
        if self.contains(ChessPiece::PAWN) {
            if self.contains(ChessPiece::WHITE) {
                vec![position + Hex::new(0, 1)]
            } else {
                vec![position + Hex::new(0, -1)]
            }
        } else if self.contains(ChessPiece::KNIGHT) {
            vec![
                position + Hex::new(1, 2),
                position + Hex::new(2, 1),
                position + Hex::new(2, -1),
                position + Hex::new(1, -2),
                position + Hex::new(-1, -2),
                position + Hex::new(-2, -1),
                position + Hex::new(-2, 1),
                position + Hex::new(-1, 2),
            ]
        } else if self.contains(ChessPiece::BISHOP) {
            vec![
                position + Hex::new(1, 1),
                position + Hex::new(1, -1),
                position + Hex::new(-1, -1),
                position + Hex::new(-1, 1),
            ]
        } else if self.contains(ChessPiece::ROOK) {
            vec![
                position + Hex::new(1, 0),
                position + Hex::new(1, -1),
                position + Hex::new(0, 1),
                position + Hex::new(0, -1),
                position + Hex::new(-1, 0),
                position + Hex::new(-1, 1),
            ]
        } else if self.contains(ChessPiece::QUEEN) {
            vec![
                position + Hex::new(1, 0),
                position + Hex::new(0, 1),
                position + Hex::new(-1, 0),
                position + Hex::new(0, -1),
                position + Hex::new(1, 1),
                position + Hex::new(1, -1),
                position + Hex::new(-1, -1),
                position + Hex::new(-1, 1),
            ]
        } else if self.contains(ChessPiece::KING) {
            vec![
                position + Hex::new(1, 0),
                position + Hex::new(0, 1),
                position + Hex::new(-1, 0),
                position + Hex::new(0, -1),
                position + Hex::new(1, 1),
                position + Hex::new(1, -1),
                position + Hex::new(-1, -1),
                position + Hex::new(-1, 1),
            ]
        } else {
            Vec::new()
        }
    }
}
