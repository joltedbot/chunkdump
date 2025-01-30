use crate::fileio::{read_chunk_size_from_file, skip_over_bytes_in_file};
use std::error::Error;
use std::fs::File;

pub fn skip_data_chunk(wave_file: &mut File) -> Result<(), Box<dyn Error>> {
    let chunk_size = read_chunk_size_from_file(wave_file)?;
    skip_over_bytes_in_file(wave_file, chunk_size as i64)?;

    Ok(())
}
