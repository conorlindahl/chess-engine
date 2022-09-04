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
    let rank = Rank::build(4).expect("Error creating a rank");
    let file = File::build(7).expect("Error creating a file");
    let sq = Square{
        rank: rank,
        file: file,
    };
    println!("The best square is {:?}", sq);
}
