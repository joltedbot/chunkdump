use crate::chunk::Chunk;
use crate::errors::LocalError;
use crate::fileio::{
    canonicalize_file_path, get_file_name_from_file_path, read_bytes_from_file, read_bytes_from_file_as_string,
    read_chunk_size_from_file, skip_over_bytes_in_file,
};
use crate::output::write_out_file_data;
use crate::template::Template;

use byte_unit::{Byte, UnitType};
use std::error::Error;
use std::fs::File;
use upon::Value;

const TEMPLATE_NAME: &str = "riff";
const TEMPLATE_PATH: &str = include_str!("templates/wave/riff.tmpl");
const WAVEID_FIELD_LENGTH_IN_BYTES: usize = 4;
const CHUNKID_FIELD_LENGTH_IN_BYTES: usize = 4;
const RIFF_CHUNK_SIZE_FIELD_LENGTH_IN_BYTES: usize = 4;
const CORRECT_WAVE_ID: &str = "WAVE";
const NOT_ENOUGH_BYTES_LEFT_IN_FILE_ERROR_MESSAGE: &str = "failed to fill whole buffer";

#[derive(Default)]
pub struct Wave {
    pub template_name: &'static str,
    pub template_path: &'static str,
    pub name: String,
    pub original_file_path: String,
    pub canonical_path: String,
    pub output_file_path: String,
    pub size_in_bytes: u64,
    chunks: Chunk,
}

impl Wave {
    pub fn new(wave_file_path: String, output_file_path: String, wave_file: &mut File) -> Result<Self, Box<dyn Error>> {
        skip_over_bytes_in_file(wave_file, RIFF_CHUNK_SIZE_FIELD_LENGTH_IN_BYTES)?;
        let wave_id = read_bytes_from_file_as_string(wave_file, WAVEID_FIELD_LENGTH_IN_BYTES)?;

        if wave_id != CORRECT_WAVE_ID {
            return Err(Box::new(LocalError::InvalidWaveID));
        }

        let mut new_wave = Self {
            size_in_bytes: wave_file.metadata()?.len(),
            ..Default::default()
        };

        new_wave.extract_file_metadata(wave_file_path, output_file_path)?;
        new_wave.extract_metadata_from_wave_chunks(wave_file)?;

        Ok(new_wave)
    }

    fn extract_file_metadata(&mut self, file_path: String, output_file_path: String) -> Result<(), Box<dyn Error>> {
        self.original_file_path = file_path.clone();
        self.name = get_file_name_from_file_path(&file_path)?;
        self.canonical_path = canonicalize_file_path(&file_path)?;
        self.chunks = Chunk::new(self.canonical_path.clone());
        self.template_name = TEMPLATE_NAME;
        self.template_path = TEMPLATE_PATH;
        self.output_file_path = output_file_path;

        Ok(())
    }

    fn extract_metadata_from_wave_chunks(&mut self, wave_file: &mut File) -> Result<(), Box<dyn Error>> {
        loop {
            let chunk_id: String = match read_bytes_from_file_as_string(wave_file, CHUNKID_FIELD_LENGTH_IN_BYTES) {
                Ok(chunk_id) => chunk_id.to_lowercase(),
                Err(error) => {
                    if error.to_string() == NOT_ENOUGH_BYTES_LEFT_IN_FILE_ERROR_MESSAGE {
                        break;
                    } else {
                        return Err(error);
                    }
                }
            };

            let chunk_size = read_chunk_size_from_file(wave_file)?;
            let mut chunk_data: Vec<u8> = vec![];

            if self.chunks.ignore_data_for_chunks.contains(&chunk_id.as_str()) {
                skip_over_bytes_in_file(wave_file, chunk_size)?;
            } else {
                chunk_data = read_bytes_from_file(wave_file, chunk_size)?;
            }

            self.chunks.add_chunk(chunk_id, chunk_data)?;
        }

        Ok(())
    }

    pub fn output_metadata(&self, mut template: Template) -> Result<(), Box<dyn Error>> {
        let mut output_lines: Vec<String> = vec![];

        output_lines.push(self.format_data_for_output(&mut template)?);
        let wave_chunk_output_lines = self.chunks.format_data_for_output(&mut template)?;
        output_lines.extend(wave_chunk_output_lines);

        write_out_file_data(output_lines, self.output_file_path.clone())?;

        Ok(())
    }

    fn format_data_for_output(&self, template: &mut Template) -> Result<String, Box<dyn Error>> {
        let wave_output_values: Value = upon::value! {
            file_name: self.name.clone(),
            file_path: self.original_file_path.clone(),
            file_size: format_file_size_as_string(self.size_in_bytes),
            chunk_ids_found: self.chunks.found_chunk_ids.join(", "),
        };

        let formated_output = template.get_wave_chunk_output(self.template_name, self.template_path, wave_output_values)?;
        Ok(formated_output)
    }
}

pub fn add_one_if_byte_size_is_odd(mut byte_size: u32) -> u32 {
    if byte_size % 2 > 0 {
        byte_size += 1;
    }

    byte_size
}

fn format_file_size_as_string(file_size_in_bytes: u64) -> String {
    format!(
        "{:#.2}",
        Byte::from_u64(file_size_in_bytes).get_appropriate_unit(UnitType::Binary)
    )
}
