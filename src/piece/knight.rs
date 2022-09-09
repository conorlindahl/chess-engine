use crate::board::Board;
use crate::piece::Color;
use crate::file::File;
use crate::piece::Piece;
use crate::rank::Rank;
use crate::square::Square;

#[derive(Debug, Clone, Copy)]
pub struct Knight {
    color: Color,
}

impl Piece for Knight {
    fn get_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
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
            }).collect::<Vec<&Square>>()
        }).collect()
    }

    fn get_color(&self) -> Color {
        return self.color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn central_square_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(4,4).unwrap();
        let knight = Knight{color: Color::White};
        
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

        assert!(valid_moves.len() == knight_moves.len());
        
        assert!(
            valid_moves.iter().all(|m| {
                knight_moves.contains(&m)
            })
        );
    }

    #[test]
    fn edge_square_empty_board() {
        let board = Board::build().unwrap();
        let square = Square::build(0,4).unwrap();
        let knight = Knight{color: Color::White};
        
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
        let board = Board::build().unwrap();
        let square = Square::build(0,0).unwrap();
        let knight = Knight{color: Color::White};
        
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

}