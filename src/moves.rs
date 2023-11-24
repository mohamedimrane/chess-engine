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

    pub const departure_file_mask: u16 = 0b0000000000000111;
    pub const departure_rank_mask: u16 = 0b0000000000111000;
    pub const target_file_mask: u16 = 0b0000000111000000;
    pub const target_rank_mask: u16 = 0b0000111000000000;

    pub const capture_mask: u16 = 0b0001000000000000;

    pub const promotion_mask: u16 = 0b0010000000000000;

    pub const castling_mask: u16 = 0b1100000000000000;

    pub const special_one_mask: u16 = 0b0100000000000000;
    pub const special_two_mask: u16 = 0b1000000000000000;

    pub fn new_move(
        departure_file: u16,
        departure_rank: u16,
        target_file: u16,
        target_rank: u16,
    ) -> u16 {
        departure_file | departure_rank << 3 | target_file << 6 | target_rank << 9
    }

    pub fn departure_file(v_move: u16) -> u8 {
        (v_move & Move::departure_file_mask) as u8
    }

    pub fn departure_rank(v_move: u16) -> u8 {
        ((v_move & Move::departure_rank_mask) >> 3) as u8
    }

    pub fn target_file(v_move: u16) -> u8 {
        ((v_move & Move::target_file_mask) >> 6) as u8
    }

    pub fn target_rank(v_move: u16) -> u8 {
        ((v_move & Move::target_rank_mask) >> 9) as u8
    }

    pub fn capture(v_move: u16) -> bool {
        (v_move & Move::capture_mask) == Self::Capture
    }

    pub fn is_promotion(v_move: u16) -> bool {
        (v_move & Move::promotion_mask) == Self::promotion_mask
    }

    pub fn promotion_type(v_move: u16) -> u16 {
        v_move >> 13 << 13
    }

    pub fn is_castling(v_move: u16) -> bool {
        (v_move & Move::castling_mask) == Self::castling_mask
    }

    pub fn special_one(v_move: u16) -> bool {
        (v_move & Move::special_one_mask) == Self::special_one_mask
    }

    pub fn special_two(v_move: u16) -> bool {
        (v_move & Move::special_two_mask) == Self::special_two_mask
    }
}
