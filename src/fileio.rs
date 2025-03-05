use crate::bytes::Endian;
use crate::chunks::{CHUNK_ID_FIELD_LENGTH_IN_BYTES, CHUNK_SIZE_FIELD_LENGTH_IN_BYTES};
use crate::formating;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek};

const AIFF_FILE_CHUNKID: &str = "FORM";
const FLAC_FILE_CHUNKID: &str = "fLaC";
const WAVE_FILE_CHUNKID: &str = "RIFF";
const MIDI_FILE_CHUNKID: &str = "MThd";

#[derive(Debug, PartialEq)]
pub enum FileType {
    Aiff,
    Flac,
    Wave,
    Midi,
    Unsupported,
}

pub fn skip_over_bytes_in_file(file: &mut File, number_of_bytes: usize) -> Result<(), Box<dyn Error>> {
    file.seek_relative(number_of_bytes as i64)?;

    Ok(())
}

pub fn read_bytes_from_file(file: &mut File, number_of_bytes: usize) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut read_bytes: Vec<u8> = vec![0; number_of_bytes];
    file.read_exact(&mut read_bytes)?;

    Ok(read_bytes)
}

pub fn get_file_id_from_file(input_file_path: &str) -> Result<FileType, Box<dyn Error>> {
    let mut input_file = File::open(input_file_path)?;
    let file_id_bytes = read_bytes_from_file(&mut input_file, CHUNK_ID_FIELD_LENGTH_IN_BYTES)?;
    let file_id = String::from_utf8(file_id_bytes)?;

    let file_type = match file_id.as_str() {
        AIFF_FILE_CHUNKID => FileType::Aiff,
        FLAC_FILE_CHUNKID => FileType::Flac,
        WAVE_FILE_CHUNKID => FileType::Wave,
        MIDI_FILE_CHUNKID => FileType::Midi,
        _ => FileType::Unsupported,
    };

    Ok(file_type)
}

pub fn read_chunk_id_from_file(file: &mut File) -> Result<String, Box<dyn Error>> {
    let read_bytes = read_bytes_from_file(file, CHUNK_ID_FIELD_LENGTH_IN_BYTES)?;
    Ok(String::from_utf8(read_bytes)?)
}

pub fn read_chunk_size_from_file(file: &mut File, endianness: Endian) -> Result<usize, Box<dyn Error>> {
    let chunk_size_bytes = read_bytes_from_file(file, 4)?;
    let mut byte_array: [u8; CHUNK_SIZE_FIELD_LENGTH_IN_BYTES] = Default::default();
    byte_array.copy_from_slice(chunk_size_bytes.as_slice());

    let mut chunk_size = match endianness {
        Endian::Little => u32::from_le_bytes(byte_array),
        Endian::Big => u32::from_be_bytes(byte_array),
    };

    chunk_size = formating::add_one_if_byte_size_is_odd(chunk_size);

    Ok(chunk_size as usize)
}

pub fn read_chunk_size_from_midi_file(file: &mut File, endianness: Endian) -> Result<usize, Box<dyn Error>> {
    let chunk_size_bytes = read_bytes_from_file(file, 4)?;
    let mut byte_array: [u8; CHUNK_SIZE_FIELD_LENGTH_IN_BYTES] = Default::default();
    byte_array.copy_from_slice(chunk_size_bytes.as_slice());

    let chunk_size = match endianness {
        Endian::Little => u32::from_le_bytes(byte_array),
        Endian::Big => u32::from_be_bytes(byte_array),
    };

    Ok(chunk_size as usize)
}
