use std::str::FromStr;

use crate::{board::GameBoard, pieces::Piece};

pub struct Game {
    board: GameBoard,
    fen_str: String,
}

impl Game{
    pub fn new() -> Self {
        Self::load("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()).unwrap()
    }

    fn load(fen_str: String) -> Result<Self, crate::error::Error> {    
        
        if ! is_valid_fen(&fen_str) {
            return Err(crate::error::Error::InvalidFen(fen_str));
        }
        Ok(Self {
            board:build_board_from_fen(&fen_str),
            fen_str: fen_str.to_string()
        })
    }

    pub fn get_fen(&self) -> &str {
        &self.fen_str
    }
}

fn build_board_from_fen(fen: &str) -> GameBoard {
    let mut board = GameBoard::new();
    let fen = fen.split_ascii_whitespace()
        .next().unwrap()
        .split('/')
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
    


    for (i, rank) in fen.iter().enumerate() {
        rank.chars().enumerate().for_each(|(j, c)| {

            board.cells[i*8+j].piece = if c.is_alphabetic() {
                Some( Piece::from_str(c.to_string().as_str()).expect("Invalid String for a piece"))
            } else {
                None
            };

        })
    }

    return board;
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        Self::load(value.into()).expect("Failed to create Game")
    }
}

fn is_valid_fen(fen_str: &str) -> bool{
    fen_str.chars().filter(|&c| c == '/').count() == 7 && 
    fen_str.split(' ').count() == 6 && 
    !fen_str.starts_with('/') 
}

#[cfg(test)]
mod game_tests{
    use super::*;

    #[test]
    fn test_game_created(){
        let mut game = Game::new();
        assert_eq!(game.get_fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(game.board.remove_piece(10).unwrap(), Piece::from_str("p").unwrap());
 
    }
    #[test]
    fn test_game_loaded(){
        let game = Game::from("r6k/2R5/6R1/pp1Ppp2/8/Pn2B1Pr/4KP2/8 w – – 0 1");
        println!("{}", game.board);

        assert_eq!(game.get_fen(), r"r6k/2R5/6R1/pp1Ppp2/8/Pn2B1Pr/4KP2/8 w – – 0 1")
    }

    #[test]
    fn test_for_valid_fen() {
        assert!(is_valid_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
    }
    #[test]
    fn test_for_invalid_fen(){
        assert!(!is_valid_fen("not a fen"));
    }
}