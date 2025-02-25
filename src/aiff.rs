mod chunk;
mod comm;
mod comt;
mod fver;
mod mark;

use crate::aiff::chunk::{Chunk, ID3_CHUNK_ID};
use crate::fileio::{
    canonicalize_file_path, get_file_name_from_file_path, read_bytes_from_file, read_bytes_from_file_as_string,
    read_chunk_size_from_file, skip_over_bytes_in_file, write_out_file_data, Endian,
};
use crate::formating::format_file_size_as_string;
use crate::template::Template;
use std::error::Error;
use std::fs::File;
use upon::Value;

const TEMPLATE_NAME: &str = "aiff";
const TEMPLATE_CONTENT: &str = include_str!("templates/aiff/aiff.tmpl");
const CHUNK_ID_LENGTH_IN_BYTES: usize = 4;
const AIFF_CHUNK_SIZE_LENGTH_IN_BYTES: usize = 4;
const AIFF_FORM_TYPE_LENGTH_IN_BYTES: usize = 4;
const NOT_ENOUGH_BYTES_LEFT_IN_FILE_ERROR_MESSAGE: &str = "failed to fill whole buffer";

#[derive(Default)]
struct Aiff {
    name: String,
    original_file_path: String,
    canonical_path: String,
    size_in_bytes: u64,
    form_type: String,
    chunks: Chunk,
}

impl Aiff {
    fn format_data_for_output(&mut self, template: &mut Template, file_path: &str) -> Result<String, Box<dyn Error>> {
        let mut aiff_file = File::open(file_path)?;
        self.extract_file_metadata(file_path, &mut aiff_file)?;
        self.extract_metadata_from_aiff_chunks(&mut aiff_file)?;

        let wave_output_values: Value = upon::value! {
            file_name: &self.name,
            file_path: &self.canonical_path,
            file_size: format_file_size_as_string(self.size_in_bytes),
            form_type: &self.form_type,
            chunk_ids_found: self.chunks.found_chunk_ids.join("', '"),
            chunk_ids_skipped: self.chunks.skipped_chunk_ids.join("', '"),
        };

        let formated_wave_output: String =
            template.get_wave_chunk_output(TEMPLATE_NAME, TEMPLATE_CONTENT, wave_output_values)?;
        Ok(formated_wave_output)
    }

    fn extract_file_metadata(&mut self, file_path: &str, aiff_file: &mut File) -> Result<(), Box<dyn Error>> {
        skip_over_bytes_in_file(aiff_file, CHUNK_ID_LENGTH_IN_BYTES + AIFF_CHUNK_SIZE_LENGTH_IN_BYTES)?;

        let form_type = read_bytes_from_file_as_string(aiff_file, AIFF_FORM_TYPE_LENGTH_IN_BYTES)?;

        let metadata = aiff_file.metadata()?;
        self.size_in_bytes = metadata.len();
        self.original_file_path = file_path.to_string();
        self.name = get_file_name_from_file_path(file_path)?;
        self.canonical_path = canonicalize_file_path(file_path)?;
        self.form_type = form_type;
        self.chunks = Chunk::new(file_path.to_string());

        Ok(())
    }

    fn extract_metadata_from_aiff_chunks(&mut self, aiff_file: &mut File) -> Result<(), Box<dyn Error>> {
        loop {
            let chunk_id: String = match read_bytes_from_file_as_string(aiff_file, CHUNK_ID_LENGTH_IN_BYTES) {
                Ok(chunk_id) => chunk_id.to_lowercase(),
                Err(error) if error.to_string() == NOT_ENOUGH_BYTES_LEFT_IN_FILE_ERROR_MESSAGE => break,
                Err(error) => return Err(error),
            };

            let chunk_size = match chunk_id.as_str() {
                ID3_CHUNK_ID => read_chunk_size_from_file(aiff_file, Endian::Little)?,
                _ => read_chunk_size_from_file(aiff_file, Endian::Big)?,
            };

            let mut chunk_data: Vec<u8> = vec![];

            if self.chunks.ignore_data_for_chunks.contains(&chunk_id.as_str()) {
                skip_over_bytes_in_file(aiff_file, chunk_size)?;
            } else {
                chunk_data = read_bytes_from_file(aiff_file, chunk_size)?;
            }

            self.chunks.add_chunk(&chunk_id, chunk_data)?;
        }

        Ok(())
    }
}

pub fn extract_and_output_aiff_metadata(
    aiff_file_path: &str,
    output_file_path: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let mut template = Template::new();
    let mut aiff: Aiff = Default::default();

    let mut output_lines: Vec<String> = vec![aiff.format_data_for_output(&mut template, aiff_file_path)?];
    let formated_chunk_output = aiff.chunks.format_data_for_output(&mut template)?;
    output_lines.extend(formated_chunk_output);

    write_out_file_data(output_lines, output_file_path)?;

    Ok(())
}
