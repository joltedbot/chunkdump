use crate::byte_arrays::Endian;
use crate::chunks::get_metadata_from_chunks;
use crate::fileio::{get_file_metadata, skip_over_bytes_in_file};
use crate::output::OutputEntry;
use std::error::Error;
use std::fs::File;

const TEMPLATE_CONTENT: &str = include_str!("../templates/file_types/wave.tmpl");
const WAVE_HEADER_FIELDS_LENGTH_IN_BYTES: usize = 12;

pub fn get_metadata_from_file(
    wave_file_path: &str,
    mandatory_sections_only: bool,
) -> Result<Vec<OutputEntry>, Box<dyn Error>> {
    let mut wave_file = File::open(wave_file_path)?;
    skip_over_bytes_in_file(&mut wave_file, WAVE_HEADER_FIELDS_LENGTH_IN_BYTES)?;

    let file_metadata = get_file_metadata(wave_file_path, &wave_file, TEMPLATE_CONTENT)?;
    let chunk_metadata = get_metadata_from_chunks(
        &mut wave_file,
        wave_file_path,
        mandatory_sections_only,
        Endian::Little,
    )?;

    let mut chunks = vec![file_metadata];
    chunks.extend(chunk_metadata);

    Ok(chunks)
}
