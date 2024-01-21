use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum SquareParsingError {
    NotEnoughParts,
    BadFile(char),
    BadRank(char),
}

#[derive(Debug)]
pub enum UndoMoveError {
    EmptyStack,
}

#[derive(Debug)]
pub enum MoveError {
    InvalidMove,
    InvalidFile,
    InvalidRank,
}

#[derive(Debug)]
pub enum FenError<'a> {
    NotEnoughParts,
    TooManyPieces(&'a str),
    UnknownPiece(char),
    BadPlacement(&'a str),
    NoSuchSide(&'a str),
    BadCastlingCharacter(&'a str),
    BadEnPassant(&'a str),
    BadHalfmove(&'a str),
    BadFullmove(&'a str),
}

impl Display for SquareParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SquareParsingError::NotEnoughParts => write!(f, "not enough parts in square notation"),
            SquareParsingError::BadFile(file) => {
                write!(f, "bad file {} while parsing square", file)
            }
            SquareParsingError::BadRank(rank) => {
                write!(f, "bad file {} while parsing square", rank)
            }
        }
    }
}

impl Display for UndoMoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UndoMoveError::EmptyStack => write!(f, "cannot undo move because stack is empty"),
        }
    }
}

impl Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "dd"),
        }
    }
}

impl Display for FenError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FenError::NotEnoughParts => write!(f, "not enough parts in FEN string"),
            FenError::TooManyPieces(a) => write!(f, "too many pieces {} in FEN string", a),
            FenError::UnknownPiece(a) => write!(f, "unknown piece {} in FEN string", a),
            FenError::BadPlacement(a) => write!(f, "bad placement {} in FEN string", a),
            FenError::NoSuchSide(a) => write!(f, "no such side {} in FEN string", a),
            FenError::BadCastlingCharacter(a) => {
                write!(f, "bad castling character {} in FEN string", a)
            }
            FenError::BadEnPassant(a) => write!(f, "bad en passant square {} in FEN string", a),
            FenError::BadHalfmove(a) => write!(f, "bad half move counter {} in FEN string", a),
            FenError::BadFullmove(a) => write!(f, "bad full move counter {} in FEN string", a),
            _ => write!(f, "error message lol"),
        }
    }
}

impl Error for SquareParsingError {}

impl Error for UndoMoveError {}

impl Error for MoveError {}

impl Error for FenError<'_> {}
