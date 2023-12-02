use crate::{board::Board, moves::Move, piece::Piece};

mod board;
mod castling_rights;
mod colour;
mod errors;
mod moves;
mod piece;

fn main() {
    println!("Hello, world!");

    let mut board = Board::from_fen("8/8/8/2BRR/8/8/8/8 w QKqk").unwrap();

    let moves = board.generate_moves();
    println!("{:?} => {}", moves, moves.len());

    println!("{}", board.stringify(Piece::White));

    // let v_move = Move::new_move(1, 0, 2, 2);
    // board.make_move(v_move);
    // let v_move = Move::new_move(3, 1, 3, 2);
    // board.make_move(v_move);
    // let v_move = Move::new_move(2, 0, 5, 3);
    // board.make_move(v_move);

    // let v_move = Move::new_move(0, 0, 4, 7) | Move::PromoteToQueen | Move::Capture;
    // board.make_move(v_move);

    // let v_move = Move::ShortCastle;
    // board.make_move(v_move);

    // let v_move = Move::ShortCastle;
    // board.make_move(v_move);

    // println!("{}", board.stringify(Piece::White));
    // println!("{}", v_move);
}
