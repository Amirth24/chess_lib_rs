use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    
}


impl Move {
    fn new(from: usize, to:usize) -> Self {
        Self {
            from , 
            to 
        }
    }

}

fn char_to_int(ch: char, base: char) -> u8 {
    return ch.to_lowercase().next().unwrap() as u8 - base as u8;
}

fn file_to_int(ch: char) -> u8{
    char_to_int(ch, 'a')
}


fn rank_to_int(ch: char) -> u8{
    8 - char_to_int(ch, '0') 
}

impl std::str::FromStr for Move {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if !Regex::new(r"[a-h]\d[a-h]\d").unwrap().is_match(s) {
            return Err(Self::Err::InvalidMoveStr(s.to_owned()));
        }

        let cell_coors = s.split_at(2);

        let mut cell_coors = (
            cell_coors.0.chars(),
            cell_coors.1.chars()
        );
        let cell_coors = (
            file_to_int(cell_coors.0.next().unwrap())+rank_to_int(cell_coors.0.next().unwrap())*8,
            file_to_int(cell_coors.1.next().unwrap())+rank_to_int(cell_coors.1.next().unwrap())*8
        );


        
        Ok(Self::new(cell_coors.0.into(), cell_coors.1.into()))
    }
}

#[cfg(test)]
mod moves_tests {
    use std::str::FromStr;

    use crate::moves::{file_to_int, rank_to_int, Move};

    #[test]
    fn test_file_to_int() {
        assert_eq!(file_to_int('a'), 0);
        assert_eq!(file_to_int('A'), 0);

        assert_eq!(file_to_int('z'), 25);
        assert_eq!(file_to_int('Z'), 25);

    }

    #[test]
    fn test_rank_to_int() {
        assert_eq!(rank_to_int('1'), 7);
        assert_eq!(rank_to_int('2'), 6);
        assert_eq!(rank_to_int('3'), 5);

        assert_eq!(rank_to_int('4'), 4);
        assert_eq!(rank_to_int('5'), 3);
        assert_eq!(rank_to_int('6'), 2);
        assert_eq!(rank_to_int('7'), 1);
        assert_eq!(rank_to_int('8'), 0);

    }


    #[test]
    fn test_move_from_str() {
        assert_eq!(Move::from_str("a8b8").unwrap(), Move::new(0, 1));
        assert_eq!(Move::from_str("a8a7").unwrap(), Move::new(0, 8));

        assert_eq!(Move::from_str("g1h1").unwrap(), Move::new(62, 63));

    }
}