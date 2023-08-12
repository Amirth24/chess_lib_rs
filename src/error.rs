#[derive(Debug)]
pub enum Error {
    InvalidFen(String),
    PieceNotFoundAtCell(usize),
    InvalidPieceStr(String)
}