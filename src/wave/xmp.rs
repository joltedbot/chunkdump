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

    pub fn get_metadata_output(&self) -> Vec<String> {
        let mut xmp_data: Vec<String> = vec![];

        if !self.xmp_xml.is_empty() {
            xmp_data.push(
                "\n-------------\nXMP (_PMX) Chunk XPacket (XML) Data:\n-------------".to_string(),
            );
            xmp_data.push(format!("{}", self.xmp_xml));
        }

        xmp_data
    }
}
