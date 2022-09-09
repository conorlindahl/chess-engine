use crate::board::Board;
use crate::piece::Color;
use crate::file::File;
use crate::piece::Piece;
use crate::rank::Rank;
use crate::square::Square;

use crate::rank;
use crate::file;

use std::cmp;
use std::iter;

#[derive(Debug, Clone, Copy)]
pub struct Bishop {
    color: Color,
}

impl Piece for Bishop {
    fn get_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
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

        let positive_diagonal = iter::zip(File::iter_files(positive_diagonal_file_range), Rank::iter_ranks(positive_diagonal_rank_range)).filter(|(file, rank)| {
            square.file != *file || square.rank != *rank
        }).map(|(file, rank)| {
            board.get_square(file, rank)
        });

        let negative_diagonal_left_diff = cmp::min(distance_to_left, distance_to_ceiling);
        let negative_diagonal_right_diff = cmp::min(distance_to_right, distance_to_floor);

        let negative_diagonal_file_range = (square.file.value()-negative_diagonal_left_diff)..=(square.file.value()+negative_diagonal_right_diff);
        let negative_diagonal_rank_range = ((square.rank.value()-negative_diagonal_right_diff)..=(square.rank.value()+negative_diagonal_left_diff)).rev();

        let negative_diagonal = iter::zip(File::iter_files(negative_diagonal_file_range), Rank::iter_ranks(negative_diagonal_rank_range)).filter(|(file, rank)| {
            square.file != *file || square.rank != *rank
        }).map(|(file, rank)| {
            board.get_square(file, rank)
        });

        positive_diagonal.chain(negative_diagonal).collect()
    }

    fn get_color(&self) -> Color {
        return self.color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn central_piece_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(4,4).unwrap();
        let bishop = Bishop{color: Color::White};
        
        let valid_moves = vec!(
            Square::build(0, 0).unwrap(),
            Square::build(1, 1).unwrap(),
            Square::build(2, 2).unwrap(),
            Square::build(3, 3).unwrap(),
            Square::build(5, 5).unwrap(),
            Square::build(6, 6).unwrap(),
            Square::build(7, 7).unwrap(),
            Square::build(1, 7).unwrap(),
            Square::build(2, 6).unwrap(),
            Square::build(3, 5).unwrap(),
            Square::build(5, 3).unwrap(),
            Square::build(6, 2).unwrap(),
            Square::build(7, 1).unwrap(),
        );

        let bishop_moves = bishop.get_moves(&board, &square);

        assert_eq!(valid_moves.len(), bishop_moves.len());
        
        assert!(
            valid_moves.iter().all(|m| {
                bishop_moves.contains(&m)
            })
        );
    }

    #[test]
    fn central_piece_lower_file_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(2,4).unwrap();
        let bishop = Bishop{color: Color::White};
        
        let valid_moves = vec!(
            Square::build(0, 2).unwrap(),
            Square::build(1, 3).unwrap(),
            Square::build(3, 5).unwrap(),
            Square::build(4, 6).unwrap(),
            Square::build(5, 7).unwrap(),
            Square::build(0, 6).unwrap(),
            Square::build(1, 5).unwrap(),
            Square::build(3, 3).unwrap(),
            Square::build(4, 2).unwrap(),
            Square::build(5, 1).unwrap(),
            Square::build(6, 0).unwrap(),
        );

        let bishop_moves = bishop.get_moves(&board, &square);

        assert_eq!(valid_moves.len(), bishop_moves.len());
        
        assert!(
            valid_moves.iter().all(|m| {
                bishop_moves.contains(&m)
            })
        );
    }

    #[test]
    fn central_piece_lower_rank_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(4,2).unwrap();
        let bishop = Bishop{color: Color::White};
        
        let valid_moves = vec!(
            Square::build(2, 0).unwrap(),
            Square::build(3, 1).unwrap(),
            Square::build(5, 3).unwrap(),
            Square::build(6, 4).unwrap(),
            Square::build(7, 5).unwrap(),
            Square::build(0, 6).unwrap(),
            Square::build(1, 5).unwrap(),
            Square::build(2, 4).unwrap(),
            Square::build(3, 3).unwrap(),
            Square::build(5, 1).unwrap(),
            Square::build(6, 0).unwrap(),
        );

        let bishop_moves = bishop.get_moves(&board, &square);

        assert_eq!(valid_moves.len(), bishop_moves.len());
        
        assert!(
            valid_moves.iter().all(|m| {
                bishop_moves.contains(&m)
            })
        );
    }

    #[test]
    fn left_edge_piece_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(0,4).unwrap();
        let bishop = Bishop{color: Color::White};
        
        let valid_moves = vec!(
            Square::build(1, 5).unwrap(),
            Square::build(2, 6).unwrap(),
            Square::build(3, 7).unwrap(),
            Square::build(1, 3).unwrap(),
            Square::build(2, 2).unwrap(),
            Square::build(3, 1).unwrap(),
            Square::build(4, 0).unwrap(),
        );

        let bishop_moves = bishop.get_moves(&board, &square);

        assert_eq!(valid_moves.len(), bishop_moves.len());
        
        assert!(
            valid_moves.iter().all(|m| {
                bishop_moves.contains(&m)
            })
        );
    }

    #[test]
    fn top_edge_piece_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(4,7).unwrap();
        let bishop = Bishop{color: Color::White};
        
        let valid_moves = vec!(
            Square::build(0, 3).unwrap(),
            Square::build(1, 4).unwrap(),
            Square::build(2, 5).unwrap(),
            Square::build(3, 6).unwrap(),
            Square::build(5, 6).unwrap(),
            Square::build(6, 5).unwrap(),
            Square::build(7, 4).unwrap(),
        );

        let bishop_moves = bishop.get_moves(&board, &square);

        assert_eq!(valid_moves.len(), bishop_moves.len());
        
        assert!(
            valid_moves.iter().all(|m| {
                bishop_moves.contains(&m)
            })
        );
    }

    #[test]
    fn right_edge_piece_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(7,4).unwrap();
        let bishop = Bishop{color: Color::White};
        
        let valid_moves = vec!(
            Square::build(3, 0).unwrap(),
            Square::build(4, 1).unwrap(),
            Square::build(5, 2).unwrap(),
            Square::build(6, 3).unwrap(),
            Square::build(4, 7).unwrap(),
            Square::build(5, 6).unwrap(),
            Square::build(6, 5).unwrap(),
        );

        let bishop_moves = bishop.get_moves(&board, &square);

        assert_eq!(valid_moves.len(), bishop_moves.len());
        
        assert!(
            valid_moves.iter().all(|m| {
                bishop_moves.contains(&m)
            })
        );
    }

    #[test]
    fn bottom_edge_piece_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(4,0).unwrap();
        let bishop = Bishop{color: Color::White};
        
        let valid_moves = vec!(
            Square::build(0, 4).unwrap(),
            Square::build(1, 3).unwrap(),
            Square::build(2, 2).unwrap(),
            Square::build(3, 1).unwrap(),
            Square::build(5, 1).unwrap(),
            Square::build(6, 2).unwrap(),
            Square::build(7, 3).unwrap(),
        );

        let bishop_moves = bishop.get_moves(&board, &square);

        assert_eq!(valid_moves.len(), bishop_moves.len());
        
        assert!(
            valid_moves.iter().all(|m| {
                bishop_moves.contains(&m)
            })
        );
    }
}
