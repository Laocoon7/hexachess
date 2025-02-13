use crate::components::PieceType;

pub struct ChessMove {
    pub from: String,
    pub to: String,
    pub promotion: Option<PieceType>,
}

pub enum Turn {
    White,
    Black,
}

pub struct Game {
    pub turn: Turn,
    pub history: Vec<ChessMove>,
}