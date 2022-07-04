#[derive(Debug, Clone, Copy)]
pub enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

impl Rank {
    pub fn next(&self) -> Option<Rank> {
        match self {
            Rank::R1 => Some(Rank::R2),
            Rank::R2 => Some(Rank::R3),
            Rank::R3 => Some(Rank::R4),
            Rank::R4 => Some(Rank::R5),
            Rank::R5 => Some(Rank::R6),
            Rank::R6 => Some(Rank::R7),
            Rank::R7 => Some(Rank::R8),
            Rank::R8 => None,
        }
    }

    pub fn previous(&self) -> Option<Rank> {
        match self {
            Rank::R1 => None,
            Rank::R2 => Some(Rank::R1),
            Rank::R3 => Some(Rank::R2),
            Rank::R4 => Some(Rank::R3),
            Rank::R5 => Some(Rank::R4),
            Rank::R6 => Some(Rank::R5),
            Rank::R7 => Some(Rank::R6),
            Rank::R8 => Some(Rank::R7),
        }
    }
}