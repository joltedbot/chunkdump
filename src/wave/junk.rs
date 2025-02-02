use crate::fileio::{read_bytes_from_file_as_lossy_string, read_chunk_size_from_file};
use crate::template::Template;
use std::error::Error;
use std::fs::File;
use upon::Value;

#[derive(Debug, Clone, Default)]
pub struct JunkFields {
    junk_as_string: String,
}

impl JunkFields {
    pub fn new(wave_file: &mut File) -> Result<Self, Box<dyn Error>> {
        let chunk_size = read_chunk_size_from_file(wave_file)?;
        let junk_as_string = read_bytes_from_file_as_lossy_string(wave_file, chunk_size as usize)?;
        Ok(JunkFields { junk_as_string })
    }

    pub fn get_metadata_output(&self, template: &Template, template_name: &str) -> Result<String, Box<dyn Error>> {
        if self.junk_as_string.is_empty() {
            return Ok("".to_string());
        }

        let wave_output_values: Value = upon::value! {
            junk: self.junk_as_string.clone(),
        };

        Ok(template.get_wave_chunk_output(template_name, wave_output_values)?)
    }
}
