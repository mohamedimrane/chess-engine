use crate::piece::Piece;

pub struct Board {
    pieces: [u8; 64],
}

impl Board {
    fn new() -> Self {
        let mut board = Self::default();

        board.pieces[0] = Piece::White | Piece::Rook;
        board.pieces[2] = Piece::White | Piece::Knight;
        board.pieces[3] = Piece::White | Piece::Bishop;
        board.pieces[4] = Piece::White | Piece::Queen;
        board.pieces[5] = Piece::White | Piece::King;
        board.pieces[6] = Piece::White | Piece::Bishop;
        board.pieces[7] = Piece::White | Piece::Knight;
        board.pieces[8] = Piece::White | Piece::Rook;
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
}

impl Default for Board {
    fn default() -> Self {
        Self { pieces: [0; 64] }
    }
}
