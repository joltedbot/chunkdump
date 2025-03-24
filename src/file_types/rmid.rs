use crate::byte_arrays::Endian;
use crate::chunks::{
    get_chunk_metadata, CHUNKS_NOT_TO_EXTRACT_DATA_FROM,
    ERROR_TO_MATCH_IF_NOT_ENOUGH_BYTES_LEFT_IN_FILE, MANDATORY_CHUNKS,
};
use crate::file_types::midi::get_metadata_from_midi_data;
use crate::fileio::{
    get_file_metadata, read_bytes_from_file, read_chunk_id_from_file, read_chunk_size_from_file,
    skip_over_bytes_in_file,
};
use crate::output::OutputEntry;
use std::error::Error;
use std::fs::File;

const TEMPLATE_HEADER_CONTENT: &str = include_str!("../templates/file_types/rmid.tmpl");

const RMID_HEADER_FIELDS_LENGTH_IN_BYTES: usize = 12;

const RMID_MIDI_DATA_CHUNK_ID: &str = "data";

pub fn get_metadata_from_file(
    rmid_file_path: &str,
    mandatory_sections_only: bool,
) -> Result<Vec<OutputEntry>, Box<dyn Error>> {
    let mut rmid_file = File::open(rmid_file_path)?;
    skip_over_bytes_in_file(&mut rmid_file, RMID_HEADER_FIELDS_LENGTH_IN_BYTES)?;

    let file_metadata = get_file_metadata(rmid_file_path, &rmid_file, TEMPLATE_HEADER_CONTENT)?;
    let chunk_metadata =
        get_metadata_from_rmid_chunks(&mut rmid_file, rmid_file_path, mandatory_sections_only)?;

    let mut output = vec![file_metadata];
    output.extend(chunk_metadata);

    Ok(output)
}

fn get_metadata_from_rmid_chunks(
    input_file: &mut File,
    file_path: &str,
    mandatory_sections_only: bool,
) -> Result<Vec<OutputEntry>, Box<dyn Error>> {
    let mut output: Vec<OutputEntry> = vec![];

    loop {
        let chunk_id = match read_chunk_id_from_file(input_file) {
            Ok(chunk_id) => chunk_id.to_lowercase(),
            Err(error) if error.to_string() == ERROR_TO_MATCH_IF_NOT_ENOUGH_BYTES_LEFT_IN_FILE => {
                break
            }
            Err(error) => return Err(error),
        };

        let chunk_size = read_chunk_size_from_file(input_file, Endian::Little)?;

        if chunk_id == RMID_MIDI_DATA_CHUNK_ID {
            let mut chunk_data = read_bytes_from_file(input_file, chunk_size).unwrap_or_default();
            output.extend(get_metadata_from_midi_data(
                &mut chunk_data,
                mandatory_sections_only,
            )?);
            continue;
        }

        if mandatory_sections_only && !MANDATORY_CHUNKS.contains(&chunk_id.as_str()) {
            skip_over_bytes_in_file(input_file, chunk_size)?;
            continue;
        }

        let chunk_data = if CHUNKS_NOT_TO_EXTRACT_DATA_FROM.contains(&chunk_id.as_str()) {
            skip_over_bytes_in_file(input_file, chunk_size)?;
            Vec::new()
        } else {
            read_bytes_from_file(input_file, chunk_size).unwrap_or_default()
        };

        output.push(get_chunk_metadata(chunk_id, chunk_data, file_path)?);
    }

    Ok(output)
}
