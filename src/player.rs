use crate::pieces::Color;

#[derive(PartialEq)]
pub(crate) struct Player{
    pub(super) color: Color,
    castle: (bool, bool)
}
impl Player {
    pub(crate) fn black() -> Self{
        Self::new(Color::Black)
    }
    fn new(color: Color) -> Self {
        Self { color,  castle:(true, true)}
    }
    pub(crate) fn white() -> Self{
        Self::new(Color::White)
    }

    pub(super) fn set_castle_prev(&mut self, previl: (bool, bool)){
        self.castle = previl
    }
}


#[cfg(test)]
mod player_tests {

}