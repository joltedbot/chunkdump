mod acid;
mod bext;
mod cart;
mod chunk;
mod cue;
mod fact;
mod fmt;
mod list;
mod resu;
mod smpl;
mod sndm;
mod umid;
mod xmp;

use crate::errors::LocalError;
use crate::fileio::{
    canonicalize_file_path, get_file_name_from_file_path, read_bytes_from_file, read_bytes_from_file_as_string,
    read_chunk_size_from_file, skip_over_bytes_in_file, Endian,
};
use crate::formating::format_file_size_as_string;
use crate::template::Template;
use crate::wave::chunk::Chunk;

use std::error::Error;
use std::fs::{File, Metadata};
use upon::Value;

const TEMPLATE_NAME: &str = "wave";
const TEMPLATE_CONTENT: &str = include_str!("templates/wave/wave.tmpl");
const WAVEID_FIELD_LENGTH_IN_BYTES: usize = 4;
const CHUNKID_FIELD_LENGTH_IN_BYTES: usize = 4;
pub const CHUNK_SIZE_FIELD_LENGTH_IN_BYTES: usize = 4;
const CORRECT_WAVE_ID: &[u8; 4] = b"WAVE";
const NOT_ENOUGH_BYTES_LEFT_IN_FILE_ERROR_MESSAGE: &str = "failed to fill whole buffer";

#[derive(Default)]
struct Wave {
    name: String,
    original_file_path: String,
    canonical_path: String,
    size_in_bytes: u64,
    chunks: Chunk,
}

impl Wave {
    fn extract_file_metadata(&mut self, file_path: &str, metadata: Metadata) -> Result<(), Box<dyn Error>> {
        self.size_in_bytes = metadata.len();
        self.original_file_path = file_path.to_string();
        self.name = get_file_name_from_file_path(file_path)?;
        self.canonical_path = canonicalize_file_path(file_path)?;
        self.chunks = Chunk::new(self.canonical_path.clone());

        Ok(())
    }

    fn extract_metadata_from_wave_chunks(&mut self, wave_file: &mut File) -> Result<(), Box<dyn Error>> {
        loop {
            let chunk_id: String = match read_bytes_from_file_as_string(wave_file, CHUNKID_FIELD_LENGTH_IN_BYTES) {
                Ok(chunk_id) => chunk_id.to_lowercase(),
                Err(error) if error.to_string() == NOT_ENOUGH_BYTES_LEFT_IN_FILE_ERROR_MESSAGE => break,
                Err(error) => return Err(error),
            };

            let chunk_size = read_chunk_size_from_file(wave_file, Endian::Little)?;
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

    fn format_data_for_output(
        &mut self,
        template: &mut Template,
        wave_file_path: &str,
        mut wave_file: File,
    ) -> Result<String, Box<dyn Error>> {
        self.extract_file_metadata(wave_file_path, wave_file.metadata()?)?;
        self.extract_metadata_from_wave_chunks(&mut wave_file)?;

        let wave_output_values: Value = upon::value! {
            file_name: self.name.clone(),
            file_path: self.canonical_path.clone(),
            file_size: format_file_size_as_string(self.size_in_bytes),
            chunk_ids_found: self.chunks.found_chunk_ids.join("', '"),
            chunk_ids_skipped: self.chunks.skipped_chunk_ids.join("', '"),
        };

        let formated_wave_output: String =
            template.get_wave_chunk_output(TEMPLATE_NAME, TEMPLATE_CONTENT, wave_output_values)?;

        Ok(formated_wave_output)
    }
}

pub fn get_metadata_from_file(wave_file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut wave_file = File::open(wave_file_path)?;
    validate_riff_wave_header(&mut wave_file)?;

    let mut wave: Wave = Default::default();
    let mut template = Template::new();
    let mut output_lines: Vec<String> = vec![];
    output_lines.push(wave.format_data_for_output(&mut template, wave_file_path, wave_file)?);

    let formated_chunk_output = wave.chunks.format_data_for_output(&mut template)?;
    output_lines.extend(formated_chunk_output);

    Ok(output_lines)
}

fn validate_riff_wave_header(wave_file: &mut File) -> Result<(), Box<dyn Error>> {
    skip_over_bytes_in_file(wave_file, CHUNKID_FIELD_LENGTH_IN_BYTES)?;
    skip_over_bytes_in_file(wave_file, CHUNK_SIZE_FIELD_LENGTH_IN_BYTES)?;

    let wave_id_bytes = read_bytes_from_file(wave_file, WAVEID_FIELD_LENGTH_IN_BYTES)?;

    if wave_id_bytes.as_slice() != CORRECT_WAVE_ID {
        return Err(Box::new(LocalError::InvalidWaveID));
    }

    Ok(())
}
