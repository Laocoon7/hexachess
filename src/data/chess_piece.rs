use bevy::prelude::*;
use hexx::{algorithms::a_star, Hex};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Reflect, Component, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub enum ChessPiece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl ChessPiece {
    pub fn value(self) -> i32 {
        match self {
            ChessPiece::Pawn => 1,
            ChessPiece::Knight => 3,
            ChessPiece::Bishop => 3,
            ChessPiece::Rook => 5,
            ChessPiece::Queen => 9,
            ChessPiece::King => i32::MAX,
        }
    }

    pub fn index(self, white: bool) -> usize {
        let mut index = match self {
            ChessPiece::Pawn => 0,
            ChessPiece::Knight => 1,
            ChessPiece::Bishop => 2,
            ChessPiece::Rook => 3,
            ChessPiece::Queen => 4,
            ChessPiece::King => 5,
        };

        if !white {
            index += 6
        }

        index
    }

    pub fn successors(self, position: Hex, white: bool) -> Vec<Hex> {
        match self {
            ChessPiece::Pawn => {
                if white {
                    vec![position + Hex::new(0, 1)]
                } else {
                    vec![position + Hex::new(0, -1)]
                }
            }
            ChessPiece::Knight => vec![
                position + Hex::new(1, 2),
                position + Hex::new(2, 1),
                position + Hex::new(2, -1),
                position + Hex::new(1, -2),
                position + Hex::new(-1, -2),
                position + Hex::new(-2, -1),
                position + Hex::new(-2, 1),
                position + Hex::new(-1, 2),
            ],
            ChessPiece::Bishop => vec![
                position + Hex::new(1, 1),
                position + Hex::new(1, -1),
                position + Hex::new(-1, -1),
                position + Hex::new(-1, 1),
            ],
            ChessPiece::Rook => vec![
                position + Hex::new(1, 0),
                position + Hex::new(1, -1),
                position + Hex::new(0, 1),
                position + Hex::new(0, -1),
                position + Hex::new(-1, 0),
                position + Hex::new(-1, 1),
            ],
            ChessPiece::Queen => vec![
                position + Hex::new(1, 0),
                position + Hex::new(0, 1),
                position + Hex::new(-1, 0),
                position + Hex::new(0, -1),
                position + Hex::new(1, 1),
                position + Hex::new(1, -1),
                position + Hex::new(-1, -1),
                position + Hex::new(-1, 1),
            ],
            ChessPiece::King => vec![
                position + Hex::new(1, 0),
                position + Hex::new(0, 1),
                position + Hex::new(-1, 0),
                position + Hex::new(0, -1),
                position + Hex::new(1, 1),
                position + Hex::new(1, -1),
                position + Hex::new(-1, -1),
                position + Hex::new(-1, 1),
            ],
        }
    }

    // pub fn cost(self) -> Option<u32> {
    //     a_star(start, end, cost)
    // }
}
