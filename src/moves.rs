#[non_exhaustive]
pub struct Move;
/// A move is represented by 16 bits.
/// 00->special moves (castling or promotion type)
/// 0->promotion
/// 0->capture
/// 000000->target square
/// 000000->departure square

impl Move {
    pub const PromoteToKnight: u16 = 0b0010000000000000;
    pub const PromoteToBishop: u16 = 0b0110000000000000;
    pub const PromoteToRook: u16 = 0b1010000000000000;
    pub const PromoteToQueen: u16 = 0b1110000000000000;

    pub const ShortCastle: u16 = 0b0100000000000000;
    pub const LongCastle: u16 = 0b1000000000000000;

    pub const Capture: u16 = 0b0001000000000000;

    pub const departure_square_mask: u16 = 0b0000000000111111;
    pub const target_square_mask: u16 = 0b0000111111000000;

    pub const capture_mask: u16 = 0b0001000000000000;

    pub const promotion_mask: u16 = 0b0010000000000000;

    pub const castling_mask: u16 = 0b1100000000000000;

    pub const special_one_mask: u16 = 0b0100000000000000;
    pub const special_two_mask: u16 = 0b1000000000000000;

    pub fn new_move(departure_square: u16, target_square: u16) -> u16 {
        departure_square | target_square << 6
    }

    pub fn departure_square(v_move: u16) -> u8 {
        (v_move & Self::departure_square_mask) as u8
    }

    pub fn target_square(v_move: u16) -> u8 {
        (v_move & Self::target_square_mask) as u8
    }

    // pub fn departure_file(v_move: u16) -> u8 {
    //     (v_move & Self::departure_file_mask) as u8
    // }

    // pub fn departure_rank(v_move: u16) -> u8 {
    //     ((v_move & Self::departure_rank_mask) >> 3) as u8
    // }

    // pub fn target_file(v_move: u16) -> u8 {
    //     ((v_move & Self::target_file_mask) >> 6) as u8
    // }

    // pub fn target_rank(v_move: u16) -> u8 {
    //     ((v_move & Self::target_rank_mask) >> 9) as u8
    // }

    pub fn capture(v_move: u16) -> bool {
        (v_move & Self::capture_mask) == Self::Capture
    }

    pub fn is_promotion(v_move: u16) -> bool {
        (v_move & Self::promotion_mask) == Self::promotion_mask
    }

    pub fn promotion_type(v_move: u16) -> u16 {
        v_move >> 13 << 13
    }

    pub fn is_castling(v_move: u16) -> bool {
        (v_move & Self::castling_mask) == Self::ShortCastle
            || (v_move & Self::castling_mask) == Self::LongCastle && !Self::is_promotion(v_move)
    }

    pub fn special_one(v_move: u16) -> bool {
        (v_move & Self::special_one_mask) == Self::special_one_mask
    }

    pub fn special_two(v_move: u16) -> bool {
        (v_move & Self::special_two_mask) == Self::special_two_mask
    }
}
