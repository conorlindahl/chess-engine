use crate::board::Board;
use crate::square::Square;
use crate::piece::Color;
use crate::piece::Piece;

#[derive(Debug, Clone, Copy)]
pub struct Pawn {
    color: Color,

    has_moved: bool,
}

impl Pawn {
    fn get_white_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
        let mut res: Vec<&Square> = Vec::new();

        if let Ok(rank_one) = square.rank.next_by(1) {
            // All pawn moves require at least one forward step

            // To move forward, square in front of pawn must be free
            if board.get_square(square.file, rank_one).is_empty() {
                res.push(board.get_square(square.file, rank_one));

                // Check second move forward only if can move one forward
                if !self.has_moved {
                    if let Ok(rank_two) = rank_one.next_by(1) {
                            if board.get_square(square.file, rank_two).is_empty() {
                                res.push(board.get_square(square.file, rank_two));
                            }
                    }
                }
            }

            // To capture, must have enemy piece diagonal from pawn
            if let Ok(file_left) = square.file.previous_by(1) {
                if !board.get_square(file_left, rank_one).is_empty() &&
                    !board.get_square(file_left, rank_one).piece_matches_color(self.color) {
                        res.push(board.get_square(file_left, rank_one));
                    }
            }
            if let Ok(file_right) = square.file.next_by(1) {
                if !board.get_square(file_right, rank_one).is_empty() &&
                    !board.get_square(file_right, rank_one).piece_matches_color(self.color) {
                        res.push(board.get_square(file_right, rank_one));
                    }
            }
        }
        res
    }

    fn get_black_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
        let mut res: Vec<&Square> = Vec::new();

        // Forward moves
        if let Ok(rank_one) = square.rank.previous_by(1) {
            // All pawn moves require at least one forward step

            // To move forward, square in front of pawn must be free
            if board.get_square(square.file, rank_one).is_empty() {
                res.push(board.get_square(square.file, rank_one));

                // Check second move forward only if can move one forward
                if !self.has_moved {
                    if let Ok(rank_two) = rank_one.previous_by(1) {
                            if board.get_square(square.file, rank_two).is_empty() {
                                res.push(board.get_square(square.file, rank_two));
                            }
                    }
                }
            }

            // To capture, must have enemy piece diagonal from pawn
            if let Ok(file_left) = square.file.previous_by(1) {
                if !board.get_square(file_left, rank_one).is_empty() &&
                    !board.get_square(file_left, rank_one).piece_matches_color(self.color) {
                        res.push(board.get_square(file_left, rank_one));
                    }
            }
            if let Ok(file_right) = square.file.next_by(1) {
                if !board.get_square(file_right, rank_one).is_empty() &&
                    !board.get_square(file_right, rank_one).piece_matches_color(self.color) {
                        res.push(board.get_square(file_right, rank_one));
                    }
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
    use std::rc::Rc;

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

    #[test]
    fn white_pawn_blocked_in_front_by_opponent() {
        let mut board = Board::build().unwrap();

        let square = Square::build(4,2).unwrap();
        board.add_piece(Rc::new(Pawn{color: Color::Black, has_moved: true}), square.file, square.rank);

        let square = Square::build(4,1).unwrap();
        let pawn = Pawn{color: Color::White, has_moved: false};

        let pawn_moves = pawn.get_moves(&board, &square);

        assert!(pawn_moves.is_empty());
    }
    
    #[test]
    fn white_pawn_blocked_in_front_by_friend() {
        let mut board = Board::build().unwrap();

        let square = Square::build(4,2).unwrap();
        board.add_piece(Rc::new(Pawn{color: Color::White, has_moved: true}), square.file, square.rank);

        let square = Square::build(4,1).unwrap();
        let pawn = Pawn{color: Color::White, has_moved: false};

        let pawn_moves = pawn.get_moves(&board, &square);

        assert!(pawn_moves.is_empty());
    }
    
    #[test]
    fn white_pawn_far_blocked_in_front_by_opponent() {
        let mut board = Board::build().unwrap();

        let square = Square::build(4,3).unwrap();
        board.add_piece(Rc::new(Pawn{color: Color::Black, has_moved: true}), square.file, square.rank);

        let square = Square::build(4,1).unwrap();
        let pawn = Pawn{color: Color::White, has_moved: false};

        let pawn_moves = pawn.get_moves(&board, &square);

        let valid_moves = vec!(
            Square::build(4, 3).unwrap(),
        );

        assert!(pawn_moves.iter().all(|sq| {
            !valid_moves.contains(&sq)
        }));
    }
    
    #[test]
    fn white_pawn_far_blocked_in_front_by_friend() {
        let mut board = Board::build().unwrap();

        let square = Square::build(4,3).unwrap();
        board.add_piece(Rc::new(Pawn{color: Color::White, has_moved: true}), square.file, square.rank);

        let square = Square::build(4,1).unwrap();
        let pawn = Pawn{color: Color::White, has_moved: false};

        let pawn_moves = pawn.get_moves(&board, &square);

        let valid_moves = vec!(
            Square::build(4, 3).unwrap(),
        );

        assert!(pawn_moves.iter().all(|sq| {
            !valid_moves.contains(&sq)
        }));
    }

    #[test]
    fn white_pawn_available_captures() {
        let mut board = Board::build().unwrap();

        let square = Square::build(3,2).unwrap();
        board.add_piece(Rc::new(Pawn{color: Color::Black, has_moved: true}), square.file, square.rank);
        let square = Square::build(5,2).unwrap();
        board.add_piece(Rc::new(Pawn{color: Color::Black, has_moved: true}), square.file, square.rank);

        let square = Square::build(4,1).unwrap();
        let pawn = Pawn{color: Color::White, has_moved: false};

        let pawn_moves = pawn.get_moves(&board, &square);

        let valid_captures = vec!(
            Square::build(3,2).unwrap(),
            Square::build(5,2).unwrap(),
        );

        assert!(valid_captures.iter().all(|sq| {
            pawn_moves.contains(&sq)
        }));
    }

    #[test]
    fn white_pawn_friends_on_captures() {
        let mut board = Board::build().unwrap();

        let square = Square::build(3,2).unwrap();
        board.add_piece(Rc::new(Pawn{color: Color::White, has_moved: true}), square.file, square.rank);
        let square = Square::build(5,2).unwrap();
        board.add_piece(Rc::new(Pawn{color: Color::White, has_moved: true}), square.file, square.rank);

        let square = Square::build(4,1).unwrap();
        let pawn = Pawn{color: Color::White, has_moved: false};

        let pawn_moves = pawn.get_moves(&board, &square);

        let invalid_captures = vec!(
            Square::build(3,2).unwrap(),
            Square::build(5,2).unwrap(),
        );

        assert!(invalid_captures.iter().all(|sq| {
            !pawn_moves.contains(&sq)
        }));
    }

    #[test]
    fn black_pawn_blocked_in_front_by_opponent() {
        let mut board = Board::build().unwrap();

        let square = Square::build(4,5).unwrap();
        board.add_piece(Rc::new(Pawn{color: Color::White, has_moved: true}), square.file, square.rank);

        let square = Square::build(4,6).unwrap();
        let pawn = Pawn{color: Color::Black, has_moved: false};

        let pawn_moves = pawn.get_moves(&board, &square);

        assert!(pawn_moves.is_empty());
    }
    
    #[test]
    fn black_pawn_blocked_in_front_by_friend() {
        let mut board = Board::build().unwrap();

        let square = Square::build(4,5).unwrap();
        board.add_piece(Rc::new(Pawn{color: Color::Black, has_moved: true}), square.file, square.rank);

        let square = Square::build(4,6).unwrap();
        let pawn = Pawn{color: Color::Black, has_moved: false};

        let pawn_moves = pawn.get_moves(&board, &square);

        assert!(pawn_moves.is_empty());
    }
    
    #[test]
    fn black_pawn_far_blocked_in_front_by_opponent() {
        let mut board = Board::build().unwrap();

        let square = Square::build(4,4).unwrap();
        board.add_piece(Rc::new(Pawn{color: Color::White, has_moved: true}), square.file, square.rank);

        let square = Square::build(4,6).unwrap();
        let pawn = Pawn{color: Color::Black, has_moved: false};

        let pawn_moves = pawn.get_moves(&board, &square);

        let valid_moves = vec!(
            Square::build(4, 4).unwrap(),
        );

        assert!(pawn_moves.iter().all(|sq| {
            !valid_moves.contains(&sq)
        }));
    }
    
    #[test]
    fn black_pawn_far_blocked_in_front_by_friend() {
        let mut board = Board::build().unwrap();

        let square = Square::build(4,4).unwrap();
        board.add_piece(Rc::new(Pawn{color: Color::Black, has_moved: true}), square.file, square.rank);

        let square = Square::build(4,6).unwrap();
        let pawn = Pawn{color: Color::Black, has_moved: false};

        let pawn_moves = pawn.get_moves(&board, &square);

        let valid_moves = vec!(
            Square::build(4, 4).unwrap(),
        );

        assert!(pawn_moves.iter().all(|sq| {
            !valid_moves.contains(&sq)
        }));
    }

    #[test]
    fn black_pawn_available_captures() {
        let mut board = Board::build().unwrap();

        let square = Square::build(3,5).unwrap();
        board.add_piece(Rc::new(Pawn{color: Color::White, has_moved: true}), square.file, square.rank);
        let square = Square::build(5,5).unwrap();
        board.add_piece(Rc::new(Pawn{color: Color::White, has_moved: true}), square.file, square.rank);

        let square = Square::build(4,6).unwrap();
        let pawn = Pawn{color: Color::Black, has_moved: false};

        let pawn_moves = pawn.get_moves(&board, &square);

        let valid_captures = vec!(
            Square::build(3,5).unwrap(),
            Square::build(5,5).unwrap(),
        );

        assert!(valid_captures.iter().all(|sq| {
            pawn_moves.contains(&sq)
        }));
    }

    #[test]
    fn black_pawn_friends_on_captures() {
        let mut board = Board::build().unwrap();

        let square = Square::build(3,5).unwrap();
        board.add_piece(Rc::new(Pawn{color: Color::Black, has_moved: true}), square.file, square.rank);
        let square = Square::build(5,5).unwrap();
        board.add_piece(Rc::new(Pawn{color: Color::Black, has_moved: true}), square.file, square.rank);

        let square = Square::build(4,6).unwrap();
        let pawn = Pawn{color: Color::Black, has_moved: false};

        let pawn_moves = pawn.get_moves(&board, &square);

        let invalid_captures = vec!(
            Square::build(3,2).unwrap(),
            Square::build(5,2).unwrap(),
        );

        assert!(invalid_captures.iter().all(|sq| {
            !pawn_moves.contains(&sq)
        }));
    }
}