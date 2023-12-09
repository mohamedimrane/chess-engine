use std::io::BufRead;

use moves::Move;

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

    // board.make_move(Move::new(7, 13));

    println!("{}", board.stringify(Piece::White));

    let moves = board.generate_moves();
    println!("move count: {}", moves.len());

    for line in std::io::stdin().lock().lines().map(|r| r.unwrap()) {
        print!("\n\n\n");

        let moves = board.generate_moves();
        println!("move count: {}", moves.len());

        let v_move = process_move(line);

        if !moves.contains(&v_move) {
            println!("Invalid move");
            continue;
        }

        board.make_move(v_move);

        println!("{}", board.stringify(Piece::White));
    }
}

fn process_move(string: String) -> u16 {
    let chars: Vec<char> = string.chars().collect();

    let (departure_file, departure_rank, target_file, target_rank) =
        (chars[0], chars[1], chars[2], chars[3]);

    let (departure_file, departure_rank, target_file, target_rank) = (
        get_file_number(departure_file),
        departure_rank.to_digit(10).unwrap() as u16 - 1,
        get_file_number(target_file),
        target_rank.to_digit(10).unwrap() as u16 - 1,
    );

    let departure_square = departure_rank * 8 + departure_file;
    let target_square = target_rank * 8 + target_file;

    let v_move = departure_square | target_square << 6;

    v_move
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

fn get_file_number(file: char) -> u16 {
    FILE_LETTERS.iter().position(|&a| a == file).unwrap() as u16
}
