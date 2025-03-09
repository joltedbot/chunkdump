use crate::bytes::Endian;
use crate::chunks::{get_chunk_metadata, Chunk, ERROR_TO_MATCH_IF_NOT_ENOUGH_BYTES_LEFT_IN_FILE};
use crate::fileio::{
    get_file_metadata, read_bytes_from_file, read_chunk_id_from_file, read_chunk_size_from_file,
    skip_over_bytes_in_file,
};
use std::error::Error;
use std::fs::File;

const TEMPLATE_CONTENT: &str = include_str!("templates/files/wave.tmpl");
const WAVE_HEADER_FIELDS_LENGTH_IN_BYTES: usize = 12;

pub fn get_metadata_from_file(wave_file_path: &str) -> Result<Vec<Chunk>, Box<dyn Error>> {
    let mut wave_file = File::open(wave_file_path)?;
    skip_over_bytes_in_file(&mut wave_file, WAVE_HEADER_FIELDS_LENGTH_IN_BYTES)?;

    let file_metadata = get_file_metadata(wave_file_path, &wave_file, TEMPLATE_CONTENT)?;
    let chunk_metadata = get_metadata_from_chunks(&mut wave_file, wave_file_path)?;

    let mut chunks = vec![file_metadata];
    chunks.extend(chunk_metadata);

    Ok(chunks)
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
