use crate::cli::print_usage_message;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LocalError {
    #[error("Could not process the supplied file path: {0}")]
    InvalidPath(String),

    #[error("Could not extract the filename from the supplied path")]
    InvalidFileName,

    #[error("Unsupported file type. Only Wave and Flac files are supported")]
    UnsupportedFileType,

    #[error("Could not read from the file {0}")]
    CouldNotReadFile(String),

    #[error("Incorrect WAVEID, file is not a valid RIFF WAVE file")]
    InvalidWaveID,

    #[error("Could not read metadata from the file: {0}")]
    CouldNotReadData(String),

    #[error("Incorrect ADTL chunk type, {0}")]
    InvalidADTLTypeID(String),

    #[error("Could not unzip ResU chunk JSON data: {0}")]
    InvalidZipDataFound(String),

    #[error("Could not extract ID3 tags from the file: {0}")]
    InvalidID3TagDataFound(String),

    #[error("Requested number of bytes {0} is greater than the available bytes: {1}")]
    InsufficientBytesToTake(usize, usize),

    #[error("Output File {0} Already Exists")]
    OutputFileAlreadyExists(String),

    #[error("Provided Mac HFS Timestamp is too small. Not a valid date.")]
    HFSTimestampTooSmall,

    #[error("Could not process the ID3 tag IDs")]
    ErrorParsingID3TagIDs,

    #[error("Could not write out metadata.")]
    CouldNotWrteOutData,

    #[error("Could not process the FLAC Vorbis tags")]
    ErrorParsingVorbisTags,
}

pub fn handle_local_error(local_error: LocalError, specific_error: String) {
    println!("\n{}: {}", local_error, specific_error);
    print_usage_message();
}
