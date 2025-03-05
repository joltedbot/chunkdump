use crate::chunks::{
    get_chunk_metadata, Chunk, Section, CHUNK_ID_FIELD_LENGTH_IN_BYTES, CHUNK_SIZE_FIELD_LENGTH_IN_BYTES,
};
use crate::errors::LocalError;
use crate::fileio::{
    read_bytes_from_file, read_chunk_id_from_file, read_chunk_size_from_file, skip_over_bytes_in_file, Endian,
};
use crate::formating::{canonicalize_file_path, format_file_size_as_string, get_file_name_from_file_path};
use crate::template::get_file_chunk_output;
use std::error::Error;
use std::fs::File;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("templates/files/wave.tmpl");
const WAVEID_FIELD_LENGTH_IN_BYTES: usize = 4;
const CORRECT_WAVE_ID: &[u8; 4] = b"WAVE";
const ERROR_TO_MATCH_IF_NOT_ENOUGH_BYTES_LEFT_IN_FILE: &str = "failed to fill whole buffer";

fn get_metadata_from_wave(file_path: &str, wave_file: &mut File) -> Result<Chunk, Box<dyn Error>> {
    let size_in_bytes = wave_file.metadata()?.len();
    let name = get_file_name_from_file_path(file_path)?;
    let canonical_path = canonicalize_file_path(file_path)?;

    let wave_output_values: Value = upon::value! {
        file_name: name,
        file_path: canonical_path,
        file_size: format_file_size_as_string(size_in_bytes),
    };

    let wave_metadata: String = get_file_chunk_output(TEMPLATE_CONTENT, wave_output_values)?;

    let output = Chunk {
        section: Section::Header,
        text: wave_metadata,
    };

    Ok(output)
}

pub fn get_metadata_from_file(wave_file_path: &str) -> Result<Vec<Chunk>, Box<dyn Error>> {
    let mut wave_file = File::open(wave_file_path)?;
    validate_riff_wave_header(&mut wave_file)?;

    let wave_metatdata = get_metadata_from_wave(wave_file_path, &mut wave_file)?;
    let chunk_metadata = get_metadata_from_chunks(&mut wave_file, wave_file_path)?;

    let mut output = vec![wave_metatdata];
    output.extend(chunk_metadata);

    Ok(output)
}

fn get_metadata_from_chunks(wave_file: &mut File, file_path: &str) -> Result<Vec<Chunk>, Box<dyn Error>> {
    let mut output: Vec<Chunk> = vec![];

    loop {
        let chunk_id = match read_chunk_id_from_file(wave_file) {
            Ok(chunk_id) => chunk_id.to_lowercase(),
            Err(error) if error.to_string() == ERROR_TO_MATCH_IF_NOT_ENOUGH_BYTES_LEFT_IN_FILE => break,
            Err(error) => return Err(error),
        };

        let chunk_size = read_chunk_size_from_file(wave_file, Endian::Little)?;
        let chunk_data = read_bytes_from_file(wave_file, chunk_size).unwrap_or_default();
        output.push(get_chunk_metadata(chunk_id, chunk_data, file_path)?);
    }

    Ok(output)
}

fn validate_riff_wave_header(wave_file: &mut File) -> Result<(), Box<dyn Error>> {
    skip_over_bytes_in_file(wave_file, CHUNK_ID_FIELD_LENGTH_IN_BYTES)?;
    skip_over_bytes_in_file(wave_file, CHUNK_SIZE_FIELD_LENGTH_IN_BYTES)?;

    let wave_id_bytes = read_bytes_from_file(wave_file, WAVEID_FIELD_LENGTH_IN_BYTES)?;

    if wave_id_bytes.as_slice() != CORRECT_WAVE_ID {
        return Err(Box::new(LocalError::InvalidWaveID));
    }

    Ok(())
}
