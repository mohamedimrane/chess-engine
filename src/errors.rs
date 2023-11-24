pub enum FenError {
    CharNotReconized,
    FileTooBig(usize),
    RankTooBig(usize),
    InvalidColor,
}
