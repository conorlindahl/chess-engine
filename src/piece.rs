use crate::square::Square;

pub mod king;

pub trait Piece {
    fn get_moves(&self) -> Vec<Square>;
}
