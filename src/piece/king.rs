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

/* Idea for an improvement: Create a 3x3 array of Option<T>. If left/right/up/down are None, fill out the adjacent values with None.
 * Only calculate squares that aren't adjacent to None.
 */
impl Piece for King {
    fn get_moves<'a>(&self, board: &'a Board, square: &Square) -> Vec<&'a Square> {
        let rank_val = square.rank().value();
        let file_val = square.file().value();

        Rank::iter_ranks((rank_val-1)..=(rank_val+1)).flat_map(|rank| {
            File::iter_files((file_val-1)..=(file_val+1)).filter(move |file| {
                !(rank.value() == rank_val && file.value() == file_val)
            }).map(move |file| {
                board.get_square(file, rank)
            })
        }).collect()
    }

    fn get_color(&self) -> Color {
        return self.color;
    }
}