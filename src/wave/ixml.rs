use crate::fileio::{read_bytes_from_file_as_lossy_string, read_chunk_size_from_file};
use crate::template::Template;
use std::error::Error;
use std::fs::File;
use upon::Value;

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

    pub fn get_metadata_outputs(&self, template: &Template, template_name: &str) -> Result<String, Box<dyn Error>> {
        let wave_output_values: Value = upon::value! {
            ixml_xml: self.ixml_xml.clone(),
        };

        Ok(template.get_wave_chunk_output(template_name, wave_output_values)?)
    }
}
