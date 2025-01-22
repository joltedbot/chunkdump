use crate::fileio::{read_bytes_from_file_as_lossy_string, read_four_byte_integer_from_file};
use std::error::Error;
use std::fs::File;

pub fn read_extra_chunk_fields(
    wave_file: &mut File,
    chunk_id: String,
) -> Result<(String, String), Box<dyn Error>> {
    let chunk_size = read_four_byte_integer_from_file(wave_file)?;
    let chunk_data = read_bytes_from_file_as_lossy_string(wave_file, chunk_size as usize)?;

    Ok((chunk_id, chunk_data))
}
