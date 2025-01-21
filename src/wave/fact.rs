use crate::fileio::{read_four_byte_integer_from_file, skip_over_bytes_in_file};
use std::error::Error;
use std::fs::File;

const FACT_CKSIZE_FIELD_LENGTH_IN_BYTES: i64 = 4;

pub fn read_fact_chunk(wave_file: &mut File) -> Result<u32, Box<dyn Error>> {
    skip_over_bytes_in_file(wave_file, FACT_CKSIZE_FIELD_LENGTH_IN_BYTES)?;
    let number_of_samples_per_channel = read_four_byte_integer_from_file(wave_file)?;
    Ok(number_of_samples_per_channel)
}
