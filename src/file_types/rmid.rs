use crate::byte_arrays::Endian;
use crate::chunks::{get_chunk_metadata, ERROR_TO_MATCH_IF_NOT_ENOUGH_BYTES_LEFT_IN_FILE};
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

const RMID_CHUNK_ID_FOR_CHUNK_CONTAINING_MIDI_DATA: &str = "data";

pub fn get_metadata_from_file(rmid_file_path: &str) -> Result<Vec<OutputEntry>, Box<dyn Error>> {
    let mut rmid_file = File::open(rmid_file_path)?;
    skip_over_bytes_in_file(&mut rmid_file, RMID_HEADER_FIELDS_LENGTH_IN_BYTES)?;

    let file_metadata = get_file_metadata(rmid_file_path, &rmid_file, TEMPLATE_HEADER_CONTENT)?;
    let chunk_metadata = get_metadata_from_rmid_chunks(&mut rmid_file, rmid_file_path)?;

    let mut output = vec![file_metadata];
    output.extend(chunk_metadata);

    Ok(output)
}

fn get_metadata_from_rmid_chunks(
    rmid_file: &mut File,
    file_path: &str,
) -> Result<Vec<OutputEntry>, Box<dyn Error>> {
    let mut output: Vec<OutputEntry> = vec![];

    loop {
        let chunk_id = match read_chunk_id_from_file(rmid_file) {
            Ok(chunk_id) => chunk_id.to_lowercase(),
            Err(error) if error.to_string() == ERROR_TO_MATCH_IF_NOT_ENOUGH_BYTES_LEFT_IN_FILE => {
                break
            }
            Err(error) => return Err(error),
        };

        let chunk_size = read_chunk_size_from_file(rmid_file, Endian::Little)?;
        let mut chunk_data = read_bytes_from_file(rmid_file, chunk_size).unwrap_or_default();
        if chunk_id == RMID_CHUNK_ID_FOR_CHUNK_CONTAINING_MIDI_DATA {
            output.extend(get_metadata_from_midi_data(&mut chunk_data)?);
        } else {
            output.push(get_chunk_metadata(chunk_id, chunk_data, file_path)?);
        }
    }

    Ok(output)
}
