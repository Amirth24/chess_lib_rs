use crate::{
    pieces::Piece,
    error::Error,
};


#[derive(Clone, Copy)]
pub(crate) struct Cell{
    pub(super)idx: usize,
    pub(crate)piece: Option<Piece>
}

impl Cell {
    pub fn new(idx: usize, piece: Option<Piece>) -> Self {
        Self {
            idx, 
            piece
        }
    }

    pub fn is_occupied(&self) -> bool{
        self.piece.is_some()
    }

    pub fn place_piece(&mut self, piece: Option<Piece>){
        self.piece = piece;
    }

    pub fn pop_piece(&mut self) -> Result<Piece, Error>{

        match self.piece {
            Some(p) => {
                let piece = p;
                self.piece = None;
                Ok(piece)
            },
            None => Err(Error::PieceNotFoundAtCell(self.idx))
        }

    }
}
