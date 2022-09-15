use crate::board::Board;
use crate::piece::Color;
use crate::piece::Piece;
use crate::square::Square;

#[derive(Debug, Clone, Copy)]
pub struct Bishop {
    color: Color,
}

impl Piece for Bishop {
    fn get_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
        // Distance is # of steps to edge square
        board.get_diagonals(square).iter().filter_map(|&sq| {
            if sq.rank != square.rank && sq.file != square.file && !sq.piece_matches_color(self.color) {
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

    #[test]
    fn central_piece_near_friendly_piece() {
        let mut board = Board::build().unwrap();
        let color = Color::Black;

        let square_bad = Square::build(1, 7).unwrap();
        board.add_piece(Rc::new(Bishop{color}), square_bad.file, square_bad.rank);

        let square = Square::build(4,4).unwrap();
        let bishop = Bishop{color};

        let bishop_moves = bishop.get_moves(&board, &square);

        assert!(!bishop_moves.contains(&&square_bad));
    }
}
