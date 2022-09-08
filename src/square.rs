use crate::rank::Rank;
use crate::file::File;

use crate::piece::Piece;

#[derive(Debug)]
pub struct Square {
    pub rank: Rank,
    pub file: File,

    pub piece: Option<Box<dyn Piece>>,
}

impl Square {
    pub fn build(rank: u8, file: u8) -> Result<Square, &'static str> {
        let rank = Rank::build(rank)?;
        let file = File::build(file)?;
        let piece: Option<Box<dyn Piece>> = None;
        Ok(Square{rank, file, piece})
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn file(&self) -> File {
        self.file
    }
}

impl PartialEq for Square {
    fn eq(&self, other: &Square) -> bool {
        self.rank.value() == other.rank.value() &&
            self.file.value() == other.file.value()
    }
}
