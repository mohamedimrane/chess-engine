use crate::colour::Color;

pub struct CastlingRights;

impl CastlingRights {
    pub const WhiteCanCastle: u8 = 0b00000011;
    pub const WhiteCanNotCastle: u8 = 0b00000000;
    pub const WhiteCanShortCastle: u8 = 0b00000001;
    pub const WhiteCanLongCastle: u8 = 0b00000010;

    pub const BlackCanCastle: u8 = 0b00110000;
    pub const BlackCanNotCastle: u8 = 0b00000000;
    pub const BlackCanShortCastle: u8 = 0b00010000;
    pub const BlackCanLongCastle: u8 = 0b00100000;

    pub const CanCastle: u8 = 0b00000011;
    pub const CanNotCastle: u8 = 0b00000000;
    pub const CanShortCastle: u8 = 0b00000001;
    pub const CanLongCastle: u8 = 0b00000010;

    pub const white_rights_mask: u8 = 0b00001111;
    pub const black_rights_mask: u8 = 0b11110000;

    pub fn white_rights(rights: u8) -> u8 {
        rights & Self::white_rights_mask
    }

    pub fn black_rights(rights: u8) -> u8 {
        (rights & Self::black_rights_mask) >> 4
    }

    pub fn rights(rights: u8, colour: bool) -> u8 {
        match colour {
            Color::White => Self::white_rights(rights),
            Color::Black => Self::black_rights(rights),
            _ => unreachable!(),
        }
    }

    pub fn can_short_castle(rights: u8) -> bool {
        rights & Self::CanShortCastle == Self::CanShortCastle
    }

    pub fn can_long_castle(rights: u8) -> bool {
        rights & Self::CanLongCastle == Self::CanLongCastle
    }
}
