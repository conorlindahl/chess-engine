use crate::square::Square;
use crate::rank;
use crate::rank::Rank;
use crate::file;
use crate::file::File;

use std::cmp;
use std::iter;

pub struct Board {
    squares: [[Square; 8]; 8]
}

impl Board {
    pub fn build() -> Result<Board, &'static str> {
        let squares = [
            [Square::build(0, 0)?, Square::build(0, 1)?, Square::build(0, 2)?, Square::build(0, 3)?, Square::build(0, 4)?, Square::build(0, 5)?, Square::build(0, 6)?, Square::build(0, 7)?],
            [Square::build(1, 0)?, Square::build(1, 1)?, Square::build(1, 2)?, Square::build(1, 3)?, Square::build(1, 4)?, Square::build(1, 5)?, Square::build(1, 6)?, Square::build(1, 7)?],
            [Square::build(2, 0)?, Square::build(2, 1)?, Square::build(2, 2)?, Square::build(2, 3)?, Square::build(2, 4)?, Square::build(2, 5)?, Square::build(2, 6)?, Square::build(2, 7)?],
            [Square::build(3, 0)?, Square::build(3, 1)?, Square::build(3, 2)?, Square::build(3, 3)?, Square::build(3, 4)?, Square::build(3, 5)?, Square::build(3, 6)?, Square::build(3, 7)?],
            [Square::build(4, 0)?, Square::build(4, 1)?, Square::build(4, 2)?, Square::build(4, 3)?, Square::build(4, 4)?, Square::build(4, 5)?, Square::build(4, 6)?, Square::build(4, 7)?],
            [Square::build(5, 0)?, Square::build(5, 1)?, Square::build(5, 2)?, Square::build(5, 3)?, Square::build(5, 4)?, Square::build(5, 5)?, Square::build(5, 6)?, Square::build(5, 7)?],
            [Square::build(6, 0)?, Square::build(6, 1)?, Square::build(6, 2)?, Square::build(6, 3)?, Square::build(6, 4)?, Square::build(6, 5)?, Square::build(6, 6)?, Square::build(6, 7)?],
            [Square::build(7, 0)?, Square::build(7, 1)?, Square::build(7, 2)?, Square::build(7, 3)?, Square::build(7, 4)?, Square::build(7, 5)?, Square::build(7, 6)?, Square::build(7, 7)?],           
        ];
        Ok(Board{squares})
    }

    pub fn get_square(&self, file: File, rank: Rank) -> &Square {
        let file = usize::from(file.value());
        let rank = usize::from(rank.value());
        &self.squares[file][rank]
    }

    pub fn get_rank(&self, rank: Rank) -> Vec<&Square> {
        File::iter_files(0..file::MAX_NUMBER_OF_FILES).map(|file| {
            self.get_square(file, rank)
        }).collect()
    }

    pub fn get_file(&self, file: File) -> Vec<&Square> {
        Rank::iter_ranks(0..rank::MAX_NUMBER_OF_RANKS).map(|rank| {
            self.get_square(file, rank)
        }).collect()
    }

    pub fn get_diagonals(&self, square: &Square) -> Vec<&Square> {
        // Distance is # of steps to edge square
        let distance_to_left = square.file.value();
        let distance_to_floor = square.rank.value();
        let distance_to_right = file::MAX_NUMBER_OF_FILES - square.file.value() - 1;
        let distance_to_ceiling = rank::MAX_NUMBER_OF_RANKS - square.rank.value() - 1;

        let positive_diagonal_left_diff = cmp::min(distance_to_left, distance_to_floor);
        let positive_diagonal_right_diff = cmp::min(distance_to_right, distance_to_ceiling);

        // Use steps so that we can
        let positive_diagonal_file_range = (square.file.value()-positive_diagonal_left_diff)..=(square.file.value()+positive_diagonal_right_diff);
        let positive_diagonal_rank_range = (square.rank.value()-positive_diagonal_left_diff)..=(square.rank.value()+positive_diagonal_right_diff);

        let positive_diagonal = iter::zip(File::iter_files(positive_diagonal_file_range), Rank::iter_ranks(positive_diagonal_rank_range)).map(|(file, rank)| {
            self.get_square(file, rank)
        });

        let negative_diagonal_left_diff = cmp::min(distance_to_left, distance_to_ceiling);
        let negative_diagonal_right_diff = cmp::min(distance_to_right, distance_to_floor);

        let negative_diagonal_file_range = (square.file.value()-negative_diagonal_left_diff)..=(square.file.value()+negative_diagonal_right_diff);
        let negative_diagonal_rank_range = ((square.rank.value()-negative_diagonal_right_diff)..=(square.rank.value()+negative_diagonal_left_diff)).rev();

        let negative_diagonal = iter::zip(File::iter_files(negative_diagonal_file_range), Rank::iter_ranks(negative_diagonal_rank_range)).map(|(file, rank)| {
            self.get_square(file, rank)
        });

        positive_diagonal.chain(negative_diagonal).collect()
    }
}