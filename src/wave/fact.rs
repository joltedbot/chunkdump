use crate::fileio::{read_four_byte_integer_from_file, skip_over_bytes_in_file};
use byte_unit::rust_decimal::prelude::Zero;
use std::error::Error;
use std::fs::File;

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

    pub fn get_metadata_output(&self) -> Vec<String> {
        let mut fact_data: Vec<String> = vec![];

        if !self.samples_per_channel.is_zero() {
            fact_data.push("\n-------------\nFact Chunk Details:\n-------------".to_string());
            fact_data.push(format!("Samples per Channel: {}", self.samples_per_channel));
        }
        fact_data
    }
}
