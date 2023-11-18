use crate::{errors::FenError, piece::Piece};

pub struct Board {
    pieces: [u8; 64],
}

impl Board {
    fn new() -> Self {
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
}

impl Default for Board {
    fn default() -> Self {
        Self { pieces: [0; 64] }
    }
}
