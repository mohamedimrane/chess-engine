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
}
