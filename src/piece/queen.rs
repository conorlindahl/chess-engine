use crate::board::Board;
use crate::piece::Color;
use crate::file::File;
use crate::piece::Piece;
use crate::rank::Rank;
use crate::square::Square;

use crate::rank;
use crate::file;

#[derive(Debug, Clone, Copy)]
pub struct Queen {
    color: Color,
}

impl Piece for Queen {
    fn get_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
        // Distance is # of steps to edge square
        board.get_diagonals(square).iter().chain(
            board.get_file(square.file).iter()
        ).chain(
            board.get_rank(square.rank).iter()
        ).filter_map(|&sq| {
            if (sq.rank != square.rank || sq.file != square.file) && !sq.piece_matches_color(self.color) {
                Some(sq)
            } else {
                None
            }
        }).collect()
    }

    fn get_color(&self) -> Color {
        return self.color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn central_pice_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(4,4).unwrap();
        let queen = Queen{color: Color::White};
        
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

        let queen_moves = queen.get_moves(&board, &square);

        assert_eq!(valid_moves.len(), queen_moves.len());
        
        assert!(
            valid_moves.iter().all(|m| {
                queen_moves.contains(&m)
            })
        );
    }

    #[test]
    fn edge_pice_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(0,4).unwrap();
        let queen = Queen{color: Color::White};
        
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
            Square::build(1, 5).unwrap(),
            Square::build(2, 6).unwrap(),
            Square::build(3, 7).unwrap(),
            Square::build(1, 3).unwrap(),
            Square::build(2, 2).unwrap(),
            Square::build(3, 1).unwrap(),
            Square::build(4, 0).unwrap(),
        );

        let queen_moves = queen.get_moves(&board, &square);

        assert_eq!(valid_moves.len(), queen_moves.len());
        
        assert!(
            valid_moves.iter().all(|m| {
                queen_moves.contains(&m)
            })
        );
    }

    #[test]
    fn corner_pice_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(0,0).unwrap();
        let queen = Queen{color: Color::White};
        
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
            Square::build(1, 1).unwrap(),
            Square::build(2, 2).unwrap(),
            Square::build(3, 3).unwrap(),
            Square::build(4, 4).unwrap(),
            Square::build(5, 5).unwrap(),
            Square::build(6, 6).unwrap(),
            Square::build(7, 7).unwrap(),
        );

        let queen_moves = queen.get_moves(&board, &square);

        assert_eq!(valid_moves.len(), queen_moves.len());
        
        assert!(
            valid_moves.iter().all(|m| {
                queen_moves.contains(&m)
            })
        );
    }

    #[test]
    fn central_pice_near_friendly_piece() {
        let mut board = Board::build().unwrap();
        let color = Color::White;

        let square_bad = Square::build(4, 7).unwrap();
        board.add_piece(Rc::new(Queen{color}), square_bad.file, square_bad.rank);
        
        let square = Square::build(4,4).unwrap();
        let queen = Queen{color};
        
        let queen_moves = queen.get_moves(&board, &square);

        assert!(!queen_moves.contains(&&square_bad));
    }
}