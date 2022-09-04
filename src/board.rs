use crate::square::Square;
use crate::rank::Rank;
use crate::file::File;

pub struct Board {
    squares: [[Square; 8]; 8]
}

impl Board {
    pub fn get_square(&self, file: usize, row: usize) -> Square {
        self.squares[file][row]


    }
}