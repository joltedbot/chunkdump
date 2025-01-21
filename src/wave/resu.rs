use crate::bytes::{read_bytes_from_file, read_four_byte_integer_from_file};
use flate2::read::ZlibDecoder;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
#[derive(Debug, Clone, Default)]
pub struct ResUFields {
    json_data: String,
}

pub fn read_resu_chunk_fields(wave_file: &mut File) -> Result<ResUFields, Box<dyn Error>> {
    let mut chunk_size = read_four_byte_integer_from_file(wave_file)?;

    if !chunk_size.is_power_of_two() {
        chunk_size += 1;
    }

    let resu = read_bytes_from_file(wave_file, chunk_size as usize)?;

    let mut zlib = ZlibDecoder::new(resu.as_slice());
    let mut json_data = String::new();
    zlib.read_to_string(&mut json_data)?;

    Ok(ResUFields { json_data })
}
