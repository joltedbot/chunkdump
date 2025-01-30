use crate::fileio::{read_bytes_from_file_as_lossy_string, read_chunk_size_from_file};
use std::error::Error;
use std::fs::File;

#[derive(Debug, Clone, Default)]
pub struct IXMLFields {
    ixml_xml: String,
}

impl IXMLFields {
    pub fn new(wave_file: &mut File) -> Result<Self, Box<dyn Error>> {
        let chunk_size = read_chunk_size_from_file(wave_file)?;
        let ixml_xml = read_bytes_from_file_as_lossy_string(wave_file, chunk_size as usize)?;
        Ok(Self { ixml_xml })
    }

    pub fn get_metadata_output(&self) -> Vec<String> {
        let mut ixml_data: Vec<String> = vec![];

        if !self.ixml_xml.is_empty() {
            ixml_data.push("\n-------------\niXML Chunk XML Data:\n-------------".to_string());
            ixml_data.push(format!("{}", self.ixml_xml));
        }

        ixml_data
    }
}
