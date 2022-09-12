mod rank;
mod file;
mod square;
mod board;
mod piece;

use crate::rank::Rank;
use crate::file::File;
use crate::square::Square;
use crate::piece::Piece;

use std::rc::Rc;

fn main() {
    let rank = Rank::build(4).expect("Error creating a rank");
    let file = File::build(7).expect("Error creating a file");
    let piece: Option<Rc<dyn Piece>> = None;
    let sq = Square{
        rank,
        file,
        piece
    };
    println!("The best square is {:?}", sq);
}
