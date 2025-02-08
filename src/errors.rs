use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LocalError {
    #[error("Could not process the supplied file path")]
    InvalidPath,

    #[error("Could not extract the filename from the supplied path")]
    InvalidFileName,

    #[error("{0} is not a supported file type")]
    UnsupportedFileType(String),

    #[error("Could not read from the file {0}")]
    CouldNotReadFile(String),

    #[error("Incorrect WAVEID, file is not a valid RIFF WAVE file")]
    InvalidWaveID,

    #[error("Could not read metadata from the file: {0}")]
    CouldNotReadData(String),

    #[error("Incorrect INFO chunk type, {0}")]
    InvalidInfoTypeID(String),

    #[error("Incorrect ADTL chunk type, {0}")]
    InvalidADTLTypeID(String),

    #[error("Requested number of bytes {0} is greater than the available bytes: {1}")]
    InsufficientBytesToTake(usize, usize),

    #[error("Output File {0} Already Exists")]
    OutputFileAlreadyExists(String),
}
