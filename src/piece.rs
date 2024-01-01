use crate::colour::Colour;

#[non_exhaustive]
/// A piece is represented by 6 bits
///        `00`->color `0000`->kind
pub struct Piece;

#[allow(non_upper_case_globals, dead_code)]
impl Piece {
    pub const None: u8 = 0;

    pub const Pawn: u8 = 0b1; // 0001 => 1
    pub const Knight: u8 = 0b10; // 0010 => 2
    pub const Bishop: u8 = 0b11; // 0011 => 3
    pub const Rook: u8 = 0b100; // 0100 => 4
    pub const Queen: u8 = 0b101; // 0101 => 5
    pub const King: u8 = 0b110; // 0111 => 6

    pub const PawnValue: i32 = 100;
    pub const KnightValue: i32 = 300;
    pub const BishopValue: i32 = 300;
    pub const RookValue: i32 = 500;
    pub const QueenValue: i32 = 900;

    pub const White: u8 = 0b00010000; // 1000 => 16
    pub const Black: u8 = 0b00100000; // 10000 => 32

    const TYPE_MASK: u8 = 0b00001111;

    const WHITE_MASK: u8 = 0b00010000;
    const BLACK_MASK: u8 = 0b00100000;
    const COLOUR_MASK: u8 = Self::WHITE_MASK | Self::BLACK_MASK;

    pub fn colour(piece: u8) -> u8 {
        piece & Self::COLOUR_MASK
    }

    pub fn colour_bool(piece: u8) -> bool {
        let colour = piece & Self::COLOUR_MASK;
        match colour {
            Self::White => Colour::White,
            Self::Black => Colour::Black,
            _ => unreachable!(),
        }
    }

    pub fn is_sliding_piece(piece: u8) -> bool {
        matches!(
            Self::piece_type(piece),
            Piece::Queen | Piece::Rook | Piece::Bishop
        )
    }

    pub fn piece_type(piece: u8) -> u8 {
        piece & Self::TYPE_MASK
    }

    pub fn is_colour(piece: u8, colour: u8) -> bool {
        piece & Self::COLOUR_MASK == colour
    }

    pub fn is_type(piece: u8, v_type: u8) -> bool {
        piece & Self::TYPE_MASK == v_type
    }

    pub fn is_colour_bool(piece: u8, colour: bool) -> bool {
        let colour = match colour {
            Colour::White => Piece::White,
            Colour::Black => Piece::Black,
        };

        piece & Self::COLOUR_MASK == colour
    }
}
