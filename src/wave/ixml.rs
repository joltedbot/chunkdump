use crate::fileio::{read_bytes_from_file_as_lossy_string, read_four_byte_integer_from_file};
use std::error::Error;
use std::fs::File;

pub fn read_ixml_chunk(wave_file: &mut File) -> Result<String, Box<dyn Error>> {
    let chunk_size = read_four_byte_integer_from_file(wave_file)?;
    let ixml_data = read_bytes_from_file_as_lossy_string(wave_file, chunk_size as usize)?;
    Ok(ixml_data)
}
