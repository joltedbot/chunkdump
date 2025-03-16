use crate::cli::print_usage_message;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LocalError {
    #[error("Could not process the supplied file path: {0}")]
    InvalidPath(String),

    #[error("Could not extract the filename from the supplied path")]
    InvalidFileName,

    #[error("Unsupported file type: '{0}'.  Only Wave, AIFF, Flac, and RMID or SWF MIDI files are supported")]
    UnsupportedFileType(String),

    #[error("Incorrect RIFF file type, file is not a valid RIFF WAVE or RMID file")]
    InvalidRiffTypeID,

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
}

pub fn handle_local_error(local_error: LocalError, specific_error: String) {
    eprintln!("\n{}: {}", local_error, specific_error);
    print_usage_message();
}
