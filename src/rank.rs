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

    fn new(rank: u8) -> Rank {
        Rank{val: rank}
    }

    pub fn iter_ranks(range: impl Iterator<Item = u8>) -> impl Iterator<Item = Rank> {
        range.filter(|rank| rank < &MAX_NUMBER_OF_RANKS).map(|rank| Rank::new(rank))
    }

    pub fn value(&self) -> u8 {
        self.val
    }

    pub fn next_by(&self, jump: u8) -> Result<Rank, &'static str> {
        Rank::build(self.val + jump)
    }

    pub fn previous_by(&self, jump: u8) -> Result<Rank, &'static str> {
        Rank::build(self.val - jump)
    }
}

use std::cmp::PartialEq;
use std::cmp::Eq;
impl PartialEq for Rank {
    fn eq(&self, other: &Rank) -> bool {
        self.val == other.val
    }
}
impl Eq for Rank {}

use std::cmp::Ord;
use std::cmp::PartialOrd;
use std::cmp::Ordering;
impl Ord for Rank {
    fn cmp(&self, other: &Rank) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Rank) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}