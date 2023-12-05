use crate::{board::Board, piece::Piece};

mod board;
mod castling_rights;
mod colour;
mod errors;
mod moves;
mod piece;

fn main() {
    println!("Hello, world!");

    let mut board = Board::from_fen("8/8/5N2/3N4/4Q3/8/8/7N w QKqk").unwrap();
    // let mut board = Board::from_fen("N7/8/8/8/8/8/8/8 w QKqk").unwrap();

    let moves = board.generate_moves();
    // println!("{:?} => {}", moves, moves.len());
    for v_move in moves.iter() {
        println!("{}", repr_move(*v_move));
    }
    println!("move count: {}", moves.len());

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

fn repr_move(v_move: u16) -> String {
    let departure_square = v_move & 0b111111;
    let target_square = (v_move & 0b111111000000) >> 6;
    let (departure_file, departure_rank) = square_to_coods(departure_square);
    let (target_file, target_rank) = square_to_coods(target_square);
    format!(
        "{}{} to {}{}",
        get_file_letter(departure_file),
        departure_rank + 1,
        get_file_letter(target_file),
        target_rank + 1
    )
}

fn square_to_coods(square: u16) -> (u16, u16) {
    let rank = (square as f32 / 8.).floor();
    let file = square as f32 - rank * 8.;

    (file as u16, rank as u16)
}

const FILE_LETTERS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
fn get_file_letter(file: u16) -> char {
    FILE_LETTERS[file as usize]
}
