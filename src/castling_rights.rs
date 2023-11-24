pub struct CastlingRights;

impl CastlingRights {
    pub const WhiteCanCastle: u8 = 0b00000000;
    pub const WhiteCanNotCastle: u8 = 0b00000001;
    pub const WhiteCanNotShortCastle: u8 = 0b00000010;
    pub const WhiteCanNotLongCastle: u8 = 0b00000011;

    pub const BlackCanCastle: u8 = 0b00000000;
    pub const BlackCanNotCastle: u8 = 0b00010000;
    pub const BlackCanNotShortCastle: u8 = 0b00100000;
    pub const BlackCanNotLongCastle: u8 = 0b00110000;
}
