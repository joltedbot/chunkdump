use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LocalError {
    #[error("Could not process the supplied file path")]
    InvalidPath,

    #[error("Could not extract the filename from the supplied path")]
    InvalidFileName,

    #[error("Could not read from the file {0}")]
    CouldNotReadFile(String),

    #[error("Incorrect WAVEID, file is not a valid RIFF WAVE file")]
    InvalidWaveID,

    #[error("File {0} is not a valid WAVE file")]
    InvalidWaveFile(String),

    #[error("Could not extract metadata from the file: {0}")]
    CouldNotExtractMetaData(String),

    #[error("Incorrect INFO chunk type, {0}")]
    InvalidInfoTypeID(String),

    #[error("Incorrect ADTL chunk type, {0}")]
    InvalidADTLTypeID(String),

    #[error("Requested number of bytes {0} is greater than the available bytes: {1}")]
    InsufficientBytesToTake(usize, usize),

    #[error("Could not create template store")]
    CouldNotCreateTemplateStore,

    #[error("Could note find the requested output template")]
    InvalidOutputTemplate,
}
