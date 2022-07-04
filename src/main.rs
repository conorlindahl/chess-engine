mod rank;
mod file;
mod square;

use crate::rank::Rank;
use crate::file::File;
use crate::square::Square;

fn main() {
    let sq = Square{
        rank: Rank::R4,
        file: File::H,
    };
    println!("The best square is {:?}", sq);
}
