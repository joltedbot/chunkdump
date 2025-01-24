use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LocalError {
    #[error("Could not process the supplied file path")]
    InvalidPath,

    #[error("Could extract the filename from the supplied path")]
    InvalidFileName,

    #[error("Incorrect WAVEID, file is not a valid RIFF WAVE file")]
    InvalidWaveID,

    #[error("Incorrect INFO chunk type, {0}")]
    InvalidInfoTypeID(String),

    #[error("Incorrect ADTL chunk type, {0}")]
    InvalidADTLTypeID(String),

    #[error("Requested number of bytes {0} is greater than the available bytes: {1}")]
    InsufficientBytesToTake(usize, usize),
}
