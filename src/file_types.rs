use crate::errors::LocalError;
use crate::output::OutputEntry;
use std::error::Error;

pub mod aiff;
pub mod flac;
mod m4a;
pub mod midi;
pub mod mp3;
pub mod ogg;
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
    Ogg,
    Mp3(Mp3SubType),
    M4a,
    Unsupported(String),
}

#[derive(Debug, PartialEq)]
pub enum Mp3SubType {
    ID3,
    NonId3,
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
        FileType::Ogg => ogg::get_metadata_from_file(input_file_path)?,
        FileType::Mp3(subtype) => {
            mp3::get_metadata_from_file(input_file_path, subtype, mandatory_sections_only)?
        }
        FileType::M4a => m4a::get_metadata_from_file(input_file_path, mandatory_sections_only)?,
        FileType::Unsupported(file_id) => {
            return Err(Box::new(LocalError::UnsupportedFileType(file_id)))
        }
    };
    Ok(metadata)
}
