use crate::board::Board;
use crate::square::Square;
use crate::piece::Color;
use crate::piece::Piece;
use crate::rank::Rank;
use crate::file::File;

#[derive(Debug, Clone, Copy)]
pub struct Pawn {
    color: Color,

    has_moved: bool,
}

/* Idea for an improvement: Create a 3x3 array of Option<T>. If left/right/up/down are None, fill out the adjacent values with None.
 * Only calculate squares that aren't adjacent to None.
 */
const WHITE_STARTING_RANK: u8 = 1;
const BLACK_STARTING_RANK: u8 = 6;

impl Pawn {
    fn get_white_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
        let mut res: Vec<&Square> = Vec::new();
        if let Ok(r) = square.rank.next_by(1) {
            res.push(board.get_square(square.file, r));
        }
        if !self.has_moved {
            if let Ok(r) = square.rank.next_by(2) {
                    res.push(board.get_square(square.file, r));
            }
        }
        res
    }

    fn get_black_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
        let mut res: Vec<&Square> = Vec::new();
        if let Ok(r) = square.rank.previous_by(1) {
            res.push(board.get_square(square.file, r));
        }
        if !self.has_moved {
            if let Ok(r) = square.rank.previous_by(2) {
                res.push(board.get_square(square.file, r));
            }
        }
        res
    }
}

impl Piece for Pawn {
    fn get_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
        match self.color {
            Color::White => self.get_white_moves(board, square),
            Color::Black => self.get_black_moves(board, square)
        }
    }

    fn get_color(&self) -> Color {
        return self.color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_pawn_starting_rank_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(4,1).unwrap();
        
        let pawn = Pawn{color: Color::White, has_moved: false};

        let valid_moves = vec!(
            Square::build(4, 2).unwrap(),
            Square::build(4, 3).unwrap(),
        );

        let pawn_moves = pawn.get_moves(&board, &square);

        assert!(valid_moves.len() == pawn_moves.len());
        
        assert!(
            valid_moves.iter().all(|m| {
                pawn_moves.contains(&m)
            })
        );
    }

    #[test]
    fn white_pawn_non_starting_rank_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(4,2).unwrap();
        
        let pawn = Pawn{color: Color::White, has_moved: true};

        let valid_moves = vec!(
            Square::build(4, 3).unwrap(),
        );

        let pawn_moves = pawn.get_moves(&board, &square);

        assert!(valid_moves.len() == pawn_moves.len());
        
        assert!(
            valid_moves.iter().all(|m| {
                pawn_moves.contains(&m)
            })
        );
    }

    #[test]
    fn black_pawn_starting_rank_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(4,6).unwrap();
        
        let pawn = Pawn{color: Color::Black, has_moved: false};

        let valid_moves = vec!(
            Square::build(4, 5).unwrap(),
            Square::build(4, 4).unwrap(),
        );

        let pawn_moves = pawn.get_moves(&board, &square);

        assert!(valid_moves.len() == pawn_moves.len());
        
        assert!(
            valid_moves.iter().all(|m| {
                pawn_moves.contains(&m)
            })
        );
    }

    #[test]
    fn black_pawn_non_starting_rank_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(4,5).unwrap();
        
        let pawn = Pawn{color: Color::Black, has_moved: true};

        let valid_moves = vec!(
            Square::build(4, 4).unwrap(),
        );

        let pawn_moves = pawn.get_moves(&board, &square);

        assert!(valid_moves.len() == pawn_moves.len());
        
        assert!(
            valid_moves.iter().all(|m| {
                pawn_moves.contains(&m)
            })
        );
    }
}