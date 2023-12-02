use crate::colour::Colour;

#[non_exhaustive]
/// A piece is represented by 6 bits
///        `00`->color `0000`->kind
pub struct Piece;

impl Piece {
    pub const None: u8 = 0;

    pub const Pawn: u8 = 0b1; // 0001 => 1
    pub const Knight: u8 = 0b10; // 0010 => 2
    pub const Bishop: u8 = 0b11; // 0011 => 3
    pub const Rook: u8 = 0b100; // 0100 => 4
    pub const Queen: u8 = 0b101; // 0101 => 5
    pub const King: u8 = 0b110; // 0111 => 6

    pub const White: u8 = 0b00010000; // 1000 => 16
    pub const Black: u8 = 0b00100000; // 10000 => 32

    const typeMask: u8 = 0b00001111;

    const whiteMask: u8 = 0b00010000;
    const blackMask: u8 = 0b00100000;
    const colourMask: u8 = Self::whiteMask | Self::blackMask;

    pub fn colour(piece: u8) -> u8 {
        piece & Self::colourMask
    }

    pub fn colour_bool(piece: u8) -> bool {
        let colour = piece & Self::colourMask;
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
        piece & Self::typeMask
    }

    pub fn is_colour(piece: u8, colour: u8) -> bool {
        piece & Self::colourMask == colour
    }

    pub fn is_colour_bool(piece: u8, colour: bool) -> bool {
        let colour = match colour {
            Colour::White => Piece::White,
            Colour::Black => Piece::Black,
        };

        piece & Self::colourMask == colour
    }
}
