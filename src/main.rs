mod rank;
mod file;
mod square;
mod board;
mod piece;

use crate::rank::Rank;
use crate::file::File;
use crate::square::Square;
use crate::board::Board;
use crate::piece::Piece;
use crate::piece::king::King;

fn main() {
    let sq = Square{
        rank: Rank::R4,
        file: File::H,
    };
    println!("The best square is {:?}", sq);
}
