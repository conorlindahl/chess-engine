use crate::square::Square;
use crate::rank::Rank;
use crate::file::File;

pub struct Board {
    squares: [[Square; 8]; 8]
}

impl Board {
    pub fn get_square(&self, file: File, rank: Rank) -> &Square {
        let file = usize::from(file.value());
        let rank = usize::from(rank.value());
        &self.squares[file][rank]
    }
}