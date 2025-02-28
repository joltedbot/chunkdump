use crate::chunks::{get_chunk_metadata, Chunk, Section, ID3_CHUNK_ID};
use crate::fileio::{
    canonicalize_file_path, get_file_name_from_file_path, read_bytes_from_file, read_bytes_from_file_as_string,
    read_chunk_size_from_file, skip_over_bytes_in_file, Endian,
};
use crate::formating::format_file_size_as_string;
use crate::template::get_file_chunk_output;
use std::error::Error;
use std::fs::File;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("templates/files/aiff.tmpl");
const CHUNK_ID_LENGTH_IN_BYTES: usize = 4;
const AIFF_CHUNK_SIZE_LENGTH_IN_BYTES: usize = 4;
const AIFF_FORM_TYPE_LENGTH_IN_BYTES: usize = 4;
const ERROR_TO_MATCH_IF_NOT_ENOUGH_BYTES_LEFT_IN_FILE: &str = "failed to fill whole buffer";

pub fn get_metadata_from_file(file_path: &str) -> Result<Vec<Chunk>, Box<dyn Error>> {
    let mut aiff_file = File::open(file_path)?;

    let aiff_metatdata = get_metadata_from_aiff(file_path, &mut aiff_file)?;
    let chunk_metadata = get_metadata_from_chunks(&mut aiff_file, file_path)?;

    let mut output = vec![aiff_metatdata];
    output.extend(chunk_metadata);

    Ok(output)
}

fn get_metadata_from_aiff(file_path: &str, aiff_file: &mut File) -> Result<Chunk, Box<dyn Error>> {
    skip_over_bytes_in_file(aiff_file, CHUNK_ID_LENGTH_IN_BYTES + AIFF_CHUNK_SIZE_LENGTH_IN_BYTES)?;
    let form_type = read_bytes_from_file_as_string(aiff_file, AIFF_FORM_TYPE_LENGTH_IN_BYTES)?;

    let metadata = aiff_file.metadata()?;
    let size_in_bytes = metadata.len();
    let name = get_file_name_from_file_path(file_path)?;
    let canonical_path = canonicalize_file_path(file_path)?;

    let aiff_output_values: Value = upon::value! {
        file_name: &name,
        file_path: &canonical_path,
        file_size: format_file_size_as_string(size_in_bytes),
        form_type: &form_type,
    };

    let formated_aiff_output: String = get_file_chunk_output(TEMPLATE_CONTENT, aiff_output_values)?;

    let output = Chunk {
        section: Section::Header,
        text: formated_aiff_output,
    };

    Ok(output)
}

fn get_metadata_from_chunks(aiff_file: &mut File, file_path: &str) -> Result<Vec<Chunk>, Box<dyn Error>> {
    let mut output: Vec<Chunk> = vec![];

    loop {
        let chunk_id: String = match read_bytes_from_file_as_string(aiff_file, CHUNK_ID_LENGTH_IN_BYTES) {
            Ok(chunk_id) => chunk_id.to_lowercase(),
            Err(error) if error.to_string() == ERROR_TO_MATCH_IF_NOT_ENOUGH_BYTES_LEFT_IN_FILE => break,
            Err(error) => return Err(error),
        };

        let chunk_size = match chunk_id.as_str() {
            ID3_CHUNK_ID => read_chunk_size_from_file(aiff_file, Endian::Little)?,
            _ => read_chunk_size_from_file(aiff_file, Endian::Big)?,
        };

        let chunk_data = read_bytes_from_file(aiff_file, chunk_size).unwrap_or_default();
        output.push(get_chunk_metadata(chunk_id, chunk_data, file_path)?);
    }

    Ok(output)
}
