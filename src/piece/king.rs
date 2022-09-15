use crate::board::Board;
use crate::square::Square;
use crate::piece::Color;
use crate::piece::Piece;
use crate::rank::Rank;
use crate::file::File;

#[derive(Debug, Clone, Copy)]
pub struct King {
    color: Color,
}

impl Piece for King {
    fn get_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
        let rank_val = square.rank().value();
        let file_val = square.file().value();

        Rank::iter_ranks((rank_val.saturating_sub(1))..=(rank_val+1)).flat_map(|rank| {
            File::iter_files((file_val.saturating_sub(1))..=(file_val+1)).filter(move |file| {
                !(rank.value() == rank_val && file.value() == file_val)
            }).map(move |file| {
                board.get_square(file, rank)
            })
        }).filter(|square| {
            !square.piece_matches_color(self.color)
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
    fn central_square_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(4,4).unwrap();
        
        let king = King{color: Color::White};

        let valid_moves = vec!(
            Square::build(3, 3).unwrap(),
            Square::build(3, 4).unwrap(),
            Square::build(3, 5).unwrap(),
            Square::build(4, 3).unwrap(),
            Square::build(4, 5).unwrap(),
            Square::build(5, 3).unwrap(),
            Square::build(5, 4).unwrap(),
            Square::build(5, 5).unwrap()
        );

        let king_moves = king.get_moves(&board, &square);

        assert!(valid_moves.len() == king_moves.len());
        
        assert!(
            valid_moves.iter().all(|m| {
                king_moves.contains(&m)
            })
        );
    }

    #[test]
    fn edge_square_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(0,4).unwrap();
        
        let king = King{color: Color::White};

        let valid_moves = vec!(
            Square::build(0, 3).unwrap(),
            Square::build(0, 5).unwrap(),
            Square::build(1, 3).unwrap(),
            Square::build(1, 4).unwrap(),
            Square::build(1, 5).unwrap(),
        );

        let king_moves = king.get_moves(&board, &square);

        assert!(valid_moves.len() == king_moves.len());

        assert!(
            valid_moves.iter().all(|m| {
                king_moves.contains(&m)
            })
        );
    }

    #[test]
    fn corner_square_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(0,0).unwrap();
        
        let king = King{color: Color::White};

        let valid_moves = vec!(
            Square::build(0, 1).unwrap(),
            Square::build(1, 0).unwrap(),
            Square::build(1, 1).unwrap(),
        );

        let king_moves = king.get_moves(&board, &square);

        assert!(valid_moves.len() == king_moves.len());

        assert!(
            valid_moves.iter().all(|m| {
                king_moves.contains(&m)
            })
        );
    }

    #[test]
    fn central_square_near_friendly_piece() {
        let mut board = Board::build().unwrap();
        let color = Color::White;

        let square_bad = Square::build(3,4).unwrap();
        board.add_piece(Rc::new(King{color}), square_bad.file, square_bad.rank);

        let square = Square::build(4,4).unwrap();
        let king = King{color};

        let king_moves = king.get_moves(&board, &square);

        println!("{:?}", king_moves);

        assert!(!king_moves.contains(&&square_bad));
    }
}