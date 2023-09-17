use std::str::FromStr;

use crate::error::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black , White
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum PieceType {
    King, Queen, Bishop, Knight, Rook, Pawn
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Piece {
    color: Color, 
    piece_type: PieceType
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self { color, piece_type }
    }

    pub fn color(&self) -> Color {
        return self.color
    }

    pub fn piece_type(&self) -> PieceType {
        return self.piece_type
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let piece_str= match self.piece_type {
            PieceType::King => "k",
            PieceType::Queen =>"q",
            PieceType::Bishop => "b",
            PieceType::Knight => "n",
            PieceType::Rook => "r",
            PieceType::Pawn => "p",
        };


        match self.color {
            Color::Black => f.write_str(piece_str),
            Color::White => f.write_str(piece_str.to_uppercase().as_str())
        }

    }
}

impl FromStr for Piece {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let piece_type = match s.to_lowercase().as_str() {
            "k" => Ok(PieceType::King),
            "q" => Ok(PieceType::Queen),
            "b" => Ok(PieceType::Bishop),
            "n" => Ok(PieceType::Knight),
            "r" => Ok(PieceType::Rook),
            "p" => Ok(PieceType::Pawn),
            _ => Err(Error::InvalidPieceStr(s.to_owned()))
        }?;

        let color = if s.chars().next().unwrap().is_uppercase() {Color::White} else {Color::Black};

        Ok(Self {
            piece_type, color
        }) 
    }

}

#[cfg(test)]
mod pieces_tests {
    use super::*;

    #[test]
    fn test_piece_display(){
        assert_eq!(Piece::new(PieceType::Pawn, Color::Black).to_string(), "p");
        assert_eq!(Piece::new(PieceType::Pawn, Color::White).to_string(), "P");
        assert_eq!(Piece::new(PieceType::Knight, Color::White).to_string(), "N");
        assert_eq!(Piece::new(PieceType::King, Color::Black).to_string(), "k");
        
        assert_ne!(Piece::new(PieceType::Bishop, Color::White).to_string(), "P");
        assert_ne!(Piece::new(PieceType::Rook, Color::White).to_string(), "N");
        assert_ne!(Piece::new(PieceType::King, Color::White).to_string(), "k");
        assert_ne!(Piece::new(PieceType::Queen, Color::Black).to_string(), "k");
    }

    #[test]
    fn test_from_str(){
        assert_eq!(Piece::from_str("p").unwrap(), Piece::new(PieceType::Pawn, Color::Black));
        assert_eq!(Piece::from_str("P").unwrap(), Piece::new(PieceType::Pawn, Color::White));
        assert_eq!(Piece::from_str("k").unwrap(), Piece::new(PieceType::King, Color::Black));
        assert_eq!(Piece::from_str("K").unwrap(), Piece::new(PieceType::King, Color::White));
        assert_eq!(Piece::from_str("q").unwrap(), Piece::new(PieceType::Queen, Color::Black));
        assert_eq!(Piece::from_str("Q").unwrap(), Piece::new(PieceType::Queen, Color::White));
        assert_eq!(Piece::from_str("n").unwrap(), Piece::new(PieceType::Knight, Color::Black));
        assert_eq!(Piece::from_str("N").unwrap(), Piece::new(PieceType::Knight, Color::White));
        assert_eq!(Piece::from_str("r").unwrap(), Piece::new(PieceType::Rook, Color::Black));
        assert_eq!(Piece::from_str("R").unwrap(), Piece::new(PieceType::Rook, Color::White));
        assert_eq!(Piece::from_str("b").unwrap(), Piece::new(PieceType::Bishop, Color::Black));
        assert_eq!(Piece::from_str("B").unwrap(), Piece::new(PieceType::Bishop, Color::White));

    }
}

