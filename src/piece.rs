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

    pub const White: u8 = 0b10000; // 1000 => 8
    pub const Black: u8 = 0b100000; // 10000 => 16
}
