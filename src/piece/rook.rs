use crate::board::Board;
use crate::piece::Color;
use crate::file::File;
use crate::piece::Piece;
use crate::rank::Rank;
use crate::square::Square;

use crate::rank;
use crate::file;

#[derive(Debug, Clone, Copy)]
pub struct Rook {
    color: Color,
}

impl Piece for Rook {
    fn get_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
        let rank_iter = Rank::iter_ranks(0..rank::MAX_NUMBER_OF_RANKS).filter(|rank| {
            rank.value() != square.rank.value()
        }).map(|rank| {
            board.get_square(square.file, rank)
        });
        let file_iter = File::iter_files(0..file::MAX_NUMBER_OF_FILES).filter(|file| {
            file.value() != square.file.value()
        }).map(|file| {
            board.get_square(file, square.rank)
        });
        rank_iter.chain(file_iter).collect()
    }

    fn get_color(&self) -> Color {
        return self.color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn central_pice_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(4,4).unwrap();
        let rook = Rook{color: Color::White};
        
        let valid_moves = vec!(
            Square::build(0, 4).unwrap(),
            Square::build(1, 4).unwrap(),
            Square::build(2, 4).unwrap(),
            Square::build(3, 4).unwrap(),
            Square::build(5, 4).unwrap(),
            Square::build(6, 4).unwrap(),
            Square::build(7, 4).unwrap(),
            Square::build(4, 0).unwrap(),
            Square::build(4, 1).unwrap(),
            Square::build(4, 2).unwrap(),
            Square::build(4, 3).unwrap(),
            Square::build(4, 5).unwrap(),
            Square::build(4, 6).unwrap(),
            Square::build(4, 7).unwrap(),
        );

        let rook_moves = rook.get_moves(&board, &square);

        assert!(valid_moves.len() == rook_moves.len());
        
        assert!(
            valid_moves.iter().all(|m| {
                rook_moves.contains(&m)
            })
        );
    }

    #[test]
    fn edge_pice_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(0,4).unwrap();
        let rook = Rook{color: Color::White};
        
        let valid_moves = vec!(
            Square::build(1, 4).unwrap(),
            Square::build(2, 4).unwrap(),
            Square::build(3, 4).unwrap(),
            Square::build(4, 4).unwrap(),
            Square::build(5, 4).unwrap(),
            Square::build(6, 4).unwrap(),
            Square::build(7, 4).unwrap(),
            Square::build(0, 0).unwrap(),
            Square::build(0, 1).unwrap(),
            Square::build(0, 2).unwrap(),
            Square::build(0, 3).unwrap(),
            Square::build(0, 5).unwrap(),
            Square::build(0, 6).unwrap(),
            Square::build(0, 7).unwrap(),
        );

        let rook_moves = rook.get_moves(&board, &square);

        assert!(valid_moves.len() == rook_moves.len());
        
        assert!(
            valid_moves.iter().all(|m| {
                rook_moves.contains(&m)
            })
        );
    }

    #[test]
    fn corner_pice_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(0,0).unwrap();
        let rook = Rook{color: Color::White};
        
        let valid_moves = vec!(
            Square::build(1, 0).unwrap(),
            Square::build(2, 0).unwrap(),
            Square::build(3, 0).unwrap(),
            Square::build(4, 0).unwrap(),
            Square::build(5, 0).unwrap(),
            Square::build(6, 0).unwrap(),
            Square::build(7, 0).unwrap(),
            Square::build(0, 1).unwrap(),
            Square::build(0, 2).unwrap(),
            Square::build(0, 3).unwrap(),
            Square::build(0, 4).unwrap(),
            Square::build(0, 5).unwrap(),
            Square::build(0, 6).unwrap(),
            Square::build(0, 7).unwrap(),
        );

        let rook_moves = rook.get_moves(&board, &square);

        assert!(valid_moves.len() == rook_moves.len());
        
        assert!(
            valid_moves.iter().all(|m| {
                rook_moves.contains(&m)
            })
        );
    }
}