#[derive(Debug)]
pub enum FenError<'a> {
    NotEnoughParts,
    UnknownPiece(char),
    NoSuchSide(&'a str),
    BadCastlingCharacter(&'a str),
    BadEnPassant(&'a str),
    BadHalfmove(&'a str),
    BadFullmove(&'a str),
}

// pub enum FenError {
//     CharNotReconized,
//     FileTooBig(usize),
//     RankTooBig(usize),
//     InvalidColour,
//     InvalidCastlingCharacter,
//     NotEnoughFields,
// }

// pub enum FenError<'a> {
//     NotEnoughParts,
//     BadPlacement(&'a str),
//     TooManyPieces(&'a str),
//     UnknownPiece(char),
//     NoSuchSide(&'a str),
//     BadEnPassant(&'a str),
//     BadHalfmove(&'a str),
//     BadFullmove(&'a str),
// }

#[derive(Debug)]
pub enum MoveError {
    InvalidMove,
    MissingSquares,
    InvalidFile,
    InvalidRank,
}

#[derive(Debug)]
pub enum UndoMoveError {
    EmptyStack,
}
