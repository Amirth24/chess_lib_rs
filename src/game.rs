use std::{rc::Rc, cell::RefCell, str::FromStr};

use crate::{board::GameBoard, pieces::Color, player::Player, error::Error, moves};

pub struct Game {
    board: GameBoard,
    fen_str: String,
    current_player: Rc<RefCell<Player>>,
    players: [Rc<RefCell<Player>>; 2],
    move_history: Vec<moves::Move>
}

impl Game{

    /// Returns a new Game with default Game board
    /// 
    /// # Examples
    /// ```
    /// use chess_lib_rs::game::Game;
    /// let game = Game::new();
    /// ```
    pub fn new() -> Self {
        Self::load("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()).unwrap()
    }


    fn load(fen_str: String) -> Result<Self, crate::error::Error> {    
        
        if ! is_valid_fen(&fen_str) {
            return Err(crate::error::Error::InvalidFen(fen_str));
        }
        
        let mut fen_slices = fen_str.split_ascii_whitespace();
        
        // building the board from the first part of the fen
        let board = GameBoard::build_board_from_fen(fen_slices.next().unwrap());
        let players = [
            Rc::new(
                RefCell::new(Player::white())
            ), 
            Rc::new(RefCell::new(Player::black()))
        ];

        // Determining the current player using second part of the fen
        let current_player = match fen_slices.next().unwrap() {
            "w" => Ok(players[0].clone()),
            "b" => Ok(players[1].clone()),
            _ => Err(Error::InvalidFen(fen_str.clone()))
        }?;

        // Determining the castling previlege using third part of the fen
        let castle_prev = fen_slices.next().unwrap();
        let castle_previleges = ("KQkq").chars()
            .map(|c| castle_prev.contains(c))
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|s| (s[0], s[1]))
            .collect::<Vec<_>>();
        players.iter()
            .enumerate()
            .for_each(|(i, pl)|pl.clone().borrow_mut().set_castle_prev(castle_previleges[i]));

        Ok(Self {
            board,
            fen_str: fen_str.to_string(),
            players,
            current_player,
            move_history: Vec::new()
        })
    }

    fn switch_player(&mut self){
        let color = self.current_player.borrow().color;
        self.current_player = match color {
            Color::Black => self.players[1].clone(),
            Color::White => self.players[0].clone()
        }
    }

    /// Updates the game state with new move
    /// 
    /// # Arguments
    /// * `move_code` - A string slice that holds the move code for the next state of the game
    /// 
    /// # Example
    /// ```
    /// use chess_lib_rs::game::Game;
    /// let game = Game::new();
    /// game.update("a1a2");  // Moves Pawn from a1 to a2
    /// ```
    pub fn update(&mut self, move_code: &str) -> Result<(), Error> {
        let game_move = moves::Move::from_str(move_code)?;
        self.board.apply_move(game_move, self.current_player.borrow().color)?;

        self.switch_player();
        self.move_history.push(game_move);
        Ok(())
    }

    /// Returns the fen of the current game state
    pub fn get_fen(&self) -> &str {
        &self.fen_str
    }
    
}

impl From<&str> for Game {
    /// Returns a new Game with provided fen
    /// 
    /// # Arguments
    /// 
    /// * `fen_str` - A string slice that holds the fen of the game
    ///  
    /// # Examples
    /// ```
    /// use chess_lib_rs::game::Game;
    /// let game = Game::from("rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    fn from(fen: &str) -> Self {
        Self::load(fen.into()).expect("Failed to create Game")
    }
}

fn is_valid_fen(fen_str: &str) -> bool{
    fen_str.chars().filter(|&c| c == '/').count() == 7 && 
    fen_str.split(' ').count() == 6 && 
    !fen_str.starts_with('/') 
}

#[cfg(test)]
mod game_tests{
    use crate::pieces::Piece;
    use std::str::FromStr;
    
    use super::*;

    #[test]
    fn test_game_created(){
        let mut game = Game::new();
        assert_eq!(game.get_fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(game.board.remove_piece(10).unwrap(), Piece::from_str("p").unwrap());
        assert_eq!(game.current_player.borrow().color , Color::White);
 
    }
    #[test]
    fn test_game_loaded(){
        let game = Game::from("r6k/2R5/6R1/pp1Ppp2/8/Pn2B1Pr/4KP2/8 b – – 0 1");
        assert_eq!(game.current_player.borrow().color , Color::Black);

        assert_eq!(game.get_fen(), r"r6k/2R5/6R1/pp1Ppp2/8/Pn2B1Pr/4KP2/8 b – – 0 1")
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