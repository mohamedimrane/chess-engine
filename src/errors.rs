#[derive(Debug)]
pub enum FenError {
    CharNotReconized,
    FileTooBig(usize),
    RankTooBig(usize),
    InvalidColor,
    InvalidCastlingCharacter,
}

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
