use crate::fileio::{read_bytes_from_file, read_chunk_size_from_file};
use crate::template::Template;
use flate2::read::ZlibDecoder;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use upon::Value;

#[derive(Debug, Clone, Default)]
pub struct ResuFields {
    pub resu_json: String,
}

impl ResuFields {
    pub fn new(wave_file: &mut File) -> Result<Self, Box<dyn Error>> {
        let mut chunk_size = read_chunk_size_from_file(wave_file)?;

        if !chunk_size.is_power_of_two() {
            chunk_size += 1;
        }

        let resu = read_bytes_from_file(wave_file, chunk_size as usize)?;

        let mut zlib = ZlibDecoder::new(resu.as_slice());
        let mut resu_json = String::new();
        zlib.read_to_string(&mut resu_json)?;

        Ok(Self { resu_json })
    }

    pub fn get_metadata_outputs(&self, template: &Template, template_name: &str) -> Result<String, Box<dyn Error>> {
        let wave_output_values: Value = upon::value! {
            resu_json: self.resu_json.clone(),
        };

        Ok(template.get_wave_chunk_output(template_name, wave_output_values)?)
    }
}
