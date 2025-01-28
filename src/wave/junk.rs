use crate::fileio::{read_bytes_from_file_as_lossy_string, read_four_byte_integer_from_file};
use std::error::Error;
use std::fs::File;

#[derive(Debug, Clone, Default)]
pub struct JunkFields {
    junk_as_string: String,
}

impl JunkFields {
    pub fn new(wave_file: &mut File) -> Result<Self, Box<dyn Error>> {
        let chunk_size = read_four_byte_integer_from_file(wave_file)?;
        let junk_as_string = read_bytes_from_file_as_lossy_string(wave_file, chunk_size as usize)?;
        Ok(JunkFields { junk_as_string })
    }

    pub fn get_metadata_output(&self) -> Vec<String> {
        let mut junk_data: Vec<String> = vec![];

        if !self.junk_as_string.is_empty() {
            junk_data.push("\n-------------\nJunk Chunk Details:\n-------------".to_string());
            junk_data.push(format!("Junk Data as String: {}", self.junk_as_string));
        }

        junk_data
    }
}
