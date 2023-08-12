use std::fmt::Display;

use crate::{pieces::Piece, error::Error};

#[derive(Clone, Copy)]
pub(crate) struct Cell{
    idx: usize,
    pub(super)piece: Option<Piece>
}

impl Cell {
    fn new(idx: usize, piece: Option<Piece>) -> Self {
        Self {
            idx, 
            piece
        }
    }

    fn is_occupied(&self) -> bool{
        self.piece.is_some()
    }

    fn place_piece(&mut self, piece: Option<Piece>){
        self.piece = piece;
    }

    fn pop_piece(&mut self) -> Result<Piece, Error>{

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

pub struct GameBoard {
    pub(super)cells: Vec<Cell>
}

impl GameBoard {
    pub(crate) fn new() -> Self {
        let cells = (0..64).map(|idx| Cell {idx, piece:None}).collect::<Vec<_>>();
        Self {
            cells
        }
    }

    pub(crate) fn place_piece(&mut self, piece: Piece, idx: usize) -> Option<Piece>{
        let cell = &mut self.cells[idx];
        let mut p = None;
        if cell.is_occupied() {
            p = Some(cell.pop_piece().unwrap());
        }
        cell.place_piece(Some(piece));
        p
    }

    pub(crate) fn remove_piece(&mut self, idx: usize) -> Result<Piece, Error>{
        let cell = &mut self.cells[idx];
        cell.pop_piece()

    }
}

impl Display for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cells.chunks(8).for_each(|c|{
            let _ = f.write_str("-----------------------------------------\n");
            for cell in c {
                let c = match cell.piece {
                    Some(p) => format!(" {} ", p), 
                    None => "   ".to_owned()
                };
                let _ = f.write_fmt(format_args!("|{}|", c));
            }
            let _ = f.write_str("\n-----------------------------------------\n");

        });

        Ok(())
    }
}

#[cfg(test)]
mod board_tests {
    use crate::pieces::{PieceType, Color};

    use super::*;
    #[test]
    fn test_board_creation(){
        let test_board = GameBoard::new();

        assert_eq!(test_board.cells.len(), 64);
    }

    #[test]
    fn test_correct_assignment(){
        let test_board = GameBoard::new();
        assert!(test_board.cells.get(0).is_some());
        assert_eq!(test_board.cells.get(0).unwrap().idx, 0);
        assert_eq!(test_board.cells.get(16).unwrap().idx, 16);
        assert_eq!(test_board.cells.get(63).unwrap().idx, 63);
        assert!(test_board.cells.get(64).is_none());
    }


    #[test]
    fn test_placing_piece() {
        let mut board = GameBoard::new();
        let p_1 = board.place_piece(Piece::new(PieceType::King, Color::Black), 10);
        assert!(p_1.is_none());

        // Test placing
        let cell = &board.cells[10];
        assert!(cell.piece.is_some());
        assert_eq!(cell.piece.unwrap().piece_type() , PieceType::King );
        assert_eq!(cell.piece.unwrap().color(), Color::Black);
        
        // Testing placing with replacements
        let p_2 = board.place_piece(Piece::new(PieceType::Bishop, Color::Black), 10);
        assert!(p_2.is_some());
        assert_eq!(p_2.unwrap().piece_type() , PieceType::King );
        assert_eq!(p_2.unwrap().color(), Color::Black);
    }

    #[test] 
    fn test_removing_piece(){
        let mut board = GameBoard::new();
        board.cells[10] = Cell::new(10, Some(Piece::new(PieceType::King, Color::Black)));
        assert!(board.cells[11].piece.is_none());
        assert!(board.remove_piece(11).is_err());
        let piece = board.remove_piece(10);
        assert!(piece.is_ok());
        let piece = piece.unwrap();
        assert_eq!(piece.piece_type(), PieceType::King );
        assert_eq!(piece.color(), Color::Black);


        let cell = &board.cells[10];
        assert!(cell.piece.is_none());
    }
}