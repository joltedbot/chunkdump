use crate::bytes::{read_four_byte_integer_from_file, skip_over_bytes_in_file};
use std::error::Error;
use std::fs::File;

const FACT_CKSIZE_FIELD_LENGTH_IN_BYTES: i64 = 4;

pub fn read_data_chunk_fields(wave_file: &mut File) -> Result<(), Box<dyn Error>> {
    let mut chunk_size = read_four_byte_integer_from_file(wave_file)?;

    if (chunk_size % 2) != 0 {
        chunk_size = chunk_size + 1;
    }

    skip_over_bytes_in_file(wave_file, chunk_size as i64)?;

    Ok(())
}
