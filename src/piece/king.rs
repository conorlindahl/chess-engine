use crate::square::Square;
use crate::piece::Piece;

pub struct King {
    square: Square,
}

/* Idea for an improvement: Create a 3x3 array of Option<T>. If left/right/up/down are None, fill out the adjacent values with None.
 * Only calculate squares that aren't adjacent to None.
 */
impl Piece for King {
    fn get_moves(&self) -> Vec<Square> {
        let mut moves = Vec::new();

        let left = self.square.left();
        let right = self.square.right();
        let up = self.square.up();
        let down = self.square.up();

        let left_up = match left {
            Some(sq_left) => sq_left.up(),
            None => None,
        };
        let left_down = match left {
            Some(sq_left) => sq_left.down(),
            None => None,
        };
        let right_up = match right {
            Some(sq_right) => sq_right.up(),
            None => None,
        };
        let right_down = match right {
            Some(sq_right) => sq_right.down(),
            None => None,
        };

        if let Some(sq) = left { moves.push(sq); }
        if let Some(sq) = left_up { moves.push(sq); }
        if let Some(sq) = up { moves.push(sq); }
        if let Some(sq) = right_up { moves.push(sq); }
        if let Some(sq) = right { moves.push(sq); }
        if let Some(sq) = right_down { moves.push(sq); }
        if let Some(sq) = down { moves.push(sq); }
        if let Some(sq) = left_down { moves.push(sq); }

        moves
    }
}