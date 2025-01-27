use crate::fileio::{read_bytes_from_file_as_lossy_string, read_four_byte_integer_from_file};
use std::error::Error;
use std::fs::File;

#[derive(Debug, Clone, Default)]
pub struct XMPFields {
    pub xmp_xml: String,
}

impl XMPFields {
    pub fn new(wave_file: &mut File) -> Result<Self, Box<dyn Error>> {
        let chunk_size = read_four_byte_integer_from_file(wave_file)?;
        let xmp_xml = read_bytes_from_file_as_lossy_string(wave_file, chunk_size as usize)?;
        Ok(Self { xmp_xml })
    }
}
