use crate::fileio::{read_bytes_from_file_as_lossy_string, read_chunk_size_from_file};
use crate::template::Template;
use std::error::Error;
use std::fs::File;
use upon::Value;

#[derive(Debug, Clone, Default)]
pub struct ExtraFields {
    chunk_id: String,
    chunk_data: String,
}

impl ExtraFields {
    pub fn new(wave_file: &mut File, chunk_id: String) -> Result<Self, Box<dyn Error>> {
        let chunk_size = read_chunk_size_from_file(wave_file)?;
        let chunk_data = read_bytes_from_file_as_lossy_string(wave_file, chunk_size as usize)?;

        Ok(Self { chunk_id, chunk_data })
    }

    pub fn get_metadata_outputs(&self, template: &Template, template_name: &str) -> Result<String, Box<dyn Error>> {
        let wave_output_values: Value = upon::value! {
            chunk_id: self.chunk_id.clone(),
            chunk_data: self.chunk_data.clone(),
        };

        Ok(template.get_wave_chunk_output(template_name, wave_output_values)?)
    }
}

pub fn get_extra_chunk_header_output(template: &Template, template_name: &str) -> Result<String, Box<dyn Error>> {
    Ok(template.get_wave_chunk_output(template_name, upon::value! {})?)
}
