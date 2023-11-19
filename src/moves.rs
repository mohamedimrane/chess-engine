#[non_exhaustive]
pub struct Move;
/// A move is represented by 16 bits.
/// 00->special moves (castling or promotion type)
/// 0->promotion
/// 0->capture
/// 000->target rank
/// 000->target file
/// 000->departure rank
/// 000->departure file

impl Move {
    pub const PromoteToKnight: u16 = 0b0010000000000000;
    pub const PromoteToBishop: u16 = 0b0110000000000000;
    pub const PromoteToRook: u16 = 0b1010000000000000;
    pub const PromoteToQueen: u16 = 0b1110000000000000;

    pub const ShortCastle: u16 = 0b0100000000000000;
    pub const LongCastle: u16 = 0b1000000000000000;

    pub const Capture: u16 = 0b0001000000000000;

    pub const departureFileMask: u16 = 0b0000000000000111;
    pub const departureRankMask: u16 = 0b0000000000111000;
    pub const targetFileMask: u16 = 0b0000000111000000;
    pub const targetRankMask: u16 = 0b0000111000000000;

    pub const captureMask: u16 = 0b0001000000000000;

    pub fn new_move(
        departure_file: u16,
        departure_rank: u16,
        target_file: u16,
        target_rank: u16,
    ) -> u16 {
        departure_file & departure_rank << 3 & target_file << 6 & target_rank << 9
    }

    pub fn departureFile(v_move: u16) -> u8 {
        (v_move & Move::departureFileMask) as u8
    }

    pub fn departureRank(v_move: u16) -> u8 {
        (v_move & Move::departureRankMask) as u8
    }

    pub fn targetFile(v_move: u16) -> u8 {
        (v_move & Move::targetFileMask) as u8
    }

    pub fn targetRank(v_move: u16) -> u8 {
        (v_move & Move::targetRankMask) as u8
    }

    pub fn capture(v_move: u16) -> u8 {
        (v_move & Move::captureMask) as u8
    }
}
