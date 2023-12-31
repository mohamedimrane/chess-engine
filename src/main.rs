use std::io::BufRead;

use errors::MoveError;

use crate::board::Board;

use colored::*;

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
        let moves = board.generate_moves();

        match line.as_str() {
            "listmoves" | "list" | "moves" => {
                list_moves(&moves);
            }

            a if a.contains("play") || a.contains("move") => 'blk: {
                let move_str = line.split_whitespace().collect::<Vec<_>>()[1];
                let move_str = move_str.to_string();

                let move_err = make_move(move_str.clone(), &moves, &mut board);
                if let Err(e) = move_err {
                    println!("{} ({:?}): {}", "invalid move".red(), e, move_str);
                    // match e {
                    //     MoveError::InvalidMove => {
                    //         println!("{} {}", "invalid move:".red(), move_str);
                    //     }
                    //     MoveError::MissingSquares => {
                    //         println!("{} {}", "missing squares: ".red(), move_str);
                    //     }
                    //     MoveError::InvalidFile => {
                    //         println!("{} {}", "invalid file: ".red(), move_str);
                    //     }
                    //     MoveError::InvalidRank => {
                    //         println!("{} {}", "invalid rank: ".red(), move_str);
                    //     }
                    // };

                    break 'blk;
                }
            }

            _ => {
                println!("{} {}", "invalid command:".red(), line);
            }
        }

        // println!("\n");
    }
}

fn list_moves(moves: &Vec<u16>) {
    println!(
        "moves ({}): {:?}",
        moves.len(),
        moves.iter().map(|m| repr_move(*m)).collect::<Vec<_>>()
    );
}

fn make_move(move_string: String, moves: &[u16], board: &mut Board) -> Result<(), MoveError> {
    let v_move = process_move(move_string.to_string())?;

    if !moves.contains(&v_move) {
        return Err(MoveError::InvalidMove);
    }

    board.make_move(v_move);

    println!("{} {}", "played move:".green(), move_string);

    Ok(())
}

fn process_move(string: String) -> Result<u16, MoveError> {
    let chars: Vec<char> = string.chars().collect();

    let departure_file = chars.get(0);
    let departure_rank = chars.get(1);
    let target_file = chars.get(2);
    let target_rank = chars.get(3);

    if departure_file.is_none()
        || departure_rank.is_none()
        || target_file.is_none()
        || target_rank.is_none()
    {
        return Err(MoveError::MissingSquares);
    }

    let departure_file = get_file_number(*departure_file.unwrap());
    let target_file = get_file_number(*target_file.unwrap());

    if let (None, None) = (departure_file, target_file) {
        return Err(MoveError::InvalidRank);
    }

    let departure_rank = departure_rank.unwrap().to_digit(10);
    let target_rank = target_rank.unwrap().to_digit(10);
    if let (None, None) = (departure_rank, target_rank) {
        return Err(MoveError::InvalidRank);
    }

    let departure_rank = departure_rank.unwrap() as u16 - 1;
    let target_rank = target_rank.unwrap() as u16 - 1;

    let departure_square = departure_rank * 8 + departure_file.unwrap();
    let target_square = target_rank * 8 + target_file.unwrap();

    Ok(departure_square | target_square << 6)
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

fn get_file_number(file: char) -> Option<u16> {
    FILE_LETTERS
        .iter()
        .position(|&a| a == file)
        .map(|x| x as u16)
}
