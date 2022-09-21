use crate::board::Board;
use crate::square::Square;
use crate::rank::Rank;
use crate::file::File;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    color: Color,
    piece_type: PieceType,
    has_moved: bool,
}

impl Piece {
    fn new(color: Color, piece_type: PieceType, has_moved: bool) -> Piece {
        Piece { color: color, has_moved: has_moved, piece_type: piece_type }
    }

    pub fn get_color(&self) -> Color { self.color }

    pub fn has_moved(&self) -> bool  { self.has_moved }

    pub fn set_moved(&mut self) { self.has_moved = true }

    pub fn piece_type(&self) -> PieceType { self.piece_type }

    pub fn get_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
        match self.piece_type {
            PieceType::Pawn => self.get_pawn_moves(board, square),
            PieceType::Knight => self.get_knight_moves(board, square),
            PieceType::Bishop => self.get_bishop_moves(board, square),
            PieceType::Rook => self.get_rook_moves(board, square),
            PieceType::Queen => self.get_queen_moves(board, square),
            PieceType::King => self.get_king_moves(board, square),
        }
    }

    fn get_pawn_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
        match self.color {
            Color::White => self.get_white_pawn_moves(board, square),
            Color::Black => self.get_black_pawn_moves(board, square)
        }
    }

    fn get_white_pawn_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
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

    fn get_black_pawn_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
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

    fn get_knight_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
        static KNIGHT_FILE_RANK_RELATIONSHIPS: [(u8, u8); 2] = [(2, 1), (1, 2)];

        KNIGHT_FILE_RANK_RELATIONSHIPS.iter().flat_map(|(mod_file, mod_rank)| {
            let file = square.file.value();
            let rank = square.rank.value();
            [
                (file.checked_sub(*mod_file), rank.checked_sub(*mod_rank)),
                (file.checked_sub(*mod_file), rank.checked_add(*mod_rank)),
                (file.checked_add(*mod_file), rank.checked_sub(*mod_rank)),
                (file.checked_add(*mod_file), rank.checked_add(*mod_rank))
            ].iter().filter(|(file, rank)| {
                file.is_some() && rank.is_some()
            }).map(|(file, rank)| {
                (File::build(file.unwrap()), Rank::build(rank.unwrap()))
            }).filter(|(file, rank)| {
                file.is_ok() && rank.is_ok()
            }).map(|(file, rank)| {
                board.get_square(file.unwrap(), rank.unwrap())
            }).filter(|square| {
                !square.piece_matches_color(self.color)
            }).collect::<Vec<&Square>>()
        }).collect()
    }

    fn get_bishop_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
        board.get_diagonals(square).iter().filter_map(|&sq| {
            if sq.rank != square.rank && sq.file != square.file && !sq.piece_matches_color(self.color) {
                Some(sq)
            } else {
                None
            }
        }).collect()
    }

    fn get_rook_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
        board.get_file(square.file).iter().chain(
            board.get_rank(square.rank).iter()
        ).filter(|sq| {
            (square.rank != sq.rank || square.file != sq.file) && !sq.piece_matches_color(self.color)
        }).map(|&sq| {
            sq
        }).collect()
    }

    fn get_queen_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
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

    fn get_king_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
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
}

#[cfg(test)]
mod king_tests {
    use super::*;

    #[test]
    fn central_square_empty_board() {
        let board = Board::build_empty().unwrap();
        let king = Piece{
            color: Color::White,
            piece_type: PieceType::King,
            has_moved: true
        };

        // Central Square
        let square = Square::build(4,4).unwrap();
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
        let board = Board::build_empty().unwrap();
        let king = Piece{
            color: Color::White,
            piece_type: PieceType::King,
            has_moved: true
        };

        let square = Square::build(0,4).unwrap();
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
        let board = Board::build_empty().unwrap();
        let king = Piece{
            color: Color::White,
            piece_type: PieceType::King,
            has_moved: true
        };

        let square = Square::build(0,0).unwrap();
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
}

#[cfg(test)]
mod knight_tests {
    use super::*;

    #[test]
    fn central_square_empty_board() {
        let board = Board::build_empty().unwrap();
        let knight = Piece{
            color: Color::White,
            piece_type: PieceType::Knight,
            has_moved: true
        };

        // Central Square
        let square = Square::build(4,4).unwrap();
        let valid_moves = vec!(
            Square::build(3, 6).unwrap(),
            Square::build(5, 6).unwrap(),
            Square::build(6, 5).unwrap(),
            Square::build(6, 3).unwrap(),
            Square::build(5, 2).unwrap(),
            Square::build(3, 2).unwrap(),
            Square::build(2, 3).unwrap(),
            Square::build(2, 5).unwrap()
        );
        let knight_moves = knight.get_moves(&board, &square);

        assert_eq!(valid_moves.len(), knight_moves.len());
        assert!(
            valid_moves.iter().all(|m| {
                knight_moves.contains(&m)
            })
        );
    }

    #[test]
    fn edge_square_empty_board() {
        let board = Board::build_empty().unwrap();
        let knight = Piece{
            color: Color::White,
            piece_type: PieceType::Knight,
            has_moved: true
        };

        let square = Square::build(0,4).unwrap();
        let valid_moves = vec!(
            Square::build(1, 6).unwrap(),
            Square::build(2, 5).unwrap(),
            Square::build(2, 3).unwrap(),
            Square::build(1, 2).unwrap(),
        );
        let knight_moves = knight.get_moves(&board, &square);

        assert!(valid_moves.len() == knight_moves.len());
        assert!(
            valid_moves.iter().all(|m| {
                knight_moves.contains(&m)
            })
        );
    }

    #[test]
    fn corner_square_empty_board() {
        let board = Board::build_empty().unwrap();
        let knight = Piece{
            color: Color::White,
            piece_type: PieceType::Knight,
            has_moved: true
        };

        let square = Square::build(0,0).unwrap();
        let valid_moves = vec!(
            Square::build(1, 2).unwrap(),
            Square::build(2, 1).unwrap(),
        );
        let knight_moves = knight.get_moves(&board, &square);

        assert!(valid_moves.len() == knight_moves.len());
        assert!(
            valid_moves.iter().all(|m| {
                knight_moves.contains(&m)
            })
        );
    }

    #[test]
    fn friendly_piece_collision() {
        let mut board = Board::build_empty().unwrap();
        let color = Color::White;

        let square_friendly = Square::build(2,5).unwrap();
        let knight_friendly = Piece {
            color: color,
            piece_type: PieceType::Knight,
            has_moved: true
        };
        board.add_piece(knight_friendly, square_friendly.file, square_friendly.rank);

        let square = Square::build(4,4).unwrap();
        let knight = Piece{
            color: color,
            piece_type: PieceType::Knight,
            has_moved: true
        };
        let knight_moves = knight.get_moves(&board, &square);

        assert!(!knight_moves.contains(&&square_friendly));
    }
}

#[cfg(test)]
mod bishop_tests {
    use super::*;

    #[test]
    fn empty_board() {
        let board = Board::build_empty().unwrap();
        let bishop = Piece{
            color: Color::White,
            piece_type: PieceType::Bishop,
            has_moved: true
        };

        // Central Square
        let square = Square::build(4,4).unwrap();
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
        let board = Board::build_empty().unwrap();
        let bishop = Piece{
            color: Color::White,
            piece_type: PieceType::Bishop,
            has_moved: true
        };

        let square = Square::build(2,4).unwrap();
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
        let board = Board::build_empty().unwrap();
        let bishop = Piece{
            color: Color::White,
            piece_type: PieceType::Bishop,
            has_moved: true
        };

        let square = Square::build(4,2).unwrap();
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
        let board = Board::build_empty().unwrap();
        let bishop = Piece{
            color: Color::White,
            piece_type: PieceType::Bishop,
            has_moved: true
        };

        let square = Square::build(0,4).unwrap();
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
        let board = Board::build_empty().unwrap();
        let bishop = Piece{
            color: Color::White,
            piece_type: PieceType::Bishop,
            has_moved: true
        };

        let square = Square::build(4,7).unwrap();
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
        let board = Board::build_empty().unwrap();
        let bishop = Piece{
            color: Color::White,
            piece_type: PieceType::Bishop,
            has_moved: true
        };

        let square = Square::build(7,4).unwrap();
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
        let board = Board::build_empty().unwrap();
        let bishop = Piece{
            color: Color::White,
            piece_type: PieceType::Bishop,
            has_moved: true
        };

        let square = Square::build(4,0).unwrap();
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

    #[test]
    fn central_piece_near_friendly_piece() {
        let mut board = Board::build_empty().unwrap();
        let color = Color::Black;

        let square_friendly = Square::build(1, 7).unwrap();
        let bishop = Piece::new(color, PieceType::Bishop, true);
        board.add_piece(bishop, square_friendly.file, square_friendly.rank);

        let square = Square::build(4,4).unwrap();

        let bishop_moves = bishop.get_moves(&board, &square);

        assert!(!bishop_moves.contains(&&square_friendly));
    }
}

#[cfg(test)]
mod rook_tests {
    use super::*;

    #[test]
    fn central_pice_empty_board() {
        let board = Board::build_empty().unwrap();
        let rook = Piece{
            color: Color::White,
            piece_type: PieceType::Rook,
            has_moved: true
        };

        let square = Square::build(4,4).unwrap();
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

        assert_eq!(valid_moves.len(), rook_moves.len());
        assert!(
            valid_moves.iter().all(|m| {
                rook_moves.contains(&m)
            })
        );
    }

    #[test]
    fn edge_pice_empty_board() {
        let board = Board::build_empty().unwrap();
        let rook = Piece{
            color: Color::White,
            piece_type: PieceType::Rook,
            has_moved: true
        };

        let square = Square::build(0,4).unwrap();
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

        assert_eq!(valid_moves.len(), rook_moves.len());
        assert!(
            valid_moves.iter().all(|m| {
                rook_moves.contains(&m)
            })
        );
    }

    #[test]
    fn corner_pice_empty_board() {
        let board = Board::build_empty().unwrap();
        let rook = Piece{
            color: Color::White,
            piece_type: PieceType::Rook,
            has_moved: true
        };

        let square = Square::build(0,0).unwrap();
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

        assert_eq!(valid_moves.len(), rook_moves.len());
        assert!(
            valid_moves.iter().all(|m| {
                rook_moves.contains(&m)
            })
        );
    }

    #[test]
    fn central_pice_near_friendly_piece() {
        let mut board = Board::build_empty().unwrap();
        let color = Color::Black;

        let square_bad = Square::build(1, 4).unwrap();
        let rook = Piece {
            color: color,
            piece_type: PieceType::Rook,
            has_moved: true,
        };
        board.add_piece(rook, square_bad.file, square_bad.rank);

        let square = Square::build(4,4).unwrap();
        let rook_moves = rook.get_moves(&board, &square);

        assert!(!rook_moves.contains(&&square_bad));
    }
}

#[cfg(test)]
mod queen_tests {
    use super::*;

    #[test]
    fn central_pice_empty_board() {
        let board = Board::build_empty().unwrap();
        let queen = Piece::new(Color::White, PieceType::Queen, true);

        let square = Square::build(4,4).unwrap();
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
        let board = Board::build_empty().unwrap();
        let queen = Piece::new(Color::White, PieceType::Queen, true);

        let square = Square::build(0,4).unwrap();
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
        let board = Board::build_empty().unwrap();
        let queen = Piece::new(Color::White, PieceType::Queen, true);

        let square = Square::build(0,0).unwrap();
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
        let mut board = Board::build_empty().unwrap();
        let color = Color::White;

        let square_bad = Square::build(4, 7).unwrap();
        let queen = Piece{
            color: color,
            piece_type: PieceType::Queen,
            has_moved: true,
        };
        board.add_piece(queen, square_bad.file, square_bad.rank);

        let square = Square::build(4,4).unwrap();
        let queen_moves = queen.get_moves(&board, &square);

        assert!(!queen_moves.contains(&&square_bad));
    }
}

#[cfg(test)]
mod pawn_tests {
    use super::*;

    #[test]
    fn white_pawn_starting_rank_empty_board() {
        let board = Board::build_empty().unwrap();
        let square = Square::build(4,1).unwrap();
        let pawn = Piece::new(Color::White, PieceType::Pawn, false);

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
        let board = Board::build_empty().unwrap();
        let square = Square::build(4,2).unwrap();
        let pawn = Piece::new(Color::White, PieceType::Pawn, true);

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
        let board = Board::build_empty().unwrap();
        let square = Square::build(4,6).unwrap();
        let pawn = Piece::new(Color::Black, PieceType::Pawn, false);

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
        let board = Board::build_empty().unwrap();
        let square = Square::build(4,5).unwrap();
        let pawn = Piece::new(Color::Black, PieceType::Pawn, true);

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
        let mut board = Board::build_empty().unwrap();

        let square = Square::build(4,2).unwrap();
        board.add_piece(Piece::new(Color::Black, PieceType::Pawn,true), square.file, square.rank);

        let square = Square::build(4,1).unwrap();
        let pawn = Piece::new(Color::White, PieceType::Pawn, false);

        let pawn_moves = pawn.get_moves(&board, &square);

        assert!(pawn_moves.is_empty());
    }

    #[test]
    fn white_pawn_blocked_in_front_by_friend() {
        let mut board = Board::build_empty().unwrap();

        let square = Square::build(4,2).unwrap();
        board.add_piece(Piece::new(Color::White, PieceType::Pawn, true), square.file, square.rank);

        let square = Square::build(4,1).unwrap();
        let pawn = Piece::new(Color::White, PieceType::Pawn, false);

        let pawn_moves = pawn.get_moves(&board, &square);

        assert!(pawn_moves.is_empty());
    }

    #[test]
    fn white_pawn_far_blocked_in_front_by_opponent() {
        let mut board = Board::build_empty().unwrap();

        let square = Square::build(4,3).unwrap();
        board.add_piece(Piece::new(Color::Black, PieceType::Pawn, true), square.file, square.rank);

        let square = Square::build(4,1).unwrap();
        let pawn = Piece::new(Color::White, PieceType::Pawn, false);

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
        let mut board = Board::build_empty().unwrap();

        let square = Square::build(4,3).unwrap();
        board.add_piece(Piece::new(Color::White, PieceType::Pawn, true), square.file, square.rank);

        let square = Square::build(4,1).unwrap();
        let pawn = Piece::new(Color::White, PieceType::Pawn, false);

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
        let mut board = Board::build_empty().unwrap();

        let square = Square::build(3,2).unwrap();
        board.add_piece(Piece::new(Color::Black, PieceType::Pawn, true), square.file, square.rank);
        let square = Square::build(5,2).unwrap();
        board.add_piece(Piece::new(Color::Black, PieceType::Pawn, true), square.file, square.rank);

        let square = Square::build(4,1).unwrap();
        let pawn = Piece::new(Color::White, PieceType::Pawn, false);

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
        let mut board = Board::build_empty().unwrap();

        let square = Square::build(3,2).unwrap();
        board.add_piece(Piece::new(Color::White, PieceType::Pawn, true), square.file, square.rank);
        let square = Square::build(5,2).unwrap();
        board.add_piece(Piece::new(Color::White, PieceType::Pawn, true), square.file, square.rank);

        let square = Square::build(4,1).unwrap();
        let pawn = Piece::new(Color::White, PieceType::Pawn, false);

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
        let mut board = Board::build_empty().unwrap();

        let square = Square::build(4,5).unwrap();
        board.add_piece(Piece::new(Color::White, PieceType::Pawn, true), square.file, square.rank);

        let square = Square::build(4,6).unwrap();
        let pawn = Piece::new(Color::Black, PieceType::Pawn, false);

        let pawn_moves = pawn.get_moves(&board, &square);

        assert!(pawn_moves.is_empty());
    }

    #[test]
    fn black_pawn_blocked_in_front_by_friend() {
        let mut board = Board::build_empty().unwrap();

        let square = Square::build(4,5).unwrap();
        board.add_piece(Piece::new(Color::Black, PieceType::Pawn, true), square.file, square.rank);

        let square = Square::build(4,6).unwrap();
        let pawn = Piece::new(Color::Black, PieceType::Pawn, false);

        let pawn_moves = pawn.get_moves(&board, &square);

        assert!(pawn_moves.is_empty());
    }

    #[test]
    fn black_pawn_far_blocked_in_front_by_opponent() {
        let mut board = Board::build_empty().unwrap();

        let square = Square::build(4,4).unwrap();
        board.add_piece(Piece::new(Color::White, PieceType::Pawn, true), square.file, square.rank);

        let square = Square::build(4,6).unwrap();
        let pawn = Piece::new(Color::Black, PieceType::Pawn, false);

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
        let mut board = Board::build_empty().unwrap();

        let square = Square::build(4,4).unwrap();
        board.add_piece(Piece::new(Color::Black, PieceType::Pawn, true), square.file, square.rank);

        let square = Square::build(4,6).unwrap();
        let pawn = Piece::new(Color::Black, PieceType::Pawn, false);

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
        let mut board = Board::build_empty().unwrap();

        let square = Square::build(3,5).unwrap();
        board.add_piece(Piece::new(Color::White, PieceType::Pawn, true), square.file, square.rank);
        let square = Square::build(5,5).unwrap();
        board.add_piece(Piece::new(Color::White, PieceType::Pawn, true), square.file, square.rank);

        let square = Square::build(4,6).unwrap();
        let pawn = Piece::new(Color::Black, PieceType::Pawn, false);

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
        let mut board = Board::build_empty().unwrap();

        let square = Square::build(3,5).unwrap();
        board.add_piece(Piece::new(Color::Black, PieceType::Pawn, true), square.file, square.rank);
        let square = Square::build(5,5).unwrap();
        board.add_piece(Piece::new(Color::Black, PieceType::Pawn, true), square.file, square.rank);

        let square = Square::build(4,6).unwrap();
        let pawn = Piece::new(Color::Black, PieceType::Pawn, false);

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
