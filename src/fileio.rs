use crate::byte_arrays::Endian;
use crate::chunks::{CHUNK_ID_FIELD_LENGTH_IN_BYTES, CHUNK_SIZE_FIELD_LENGTH_IN_BYTES};
use crate::errors::LocalError;
use crate::formating::{
    add_one_if_byte_size_is_odd, canonicalize_file_path, format_file_size_as_string,
    get_file_name_from_file_path,
};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek};
use upon::Value;

const AIFF_FILE_CHUNKID: &str = "FORM";
const FLAC_FILE_CHUNKID: &str = "fLaC";
const RIFF_FILE_CHUNKID: &str = "RIFF";
const MIDI_FILE_CHUNKID: &str = "MThd";
const WAVE_FILE_TYPE_ID: &str = "WAVE";
const RMID_FILE_TYPE_ID: &str = "RMID";

#[derive(Debug, PartialEq)]
pub enum FileType {
    Aiff,
    Flac,
    Wave,
    Smf,
    Rmid,
    Unsupported(String),
}

#[derive(Debug, PartialEq)]
enum RiffDataType {
    Wave,
    Rmid,
}

pub fn skip_over_bytes_in_file(
    file: &mut File,
    number_of_bytes: usize,
) -> Result<(), Box<dyn Error>> {
    file.seek_relative(number_of_bytes as i64)?;

    Ok(())
}

pub fn read_bytes_from_file(
    file: &mut File,
    number_of_bytes: usize,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut read_bytes: Vec<u8> = vec![0; number_of_bytes];
    file.read_exact(&mut read_bytes)?;

    Ok(read_bytes)
}

pub fn read_byte_from_file(file: &mut File) -> Result<u8, Box<dyn Error>> {
    let mut read_bytes = [0; 1];
    file.read_exact(&mut read_bytes)?;

    let result = read_bytes
        .first()
        .ok_or(LocalError::InsufficientBytesToTake(1, 0))?;
    Ok(*result)
}

pub fn get_file_id_from_file(input_file_path: &str) -> Result<FileType, Box<dyn Error>> {
    let mut input_file = File::open(input_file_path)?;
    let file_id_bytes = read_bytes_from_file(&mut input_file, CHUNK_ID_FIELD_LENGTH_IN_BYTES)?;
    let file_id = String::from_utf8(file_id_bytes)?;

    let file_type = match file_id.as_str() {
        AIFF_FILE_CHUNKID => FileType::Aiff,
        FLAC_FILE_CHUNKID => FileType::Flac,
        RIFF_FILE_CHUNKID => match get_riff_data_type_from_file(&mut input_file)? {
            RiffDataType::Wave => FileType::Wave,
            RiffDataType::Rmid => FileType::Rmid,
        },
        MIDI_FILE_CHUNKID => FileType::Smf,
        _ => FileType::Unsupported(file_id),
    };

    Ok(file_type)
}

fn get_riff_data_type_from_file(wave_file: &mut File) -> Result<RiffDataType, Box<dyn Error>> {
    skip_over_bytes_in_file(wave_file, CHUNK_SIZE_FIELD_LENGTH_IN_BYTES)?;

    let riff_id_bytes = read_chunk_id_from_file(wave_file)?;

    match riff_id_bytes.as_str() {
        WAVE_FILE_TYPE_ID => Ok(RiffDataType::Wave),
        RMID_FILE_TYPE_ID => Ok(RiffDataType::Rmid),
        _ => {
            eprintln!(
                "RIFF file type mismatch: {:?} ",
                riff_id_bytes.to_ascii_uppercase()
            );
            Err(Box::new(LocalError::InvalidRiffTypeID))
        }
    }
}

pub fn read_chunk_id_from_file(file: &mut File) -> Result<String, Box<dyn Error>> {
    let read_bytes = read_bytes_from_file(file, CHUNK_ID_FIELD_LENGTH_IN_BYTES)?;
    Ok(String::from_utf8(read_bytes)?)
}

pub fn read_chunk_size_from_file(
    file: &mut File,
    endianness: Endian,
) -> Result<usize, Box<dyn Error>> {
    let chunk_size_bytes = read_bytes_from_file(file, 4)?;
    let mut byte_array: [u8; CHUNK_SIZE_FIELD_LENGTH_IN_BYTES] = Default::default();
    byte_array.copy_from_slice(chunk_size_bytes.as_slice());

    let mut chunk_size = match endianness {
        Endian::Little => u32::from_le_bytes(byte_array),
        Endian::Big => u32::from_be_bytes(byte_array),
    };

    chunk_size = add_one_if_byte_size_is_odd(chunk_size);

    Ok(chunk_size as usize)
}

pub fn get_file_metadata(
    file_path: &str,
    file: &File,
    header_template: &str,
) -> Result<OutputEntry, Box<dyn Error>> {
    let size_in_bytes = file.metadata()?.len();
    let name = get_file_name_from_file_path(file_path)?;
    let canonical_path = canonicalize_file_path(file_path)?;

    let smf_output_values: Value = upon::value! {
        file_name: name,
        file_path: canonical_path,
        file_size: format_file_size_as_string(size_in_bytes),
    };

    let formated_smf_output: String = get_file_chunk_output(header_template, smf_output_values)?;

    let file_metadata = OutputEntry {
        section: Section::Header,
        text: formated_smf_output,
    };

    Ok(file_metadata)
}
