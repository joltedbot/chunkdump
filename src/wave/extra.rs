use crate::fileio::{read_bytes_from_file_as_lossy_string, read_chunk_size_from_file};
use std::error::Error;
use std::fs::File;

pub fn read_extra_chunk_fields(
    wave_file: &mut File,
    chunk_id: String,
) -> Result<(String, String), Box<dyn Error>> {
    let chunk_size = read_chunk_size_from_file(wave_file)?;
    let chunk_data = read_bytes_from_file_as_lossy_string(wave_file, chunk_size as usize)?;

    Ok((chunk_id, chunk_data))
}

pub fn get_extra_chunks_output(extra_chunks: &Vec<(String, String)>) -> Vec<String> {
    let mut extra_data: Vec<String> = vec![];

    if !extra_chunks.is_empty() {
        extra_data.push("\n-------------\nExtra Chunk Details:\n-------------".to_string());
        for chunk in extra_chunks {
            extra_data.push(format!("{}:, {}", chunk.0.clone(), chunk.1.clone()));
        }
    }

    extra_data
}
