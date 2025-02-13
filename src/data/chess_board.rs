use bevy::{prelude::*, utils::HashMap};
use hexx::{shapes, Hex, HexLayout};

use crate::components::ChessPiece;

const GLINSKI_COLUMNS: [char; 11] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'K', 'L'];
const GLINSKI_RADIUS: u32 = 5;
const GLINSKI_COLUMN_OFFSET: i32 = GLINSKI_RADIUS as i32;
const GLINSKI_ROW_OFFSET: i32 = GLINSKI_RADIUS as i32 + 1;

#[derive(Reflect)]
pub struct ChessBoard {
    pub layout: HexLayout,
    pub pieces: HashMap<Hex, Option<ChessPiece>>,
}

impl Default for ChessBoard {
    fn default() -> Self {
        Self::new(0.98)
    }
}

impl ChessBoard {
    pub fn new(hex_size: f32) -> Self {
        let pieces: HashMap<Hex, Option<ChessPiece>> = shapes::hexagon(Hex::ZERO, GLINSKI_RADIUS)
            .map(|hex| (hex, None))
            .collect();

        let mut s = Self {
            layout: HexLayout::flat().with_hex_size(hex_size),
            pieces,
        };

        s.glinski_start();

        s
    }

    pub fn get_piece_notation(&self, notation: &str) -> Option<ChessPiece> {
        if let Some(hex) = Self::from_glinski_notation(notation) {
            self.get_piece(hex)
        } else {
            None
        }
    }

    pub fn set_piece_notation(&mut self, notation: &str, piece: Option<ChessPiece>) {
        if let Some(hex) = Self::from_glinski_notation(notation) {
            self.set_piece(hex, piece);
        }
    }
}

impl ChessBoard {
    pub fn from_glinski_notation(notation: &str) -> Option<Hex> {
        if notation.len() < 2 {
            return None;
        }

        let column_char = notation.chars().next().unwrap();
        let row_part = &notation[1..];
        let row: i32 = row_part.parse().ok()?;

        let column_index = GLINSKI_COLUMNS.iter().position(|&c| c == column_char)?;

        let x = column_index as i32 - GLINSKI_COLUMN_OFFSET;

        let y = if column_index < GLINSKI_COLUMN_OFFSET as usize {
            row - x - GLINSKI_ROW_OFFSET
        } else {
            row - GLINSKI_ROW_OFFSET
        };

        Some(Hex::new(x, y))
    }

    pub fn to_glinski_notation(hex: Hex) -> Option<String> {
        // Center column (F) corresponds to x = 0
        let column_index = (hex.x() + GLINSKI_COLUMN_OFFSET) as usize;

        if column_index >= GLINSKI_COLUMNS.len() {
            return None;
        }
        let column = GLINSKI_COLUMNS[column_index];

        let row = if column_index < GLINSKI_COLUMN_OFFSET as usize {
            hex.y() + hex.x() + GLINSKI_ROW_OFFSET
        } else {
            hex.y() + GLINSKI_ROW_OFFSET
        };

        if !(1..=11).contains(&row) {
            return None;
        }

        Some(format!("{}{}", column, row))
    }

    fn glinski_start(&mut self) {
        self.set_piece_notation("B1", Some(ChessPiece::pawn(false)));
        self.set_piece_notation("C2", Some(ChessPiece::pawn(false)));
        self.set_piece_notation("D3", Some(ChessPiece::pawn(false)));
        self.set_piece_notation("E4", Some(ChessPiece::pawn(false)));
        self.set_piece_notation("F5", Some(ChessPiece::pawn(false)));
        self.set_piece_notation("G4", Some(ChessPiece::pawn(false)));
        self.set_piece_notation("H3", Some(ChessPiece::pawn(false)));
        self.set_piece_notation("I2", Some(ChessPiece::pawn(false)));
        self.set_piece_notation("K1", Some(ChessPiece::pawn(false)));

        self.set_piece_notation("C1", Some(ChessPiece::rook(false)));
        self.set_piece_notation("D1", Some(ChessPiece::knight(false)));
        self.set_piece_notation("E1", Some(ChessPiece::queen(false)));
        self.set_piece_notation("F3", Some(ChessPiece::bishop(false)));
        self.set_piece_notation("F2", Some(ChessPiece::bishop(false)));
        self.set_piece_notation("F1", Some(ChessPiece::bishop(false)));
        self.set_piece_notation("G1", Some(ChessPiece::king(false)));
        self.set_piece_notation("H1", Some(ChessPiece::knight(false)));
        self.set_piece_notation("I1", Some(ChessPiece::rook(false)));

        self.set_piece_notation("B7", Some(ChessPiece::pawn(true)));
        self.set_piece_notation("C7", Some(ChessPiece::pawn(true)));
        self.set_piece_notation("D7", Some(ChessPiece::pawn(true)));
        self.set_piece_notation("E7", Some(ChessPiece::pawn(true)));
        self.set_piece_notation("F7", Some(ChessPiece::pawn(true)));
        self.set_piece_notation("G7", Some(ChessPiece::pawn(true)));
        self.set_piece_notation("H7", Some(ChessPiece::pawn(true)));
        self.set_piece_notation("I7", Some(ChessPiece::pawn(true)));
        self.set_piece_notation("K7", Some(ChessPiece::pawn(true)));

        self.set_piece_notation("C8", Some(ChessPiece::rook(true)));
        self.set_piece_notation("D9", Some(ChessPiece::knight(true)));
        self.set_piece_notation("E10", Some(ChessPiece::queen(true)));
        self.set_piece_notation("F9", Some(ChessPiece::bishop(true)));
        self.set_piece_notation("F10", Some(ChessPiece::bishop(true)));
        self.set_piece_notation("F11", Some(ChessPiece::bishop(true)));
        self.set_piece_notation("G10", Some(ChessPiece::king(true)));
        self.set_piece_notation("H9", Some(ChessPiece::knight(true)));
        self.set_piece_notation("I8", Some(ChessPiece::rook(true)));
    }
}

impl ChessBoard {
    pub fn get_piece(&self, hex: Hex) -> Option<ChessPiece> {
        self.pieces.get(&hex).copied().flatten()
    }

    pub fn set_piece(&mut self, hex: Hex, piece: Option<ChessPiece>) {
        if let Some(maybe_piece) = self.pieces.get_mut(&hex) {
            *maybe_piece = piece;
        }
    }

    pub fn move_piece(&mut self, from: Hex, to: Hex) {
        let piece = self.get_piece(from);
        self.set_piece(to, piece);
        self.set_piece(from, None);
    }
}
