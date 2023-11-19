use crate::{board::Board, piece::Piece};

mod board;
mod errors;
mod moves;
mod piece;

fn main() {
    println!("Hello, world!");
    let board = Board::new();
    println!("{}", board.stringify(Piece::White));
}
