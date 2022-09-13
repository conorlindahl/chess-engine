use crate::board::Board;
use crate::square::Square;

pub mod king;
pub mod knight;
pub mod rook;
pub mod bishop;
pub mod queen;
pub mod pawn;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

use core::fmt::Debug;
pub trait Piece: Debug  {
    fn get_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square>;
    fn get_color(&self) -> Color;
}

/*
// Implement debug trait for Piece so we can print debug info for Squares
impl Debug for dyn Piece {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Printing Piece")
    }
}
*/