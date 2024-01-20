#[non_exhaustive]
pub struct Move;
/// A move is represented by 16 bits.
/// 00->special moves (castling or promotion type)
/// 0->promotion
/// 0->en passant
/// 000000->target square
/// 000000->departure square

#[allow(non_upper_case_globals, dead_code)]
impl Move {
    pub const PromoteToKnight: u16 = 0b0010000000000000;
    pub const PromoteToBishop: u16 = 0b0110000000000000;
    pub const PromoteToRook: u16 = 0b1010000000000000;
    pub const PromoteToQueen: u16 = 0b1110000000000000;

    pub const ShortCastle: u16 = 0b0100000000000000;
    pub const LongCastle: u16 = 0b1000000000000000;

    pub const EnPassant: u16 = 0b0001000000000000;

    pub const DEPARTURE_SQUARE_MASK: u16 = 0b0000000000111111;
    pub const TARGET_SQUARE_MASK: u16 = 0b0000111111000000;

    pub const EN_PASSANT_MASK: u16 = 0b0001000000000000;

    pub const PROMOTION_MASK: u16 = 0b0010000000000000;

    pub const CASTLING_MASK: u16 = 0b1100000000000000;

    pub const SPECIAL_ONE_MASK: u16 = 0b0100000000000000;
    pub const SPECIAL_TWO_MASK: u16 = 0b1000000000000000;

    #[allow(clippy::new_ret_no_self)]
    pub fn new(departure_square: u16, target_square: u16) -> u16 {
        departure_square | target_square << 6
    }

    pub fn departure_square(v_move: u16) -> u8 {
        (v_move & Self::DEPARTURE_SQUARE_MASK) as u8
    }

    pub fn target_square(v_move: u16) -> u8 {
        ((v_move & Self::TARGET_SQUARE_MASK) >> 6) as u8
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

    pub fn is_en_passant(v_move: u16) -> bool {
        (v_move & Self::EN_PASSANT_MASK) == Self::EnPassant
    }

    pub fn is_promotion(v_move: u16) -> bool {
        (v_move & Self::PROMOTION_MASK) == Self::PROMOTION_MASK
    }

    pub fn promotion_type(v_move: u16) -> u16 {
        v_move >> 13 << 13
    }

    pub fn is_castling(v_move: u16) -> bool {
        (v_move & Self::CASTLING_MASK) == Self::ShortCastle
            || (v_move & Self::CASTLING_MASK) == Self::LongCastle && !Self::is_promotion(v_move)
    }

    pub fn is_short_castling(v_move: u16) -> bool {
        (v_move & Self::CASTLING_MASK) == Self::ShortCastle && !Self::is_promotion(v_move)
    }

    pub fn is_long_castling(v_move: u16) -> bool {
        (v_move & Self::CASTLING_MASK) == Self::LongCastle && !Self::is_promotion(v_move)
    }

    pub fn special_one(v_move: u16) -> bool {
        (v_move & Self::SPECIAL_ONE_MASK) == Self::SPECIAL_ONE_MASK
    }

    pub fn special_two(v_move: u16) -> bool {
        (v_move & Self::SPECIAL_TWO_MASK) == Self::SPECIAL_TWO_MASK
    }
}

pub struct MoveRecord {
    pub v_move: u16,
    pub piece_on_target_square: u8,
}

impl MoveRecord {
    pub fn new(v_move: u16, piece_on_target_square: u8) -> Self {
        Self {
            v_move,
            piece_on_target_square,
        }
    }
}
