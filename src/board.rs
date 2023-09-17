use std::{fmt::Display, str::FromStr};

use crate::{
    pieces::{Piece, self},
    error::Error,
    moves::Move,
    cell::Cell
};

pub struct GameBoard {
    pub(super)cells: Vec<Box<Cell>>
}

impl GameBoard {
    pub(super) fn new() -> Self {
        let cells = (0..64).map(|idx| Box::new(Cell::new(idx, None))).collect::<Vec<_>>();
        Self {
            cells
        }
    }

    pub(super) fn build_board_from_fen(fen_slice: &str) -> Self {
        let mut board = GameBoard::new();
        let fen = fen_slice.split('/')
            .map(|rank|{
                let mut expanded = String::new();
                for c in rank.chars(){
                    if c.is_alphabetic() {
                        expanded += c.to_string().as_str();
                    } else {
                        expanded += &"1".repeat(c.to_digit(10).unwrap().try_into().unwrap())
                    }
                }
                expanded
            }).collect::<Vec<String>>();
        
    
    
        fen.iter().enumerate().for_each(|(i, rank) | {
            rank.chars().enumerate().for_each(|(j, c)| {
    
                board.cells[i*8+j].piece = if c.is_alphabetic() {
                    Some( Piece::from_str(c.to_string().as_str()).expect("Invalid String for a piece"))
                } else {
                    None
                };
    
            })
        });
    
        board
    }
    
    

    pub(super) fn place_piece(&mut self, piece: Piece, idx: usize) -> Option<Piece>{
        let cell = &mut self.cells[idx];
        let mut p = None;
        if cell.is_occupied() {
            p = Some(cell.pop_piece().unwrap());
        }
        cell.place_piece(Some(piece));
        p
    }

    pub(super) fn apply_move(&mut self, game_move: Move, curr_player: pieces::Color) -> Result<(), Error> {

        // Check for the presence of piece in the cell
        let p = if let Some(p )= self.cells[game_move.from as usize].piece{
            p.color()
        } else {
           return Err(Error::PieceNotFoundAtCell(game_move.from.into()))
        }; 

        // Check if the piece color matches the current player
        if p != curr_player {
            return Err(Error::InvalidMove(game_move, curr_player, p))
        }

        // TODO: Check for piece-wise valid moves

        // Move
        let piece = self.remove_piece(game_move.from.into())?;
        self.place_piece(piece, game_move.to as usize);
        Ok(())

    }


    pub(super) fn remove_piece(&mut self, idx: usize) -> Result<Piece, Error>{
        let cell = &mut self.cells[idx];
        cell.pop_piece()

    }

    pub(super) fn get_fen_slice(&self) -> &str {
        let mut s = String::new();
        let mut counter = 0;
        for cell  in &self.cells {
            if cell.is_occupied() {
                if counter > 0 {s+=counter.to_string().as_str()}
                s += &cell.piece.unwrap().to_string();
            }else {
                counter += 1
            }
        }

        todo!()
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
        *board.cells[10] = Cell::new(10, Some(Piece::new(PieceType::King, Color::Black)));
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