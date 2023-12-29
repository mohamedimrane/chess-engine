use std::io::BufRead;

use crate::{board::Board, piece::Piece};

mod board;
mod castling_rights;
mod colour;
mod errors;
mod moves;
mod piece;

fn main() {
    // let mut board = Board::from_fen("8/8/5N2/3N4/4Q3/2KR2R1/PPPPPPPP/7N w QKqk").unwrap();
    // let mut board = Board::from_fen("8/8/8/8/8/8/8/8 w QKqk").unwrap();
    let mut board = Board::from_fen("8/pppppppp/PPPP4/8/8/8/PPPPPPPP/8 w QKqk").unwrap();

    for line in std::io::stdin().lock().lines().map(|r| r.unwrap()) {
        print!("> ");
        let moves = board.generate_moves();

        'blk: {
            match line.as_str() {
                "listmoves" | "list" | "moves" => {
                    println!(
                        "moves ({}): {:?}",
                        moves.len(),
                        moves.iter().map(|m| repr_move(*m)).collect::<Vec<_>>()
                    );
                }

                a if a.contains("play") || a.contains("move") => {
                    let move_string = line.split_whitespace().collect::<Vec<_>>()[1];

                    // println!("{:?}", move_string);
                    let v_move = process_move(move_string.to_string());

                    if !moves.contains(&v_move) {
                        println!("invalid move: {}", v_move);
                        break 'blk;
                    }

                    // board.make_move(v_move);

                    // println!()
                }

                _ => {
                    println!("invalid command: {}", line);
                }
            }
        }

        println!("\n");
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

    departure_square | target_square << 6
}

fn repr_move(v_move: u16) -> String {
    let departure_square = v_move & 0b111111;
    let target_square = (v_move & 0b111111000000) >> 6;
    let (departure_file, departure_rank) = square_to_coods(departure_square);
    let (target_file, target_rank) = square_to_coods(target_square);
    format!(
        "{}{}{}{}",
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
