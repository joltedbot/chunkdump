use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LocalError {
    #[error("Could not process the supplied file path")]
    InvalidPath,

    #[error("Could extract the filename from the supplied path")]
    InvalidFileName,

    #[error("Incorrect WAVEID, file is not a valid RIFF WAVE file")]
    InvalidWaveID,
}
