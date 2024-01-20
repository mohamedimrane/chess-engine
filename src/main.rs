use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

use errors::MoveError;
use moves::Move;

use crate::{board::Board, colour::Colour};

// use colored::*;
use color_print::cprintln;

mod board;
mod castling_rights;
mod colour;
mod errors;
mod moves;
mod piece;

fn main() -> Result<(), Box<dyn Error>> {
    // let mut board = Board::from_fen("8/8/8/8/8/8/8/8 w QKqk - 0 1").unwrap();
    // let mut board =
    //     Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w QKqk - 0 1").unwrap();
    // let mut board = Board::new();
    // let mut board = Board::from_fen("8/5ppp/p1p3P1/1P2P3/5p2/6p1/5PP1/8 w - - 0 1").unwrap();
    let mut board = Board::from_fen("8/3p1ppp/p1p3P1/1P2P3/2p2p2/6p1/1P3PP1/8 w - - 0 1").unwrap();
    // let mut board = Board::from_fen("r3k2r/p6p/P6P/8/8/p6p/P6P/R3K2R w KQkq - 0 1").unwrap();

    // for line in std::io::stdin().lock().lines().map(|r| r.unwrap()) {
    loop {
        let moves = board.generate_moves();

        let line = get_line()?;

        let (command, args) = process_line(line);

        match command.as_str() {
            "" => {}

            "ep" => {
                cprintln!("<green>en passant square:</> {:?}", board.en_passant_square);
            }

            "listmoves" | "list" | "moves" | "ls" => {
                let moves_list = moves.iter().map(|&m| repr_move(m)).collect::<Vec<_>>();
                cprintln!("<green>moves ({}):</> {:?}", moves.len(), moves_list);
            }

            "show" | "display" | "board" => {
                cprintln!("{}", board.stringify(Colour::White));
            }

            "unmake" | "undo" => {
                if let Err(e) = board.undo_move() {
                    cprintln!("<red>cannot undo:</> {:?}", e);
                }
            }

            "eval" | "evaluate" | "evaluation" => {
                let eval = board.evaluate();
                let eval_string = if eval > 0 {
                    "+".to_string() + &eval.to_string()
                } else {
                    eval.to_string()
                };

                cprintln!("<green>evaluation:</> <bold, blue>{}</>", eval_string);
            }

            "play" | "move" => 'blk: {
                let Some(args) = args else {
                    cprintln!("<red>not enough arguments: no move provided</>");
                    break 'blk;
                };

                let move_str = match args.first() {
                    Some(m) => m,
                    None => {
                        cprintln!("<red>args error: cannot get arg</>");
                        break 'blk;
                    }
                };

                let move_err = make_move(move_str, &moves, &mut board);
                if let Err(e) = move_err {
                    cprintln!("<red>invalid move</> ({:?}): {}", e, move_str);
                    break 'blk;
                }
                cprintln!("<green>played move:</> {}", move_str);
            }

            _ => {
                cprintln!("<red>invalid command:</> {}", command);
            }
        }
    }
}

fn get_line() -> Result<String, Box<dyn Error>> {
    print!("> ");
    stdout().flush()?;

    let mut line = String::new();
    stdin().read_line(&mut line)?;
    if let Some('\n') = line.chars().next_back() {
        line.pop();
    }
    if let Some('\r') = line.chars().next_back() {
        line.pop();
    }

    Ok(line)
}

fn process_line(line: String) -> (String, Option<Vec<String>>) {
    let command;
    let mut args = None;

    match line.split_once(' ') {
        None => command = line.trim().to_string(),
        Some(line_split) => {
            command = line_split.0.trim().to_string();
            args = Some(
                line_split
                    .1
                    .split_whitespace()
                    .map(|arg| arg.trim().to_string())
                    .collect(),
            );
        }
    }

    (command, args)
}

fn make_move(move_string: &str, moves: &[u16], board: &mut Board) -> Result<(), MoveError> {
    let v_move = process_move(move_string, board)?;

    if !moves.contains(&v_move) {
        return Err(MoveError::InvalidMove);
    }

    board.make_move(v_move);

    Ok(())
}

fn process_move(string: &str, board: &Board) -> Result<u16, MoveError> {
    if string == "o-o" || string == "O-O" || string == "0-0" {
        return Ok(Move::ShortCastle);
    }

    if string == "o-o-o" || string == "O-O-O" || string == "0-0-0" {
        return Ok(Move::LongCastle);
    }

    let chars: Vec<char> = string.chars().collect();

    #[allow(clippy::get_first)]
    let departure_file = match chars.get(0) {
        Some(x) if x.is_alphabetic() => get_file_number(*x)?,
        _ => return Err(MoveError::InvalidFile),
    };
    let target_file = match chars.get(2) {
        Some(x) if x.is_alphabetic() => get_file_number(*x)?,
        _ => return Err(MoveError::InvalidFile),
    };
    let departure_rank = match chars.get(1) {
        Some(x) if x.is_ascii_digit() => x
            .to_digit(10)
            .expect("failed converting departure rank to number"),
        _ => return Err(MoveError::InvalidRank),
    } as u16
        - 1;
    let target_rank = match chars.get(3) {
        Some(x) if x.is_ascii_digit() => x
            .to_digit(10)
            .expect("failed converting target rank to number"),
        _ => return Err(MoveError::InvalidRank),
    } as u16
        - 1;

    let double_forward_pawn_move_flag = match chars.get(4) {
        Some(&x) => {
            if x == '*' {
                Move::DoubleForwardPawnMove
            } else {
                0
            }
        }
        None => 0,
    };

    // let second_rank = match colour {
    //     Colour::White => (8, 15),
    //     Colour::Black => (47, 55),
    // };

    let departure_square = departure_rank * 8 + departure_file;
    let target_square = target_rank * 8 + target_file;
    // let flags = if pie

    // if !(second_rank.0..=second_rank.1).contains(&start_square) {
    //     continue;
    // }

    Ok(departure_square | target_square << 6 | double_forward_pawn_move_flag)
}

fn repr_move(v_move: u16) -> String {
    if Move::is_short_castling(v_move) {
        return "O-O".to_string();
    }

    if Move::is_long_castling(v_move) {
        return "O-O-O".to_string();
    }

    let departure_square = v_move & 0b111111;
    let target_square = (v_move & 0b111111000000) >> 6;
    let (departure_file, departure_rank) = square_to_coods(departure_square);
    let (target_file, target_rank) = square_to_coods(target_square);
    let double_forward_pawn_move = if Move::is_double_forward_pawn_move(v_move) {
        "*"
    } else {
        ""
    };
    format!(
        "{}{}{}{}{}",
        get_file_letter(departure_file).unwrap(),
        departure_rank + 1,
        get_file_letter(target_file).unwrap(),
        target_rank + 1,
        double_forward_pawn_move
    )
}

fn square_to_coods(square: u16) -> (u16, u16) {
    let rank = (square as f32 / 8.).floor();
    let file = square as f32 - rank * 8.;

    (file as u16, rank as u16)
}

const FILE_LETTERS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
fn get_file_letter(file: u16) -> Result<char, MoveError> {
    FILE_LETTERS
        .get(file as usize)
        .copied()
        .ok_or(MoveError::InvalidFile)
}

fn get_file_number(file: char) -> Result<u16, MoveError> {
    FILE_LETTERS
        .iter()
        .position(|&a| a == file)
        .map(|x| x as u16)
        .ok_or(MoveError::InvalidFile)
}
