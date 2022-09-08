#[derive(Debug, Clone, Copy)]
pub struct Rank {
    val: u8,
}

pub const MAX_NUMBER_OF_RANKS: u8 = 8;

impl Rank {
    pub fn build(rank: u8) -> Result<Rank, &'static str> {
        if rank >= MAX_NUMBER_OF_RANKS {
            return Err("Rank outside allowable bounds");
        }
        Ok(Rank{val: rank})
    }

    pub fn value(&self) -> u8 {
        self.val
    }

    pub fn next(&self) -> Option<Rank> {
        if self.val + 1 >= MAX_NUMBER_OF_RANKS {
            None
        } else {
            Some(Rank{val: self.val+1})
        }
    }

    pub fn previous(&self) -> Option<Rank> {
        if self.val == 0 {
            None
        } else {
            Some(Rank{val: self.val - 1})
        }
    }
}