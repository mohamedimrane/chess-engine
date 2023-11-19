use std::fmt::Display;

use crate::{errors::FenError, piece::Piece};

pub struct Board {
    pieces: [u8; 64],
}

impl Board {
    pub fn make_move(&mut self, moveToMake: u16) {}

    pub fn new() -> Self {
        let mut board = Self::default();

        board.pieces[0] = Piece::White | Piece::Rook;
        board.pieces[1] = Piece::White | Piece::Knight;
        board.pieces[2] = Piece::White | Piece::Bishop;
        board.pieces[3] = Piece::White | Piece::Queen;
        board.pieces[4] = Piece::White | Piece::King;
        board.pieces[5] = Piece::White | Piece::Bishop;
        board.pieces[6] = Piece::White | Piece::Knight;
        board.pieces[7] = Piece::White | Piece::Rook;
        for x in 0..8 {
            board.pieces[8 + x] = Piece::White | Piece::Pawn;
        }

        board.pieces[56] = Piece::Black | Piece::Rook;
        board.pieces[57] = Piece::Black | Piece::Knight;
        board.pieces[58] = Piece::Black | Piece::Bishop;
        board.pieces[59] = Piece::Black | Piece::Queen;
        board.pieces[60] = Piece::Black | Piece::King;
        board.pieces[61] = Piece::Black | Piece::Bishop;
        board.pieces[62] = Piece::Black | Piece::Knight;
        board.pieces[63] = Piece::Black | Piece::Rook;
        for x in 0..8 {
            board.pieces[48 + x] = Piece::Black | Piece::Pawn;
        }

        board
    }

    fn from_fen(fen: &str) -> Result<Self, FenError> {
        let splited_fen: Vec<&str> = fen.split('/').collect();
        let mut board = Self::default();

        let pieces = splited_fen[0].chars();
        let mut file: usize = 0;
        let mut rank: usize = 0;
        for c in pieces {
            if c == '/' {
                rank += 1;
                file = 0;

                if rank > 8 {
                    return Err(FenError::RankTooBig(rank));
                }
                continue;
            }

            if c.is_numeric() {
                let num = c.to_digit(10).unwrap() as usize;
                if num + file > 8 {
                    return Err(FenError::FileTooBig(num));
                }
                file += num;
                continue;
            }

            if c.is_alphabetic() {
                let color = match c {
                    c if c.is_uppercase() => Piece::White,
                    c if c.is_lowercase() => Piece::Black,
                    _ => return Err(FenError::CharNotReconized),
                };

                let kind = match c.to_lowercase().next().unwrap() {
                    'p' => Piece::Pawn,
                    'n' => Piece::Knight,
                    'b' => Piece::Bishop,
                    'r' => Piece::Rook,
                    'q' => Piece::Queen,
                    'k' => Piece::King,
                    _ => return Err(FenError::CharNotReconized),
                };

                board.pieces[rank * 8 + file] = color | kind;
                file += 1;
                continue;
            }
        }

        Ok(board)
    }

    pub fn stringify(&self, perspective: u8) -> String {
        let mut string = String::new();

        for rank in 0..8 {
            string.push_str(&"----".repeat(8));
            string.push('\n');
            string.push_str("| ");
            for file in 0..8 {
                let perspective = match perspective {
                    Piece::White => (7 - rank, file),
                    Piece::Black => (rank, 7 - file),
                    _ => panic!("strigification: invalid perspective"),
                };

                let piece = self.pieces[perspective.0 * 8 + perspective.1];
                let piece_color = Piece::colour(piece);
                let piece_type = Piece::pieceType(piece);

                let mut piece_repr = match piece_type {
                    Piece::None => ' ',
                    Piece::Pawn => 'p',
                    Piece::Knight => 'n',
                    Piece::Bishop => 'b',
                    Piece::Rook => 'r',
                    Piece::Queen => 'q',
                    Piece::King => 'k',
                    _ => unreachable!(),
                };

                if piece_color == Piece::White {
                    piece_repr = piece_repr.to_uppercase().next().unwrap();
                }

                string.push(piece_repr);
                string.push_str(" | ");
            }
            string.push('\n');
        }
        string.push_str(&"----".repeat(8));

        string
    }
}

impl Default for Board {
    fn default() -> Self {
        Self { pieces: [0; 64] }
    }
}
