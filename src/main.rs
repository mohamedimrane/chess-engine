use std::io::BufRead;

use errors::MoveError;
use moves::Move;

use crate::{board::Board, colour::Colour};

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
    // let mut board = Board::from_fen("8/8/8/1p1p3/8/2P4/8/8 w QKqk").unwrap();
    // let mut board = Board::from_fen("8/8/8/3pP2/8/8/8/8 w QKqk").unwrap();
    // let mut board = Board::from_fen("8/pppppppp/PPPP4/8/8/8/PPPPPPPP/8 w QKqk").unwrap();
    // let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w QKqk").unwrap();
    let mut board = Board::from_fen("8/5ppp/p1p3P1/1P2P3/5p2/6p1/5PP1/8 w - - 0 1").unwrap();
    // let mut board = Board::from_fen("7b/1r2N1pp/3k4/Q7/3n3K/2p5/R1PP4/1n1q1B2 w -").unwrap();
    // let mut board = Board::from_fen("r3k2r/p6p/P6P/8/8/p6p/P6P/R3K2R w KQkq").unwrap();

    for line in std::io::stdin().lock().lines().map(|r| r.unwrap()) {
        let moves = board.generate_moves();

        let line_split = line.split_once(' ');

        let verb;
        let mut args = Vec::new();

        match line_split {
            None => verb = line.as_str(),
            Some(line_split) => {
                verb = line_split.0;
                args = line_split.1.split_whitespace().collect();
            }
        }

        match verb {
            "" => {}

            "listmoves" | "list" | "moves" | "ls" => {
                println!(
                    "{} {:?}",
                    format!("moves ({}):", moves.len()).green(),
                    moves.iter().map(|m| repr_move(*m)).collect::<Vec<_>>()
                );
            }

            "show" | "display" | "board" => {
                println!("{}", board.stringify(Colour::White));
            }

            "unmake" | "undo" => {
                if let Err(e) = board.undo_move() {
                    println!("{}{:?}", "cannot undo: ".red(), e);
                }
            }

            "eval" | "evaluate" | "evaluation" => {
                let eval_string = match board.evaluate() {
                    n if n == 0 => "0".normal(),
                    n if n > 0 => format!("{}", n).bright_green(),
                    n if n < 0 => format!("{}", n).bright_red(),
                    _ => unreachable!(),
                };

                println!("{}", format!("evaluation: {}", eval_string).green());
            }

            "play" | "move" => 'blk: {
                // let command_strs = line.split_whitespace().collect::<Vec<_>>();

                // let move_str = command_strs[1].to_string();

                let move_str = match args.first() {
                    Some(m) => *m,
                    None => {
                        println!("{}", "arg error: no move provided".red());
                        break 'blk;
                    }
                };

                let move_err = make_move(move_str, &moves, &mut board);
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

        print!("\n{}\n\n", "-".repeat(60));
    }
}

fn make_move(move_string: &str, moves: &[u16], board: &mut Board) -> Result<(), MoveError> {
    let v_move = process_move(move_string)?;

    if !moves.contains(&v_move) {
        return Err(MoveError::InvalidMove);
    }

    board.make_move(v_move);

    println!("{} {}", "played move:".green(), move_string);

    Ok(())
}

fn process_move(string: &str) -> Result<u16, MoveError> {
    if string == "o-o" || string == "O-O" || string == "0-0" {
        return Ok(Move::ShortCastle);
    }

    if string == "o-o-o" || string == "O-O-O" || string == "0-0-0" {
        return Ok(Move::LongCastle);
    }

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
