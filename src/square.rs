use crate::rank::Rank;
use crate::file::File;

use crate::piece::Color;
use crate::piece::Piece;

use std::rc::Rc;

#[derive(Debug)]
pub struct Square {
    pub file: File,
    pub rank: Rank,

    pub piece: Option<Rc<dyn Piece>>,
}

impl Square {
    pub fn build(file: u8, rank: u8) -> Result<Square, &'static str> {
        let rank = Rank::build(rank)?;
        let file = File::build(file)?;
        let piece: Option<Rc<dyn Piece>> = None;
        Ok(Square{file, rank, piece})
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn file(&self) -> File {
        self.file
    }

    pub fn is_empty(&self) -> bool {
        self.piece.is_none()
    }

    pub fn piece_matches_color(&self, color: Color) -> bool {
        match &self.piece {
            Some(piece) => piece.get_color() == color,
            None => false
        }
    }
}

impl PartialEq for Square {
    fn eq(&self, other: &Square) -> bool {
        self.rank.value() == other.rank.value() &&
            self.file.value() == other.file.value()
    }
}
