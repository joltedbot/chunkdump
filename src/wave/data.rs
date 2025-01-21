use crate::fileio::{read_four_byte_integer_from_file, skip_over_bytes_in_file};
use std::error::Error;
use std::fs::File;

pub fn skip_data_chunk(wave_file: &mut File) -> Result<(), Box<dyn Error>> {
    let mut chunk_size = read_four_byte_integer_from_file(wave_file)?;

    if (chunk_size % 2) != 0 {
        chunk_size = chunk_size + 1;
    }

    skip_over_bytes_in_file(wave_file, chunk_size as i64)?;

    Ok(())
}
