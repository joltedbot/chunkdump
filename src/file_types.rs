use crate::errors::LocalError;
use crate::output::OutputEntry;
use std::error::Error;

pub mod aiff;
pub mod flac;
pub mod midi;
pub mod rmid;
pub mod smf;
pub mod wave;

#[derive(Debug, PartialEq)]
pub enum FileType {
    Aiff,
    Flac,
    Wave,
    Smf,
    Rmid,
    Unsupported(String),
}

pub fn get_file_metadata(
    input_file_path: &str,
    file_type: FileType,
    mandatory_sections_only: bool,
) -> Result<Vec<OutputEntry>, Box<dyn Error>> {
    let metadata: Vec<OutputEntry> = match file_type {
        FileType::Wave => wave::get_metadata_from_file(input_file_path, mandatory_sections_only)?,
        FileType::Flac => flac::get_metadata_from_file(input_file_path, mandatory_sections_only)?,
        FileType::Aiff => aiff::get_metadata_from_file(input_file_path, mandatory_sections_only)?,
        FileType::Smf => smf::get_metadata_from_file(input_file_path, mandatory_sections_only)?,
        FileType::Rmid => rmid::get_metadata_from_file(input_file_path, mandatory_sections_only)?,
        FileType::Unsupported(file_id) => {
            return Err(Box::new(LocalError::UnsupportedFileType(file_id)))
        }
    };
    Ok(metadata)
}
