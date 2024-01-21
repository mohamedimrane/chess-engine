use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

use errors::{MoveError, SquareParsingError};
use moves::Move;
use utils::{get_file_number, string_to_square};

use crate::{
    board::Board,
    colour::Colour,
    utils::{get_file_letter, square_to_coods},
};

// use colored::*;
use color_print::cprintln;

mod board;
mod castling_rights;
mod colour;
mod errors;
mod moves;
mod piece;
mod utils;

type Result<T> = core::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    // let mut board = Board::from_fen("8/8/8/8/8/8/8/8 w QKqk - 0 1").unwrap();
    // let mut board =
    //     Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w QKqk - 0 1").unwrap();
    // let mut board = Board::new();
    // let mut board = Board::from_fen("8/5ppp/p1p3P1/1P2P3/5p2/6p1/5PP1/8 w - - 0 1").unwrap();
    let mut board =
        Board::from_fen("8/3p1ppp/p1p3P1/1PP1P3/p1p2p2/6p1/1P3PP1/8 w - - 0 1").unwrap();
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

fn get_line() -> Result<String> {
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

fn make_move(move_string: &str, moves: &[u16], board: &mut Board) -> Result<()> {
    let v_move = process_move(move_string)?;

    if !moves.contains(&v_move) {
        return Err(Box::new(MoveError::InvalidMove));
    }

    board.make_move(v_move);

    Ok(())
}

fn process_move(string: &str) -> Result<u16> {
    if string == "o-o" || string == "O-O" || string == "0-0" {
        return Ok(Move::ShortCastle);
    }

    if string == "o-o-o" || string == "O-O-O" || string == "0-0-0" {
        return Ok(Move::LongCastle);
    }

    let departure_square = string
        .get(0..=1)
        .ok_or(SquareParsingError::NotEnoughParts)?;

    let target_square = string
        .get(2..=3)
        .ok_or(SquareParsingError::NotEnoughParts)?;

    let departure_square = string_to_square(departure_square)?;
    let target_square = string_to_square(target_square)?;

    let en_passant_flag = match string.get(4..=4) {
        Some(x) => {
            if x == "*" {
                Move::EnPassant
            } else {
                0
            }
        }
        None => 0,
    };

    Ok(departure_square as u16 | (target_square as u16) << 6 | en_passant_flag)
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

    let en_passant = if Move::is_en_passant(v_move) { "*" } else { "" };

    format!(
        "{}{}{}{}{}",
        get_file_letter(departure_file).unwrap(),
        departure_rank + 1,
        get_file_letter(target_file).unwrap(),
        target_rank + 1,
        en_passant
    )
}
