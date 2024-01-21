use crate::{
    errors::{MoveError, SquareParsingError},
    Result,
};

const FILE_LETTERS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

pub fn get_file_letter(file: u16) -> Result<char> {
    FILE_LETTERS
        .get(file as usize)
        .copied()
        .ok_or(Box::new(MoveError::InvalidFile))
}

pub fn get_file_number(file: char) -> Result<u16> {
    FILE_LETTERS
        .iter()
        .position(|&a| a == file)
        .map(|x| x as u16)
        .ok_or(Box::new(SquareParsingError::BadFile(file)))
}

pub fn square_to_coods(square: u16) -> (u16, u16) {
    let rank = (square as f32 / 8.).floor();
    let file = square as f32 - rank * 8.;

    (file as u16, rank as u16)
}
