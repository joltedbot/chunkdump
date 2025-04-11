use crate::cli::print_usage_message;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LocalError {
    #[error("Could not process the supplied file path: {0}")]
    InvalidPath(String),

    #[error("Could not extract the filename from the supplied path")]
    InvalidFileName,

    #[error("Unsupported file type: '{0}'.  Only Wave, AIFF, Flac, Ogg Vorbis, MP3, M4a, Caf and RMID or SWF MIDI files are supported")]
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

    #[error("Can not take {0} bytes from array of size {1}.")]
    InsufficientBytesToTake(usize, usize),

    #[error("Can not read {0} bytes from the file. File has fewer bytes left that requested.")]
    InsufficientBytesToRead(usize),

    #[error("Output File {0} Already Exists")]
    OutputFileAlreadyExists(String),

    #[error("Provided Mac HFS Timestamp is too small. Not a valid date.")]
    HFSTimestampTooSmall,

    #[error("Could not process the ID3 tag IDs")]
    ErrorParsingID3TagIDs,

    #[error("Could not write out metadata.")]
    CouldNotWrteOutData,

    #[error("Invalid Vorbis User Comment. No = character found.")]
    InvalidVorbisComment,

    #[error("Value is not a valid sync safe integer. At least one byte exceeds the max sync safe byte size {0}.")]
    MP3SyncSafeIntegerOverflow(u8),

    #[error("{0} is not a valid MP3 header sample rate index value.")]
    MP3SampleRateIndexOverflow(u8),

    #[error("{0} is not a valid MP3 header version index value.")]
    MP3VersionIndexOverflow(u8),

    #[error("{0} is not a valid MP3 header layer index value.")]
    MP3LayerIndexOverflow(u8),

    #[error("{0} is not a valid MP3 header bitrate index value.")]
    MP3BitrateIndexOverflow(u8),

    #[error("[{0}] is not a valid chunk ID and likely indicates an invalid metadata format in this file. Processing can not continue.")]
    InvalidChunkIDCanNotContinue(String),
}

pub fn handle_local_error(local_error: LocalError, specific_error: String) {
    eprintln!("\n{}: {}", local_error, specific_error);
    print_usage_message();
}
