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
}
