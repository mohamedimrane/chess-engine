use lazy_static::lazy_static;
use std::cmp::min;

use crate::{
    castling_rights::CastlingRights, colour::Colour, errors::FenError, moves::Move, piece::Piece,
};

lazy_static! {
    static ref NUM_SQUARES_TO_EDGE: [[u8; 8]; 64] = {
        let mut res = [[0_u8; 8]; 64];

        for file in 0..8 {
            for rank in 0..8 {
                let num_north = 7 - rank;
                let num_south = rank;
                let num_west = file;
                let num_east = 7 - file;

                let square_index = 8 * rank + file;

                res[square_index] = [
                    num_north as u8,
                    num_south as u8,
                    num_west as u8,
                    num_east as u8,
                    min(num_north, num_west) as u8,
                    min(num_south, num_east) as u8,
                    min(num_north, num_east) as u8,
                    min(num_south, num_west) as u8,
                ];
            }
        }

        res
    };
}

const DIRECTION_OFFSETS: [i8; 8] = [8, -8, -1, 1, 7, -7, 9, -9];
const KNIGHTS_OFFSETS: [(i8, i8); 8] = [
    (2, 1),
    (2, -1),
    (1, 2),
    (-1, 2),
    (-2, 1),
    (-2, -1),
    (1, -2),
    (-1, -2),
];

pub struct Board {
    pieces: [u8; 64],
    colour_to_move: bool,
    castling_rights: u8,
}

impl Board {
    pub fn make_move(&mut self, v_move: u16) {
        let departure_square = Move::departure_square(v_move) as usize;
        let target_square = Move::target_square(v_move) as usize;
        let promotion = Move::is_promotion(v_move);
        let castling = Move::is_castling(v_move);
        let special_one = Move::special_one(v_move);
        let special_two = Move::special_two(v_move);

        let active_colour = self.colour_to_move;

        self.colour_to_move = !self.colour_to_move;

        if promotion {
            let promotion_type = Move::promotion_type(v_move);
            let piece_to_promote_to = match promotion_type {
                Move::PromoteToKnight => Piece::Knight,
                Move::PromoteToBishop => Piece::Bishop,
                Move::PromoteToRook => Piece::Rook,
                Move::PromoteToQueen => Piece::Queen,
                _ => unreachable!(),
            };
            let piece_colour = Piece::colour(self.pieces[departure_square]);

            self.pieces[target_square] = piece_colour | piece_to_promote_to;
            self.pieces[departure_square] = Piece::None;

            return;
        }

        if castling {
            let castling_rights = CastlingRights::rights(self.castling_rights, active_colour);

            if castling_rights == CastlingRights::CanNotCastle {
                return;
            }

            let (king_file, king_rank) = match active_colour {
                Colour::White => (4, 0),
                Colour::Black => (4, 7),
                _ => unreachable!(),
            };

            if special_one && CastlingRights::can_short_castle(castling_rights) {
                self.castling_rights = match active_colour {
                    Colour::White => {
                        self.castling_rights >> 4 << 4 | CastlingRights::WhiteCanNotCastle
                    }
                    Colour::Black => {
                        self.castling_rights << 4 >> 4 | CastlingRights::BlackCanNotCastle
                    }
                    _ => unreachable!(),
                };

                let king_index = king_rank * 8 + king_file;
                let king = self.pieces[king_index];
                let rook = self.pieces[king_index + 3];
                self.pieces[king_index] = Piece::None;
                self.pieces[king_index + 3] = Piece::None;
                self.pieces[king_index + 2] = king;
                self.pieces[king_index + 1] = rook;
            }

            if special_two && CastlingRights::can_long_castle(castling_rights) {
                self.castling_rights = match active_colour {
                    Colour::White => {
                        self.castling_rights >> 4 << 4 | CastlingRights::WhiteCanNotCastle
                    }
                    Colour::Black => {
                        self.castling_rights << 4 >> 4 | CastlingRights::BlackCanNotCastle
                    }
                    _ => unreachable!(),
                };

                let king_index = king_rank * 8 + king_file;
                let king = self.pieces[king_index];
                let rook = self.pieces[king_index - 4];
                self.pieces[king_index] = Piece::None;
                self.pieces[king_index - 4] = Piece::None;
                self.pieces[king_index - 2] = king;
                self.pieces[king_index - 1] = rook;
            }

            return;
        }

        self.pieces[target_square] = self.pieces[departure_square];
        self.pieces[departure_square] = Piece::None;
    }

    pub fn generate_moves(&self) -> Vec<u16> {
        let mut moves = Vec::new();

        for (start_square, piece) in self.pieces.iter().enumerate() {
            let piece = *piece;

            if piece == Piece::None {
                continue;
            }

            let piece_color = Piece::colour_bool(piece);

            if piece_color != self.colour_to_move {
                continue;
            }

            let piece_type = Piece::piece_type(piece);

            if Piece::is_sliding_piece(piece) {
                let (dir_start, dir_end) = match piece_type {
                    Piece::Queen => (0, 8),
                    Piece::Rook => (0, 4),
                    Piece::Bishop => (4, 8),
                    _ => unreachable!(),
                };

                for dir_index in dir_start..dir_end {
                    for n in 0..NUM_SQUARES_TO_EDGE[start_square][dir_index] {
                        let target_square =
                            start_square as i8 + DIRECTION_OFFSETS[dir_index] * (n as i8 + 1);

                        if !(0..=63).contains(&target_square) {
                            continue;
                        }

                        let piece_on_target_square = self.pieces[target_square as usize];

                        if Piece::is_colour_bool(piece_on_target_square, self.colour_to_move) {
                            break;
                        }

                        let m_move = Move::new(start_square as u16, target_square as u16);
                        moves.push(m_move);

                        if Piece::is_colour_bool(piece_on_target_square, !self.colour_to_move) {
                            break;
                        }
                    }
                }

                continue;
            }

            if Piece::is_type(piece_type, Piece::Knight) {
                for n in KNIGHTS_OFFSETS {
                    let (start_file, start_rank) = square_to_coods(start_square as u16);
                    let target_file = start_file as i8 + n.0;
                    let target_rank = start_rank as i8 + n.1;

                    if !(0..=7).contains(&target_file) || !(0..=7).contains(&target_rank) {
                        continue;
                    }

                    let target_square = target_rank * 8 + target_file;

                    if !(0..=63).contains(&target_square) {
                        continue;
                    }

                    let piece_on_target_square = self.pieces[target_square as usize];

                    if Piece::is_colour_bool(piece_on_target_square, self.colour_to_move) {
                        continue;
                    }

                    let v_move = Move::new(start_square as u16, target_square as u16);

                    moves.push(v_move);
                }

                continue;
            }
        }

        moves
    }

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

    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        let splited_fen: Vec<&str> = fen.split(' ').collect();
        let mut board = Self::default();

        board.colour_to_move = match splited_fen[1] {
            "w" => Colour::White,
            "b" => Colour::Black,
            _ => return Err(FenError::InvalidColor),
        };

        for right in splited_fen[2].chars() {
            board.castling_rights |= match right {
                'K' => CastlingRights::WhiteCanShortCastle,
                'Q' => CastlingRights::WhiteCanLongCastle,
                'k' => CastlingRights::BlackCanShortCastle,
                'q' => CastlingRights::BlackCanLongCastle,
                '-' => {
                    board.castling_rights =
                        CastlingRights::WhiteCanNotCastle | CastlingRights::BlackCanNotCastle;
                    break;
                }
                _ => return Err(FenError::InvalidCastlingCharacter),
            }
        }

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

                board.pieces[(7 - rank) * 8 + file] = color | kind;
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
                let piece_type = Piece::piece_type(piece);

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
        Self {
            pieces: [0; 64],
            colour_to_move: Colour::White,
            castling_rights: CastlingRights::WhiteCanCastle | CastlingRights::BlackCanCastle,
        }
    }
}

fn square_to_coods(square: u16) -> (u16, u16) {
    let rank = (square as f32 / 8.).floor();
    let file = square as f32 - rank * 8.;

    (file as u16, rank as u16)
}
