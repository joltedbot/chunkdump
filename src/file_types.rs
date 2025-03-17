use crate::errors::LocalError;
use crate::fileio::FileType;
use crate::output::OutputEntry;
use std::error::Error;

pub mod aiff;
pub mod flac;
mod midi;
pub mod rmid;
pub mod smf;
pub mod wave;

pub fn get_file_metadata(
    input_file_path: &str,
    file_type: FileType,
) -> Result<Vec<OutputEntry>, Box<dyn Error>> {
    let metadata: Vec<OutputEntry> = match file_type {
        FileType::Wave => wave::get_metadata_from_file(input_file_path)?,
        FileType::Flac => flac::get_metadata_from_file(input_file_path)?,
        FileType::Aiff => aiff::get_metadata_from_file(input_file_path)?,
        FileType::Smf => smf::get_metadata_from_file(input_file_path)?,
        FileType::Rmid => rmid::get_metadata_from_file(input_file_path)?,
        FileType::Unsupported(file_id) => {
            return Err(Box::new(LocalError::UnsupportedFileType(file_id)))
        }
    };
    Ok(metadata)
}
