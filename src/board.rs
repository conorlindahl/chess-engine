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

    pub fn get_file(&self, file: usize) -> [Square; 8] {
        let mut resultFile: [Square; 8] = [Square{rank: Rank::R1, file: File::A}; 8];

        for i in 0..8 {
            resultFile[i] = self.squares[i][file];
        }

        resultFile
    }

    pub fn get_row(&self, row: usize) -> [Square; 8] {
        self.squares[row]
    }
}