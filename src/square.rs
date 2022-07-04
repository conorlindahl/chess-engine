use crate::rank::Rank;
use crate::file::File;

#[derive(Debug, Clone, Copy)]
pub struct Square {
    pub rank: Rank,
    pub file: File,
}

impl Square {
    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn file(&self) -> File {
        self.file
    }

    fn left(&self) -> Option<Square> {
        match self.file.previous() {
            None => None,
            Some(f) => Some(
                Square{
                    rank: self.rank,
                    file: f,
                }
            )
        }
    }

    fn right(&self) -> Option<Square> {
        match self.file.next() {
            None => None,
            Some(f) => Some(
                Square{
                    rank: self.rank,
                    file: f,
                }
            )
        }
    }

    fn up(&self) -> Option<Square> {
        match self.rank.next() {
            None => None,
            Some(r) => Some(
                Square{
                    rank: r,
                    file: self.file,
                }
            )
        }
    }

    fn down(&self) -> Option<Square> {
        match self.rank.previous() {
            None => None,
            Some(r) => Some(
                Square{
                    rank: r,
                    file: self.file,
                }
            )
        }
    }
}
