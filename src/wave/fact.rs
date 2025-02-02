use crate::fileio::{read_four_byte_integer_from_file, skip_over_bytes_in_file};
use crate::template::Template;
use std::error::Error;
use std::fs::File;
use upon::Value;

const FACT_CKSIZE_FIELD_LENGTH_IN_BYTES: i64 = 4;

#[derive(Debug, Clone, Default)]
pub struct FactFields {
    pub samples_per_channel: u32,
}

impl FactFields {
    pub fn new(wave_file: &mut File) -> Result<Self, Box<dyn Error>> {
        skip_over_bytes_in_file(wave_file, FACT_CKSIZE_FIELD_LENGTH_IN_BYTES)?;

        Ok(Self {
            samples_per_channel: read_four_byte_integer_from_file(wave_file)?,
        })
    }

    pub fn get_metadata_output(&self, template: &Template, template_name: &str) -> Result<String, Box<dyn Error>> {
        let wave_output_values: Value = upon::value! {
            samples_per_channel: self.samples_per_channel,
        };

        Ok(template.get_wave_chunk_output(template_name, wave_output_values)?)
    }
}
