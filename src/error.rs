use crate::{pieces::Color, moves::Move};

#[derive(Debug)]
pub enum Error {
    InvalidFen(String),
    PieceNotFoundAtCell(usize),
    InvalidPieceStr(String),

    InvalidMoveStr(String),

    InvalidMove(Move, Color, Color),
    
}