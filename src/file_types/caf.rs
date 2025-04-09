use crate::caf_chunks::get_metadata_from_caf_chunks;
use crate::fileio::{get_file_metadata, read_bytes_from_file, skip_over_bytes_in_file};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use std::fs::File;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/file_types/caf.tmpl");
const HEADER_TEMPLATE_CONTENT: &str = include_str!("../templates/file_types/caf_header.tmpl");

const FILE_TYPE_LENGTH_IN_BYTES: usize = 4;
const FILE_VERSION_LENGTH_IN_BYTES: usize = 2;
const FILE_FLAG_LENGTH_IN_BYTES: usize = 2;

pub fn get_metadata_from_file(
    file_path: &str,
    mandatory_sections_only: bool,
) -> Result<Vec<OutputEntry>, Box<dyn Error>> {
    let mut caf_file = File::open(file_path)?;
    let file_metadata = get_file_metadata(file_path, &caf_file, TEMPLATE_CONTENT)?;
    let header_metadata = get_file_header_metadata(&mut caf_file)?;

    let chunk_metadata =
        get_metadata_from_caf_chunks(&mut caf_file, file_path, mandatory_sections_only)?;

    let mut output = vec![file_metadata, header_metadata];
    output.extend(chunk_metadata);

    Ok(output)
}

fn get_file_header_metadata(caf_file: &mut File) -> Result<OutputEntry, Box<dyn Error>> {
    skip_over_bytes_in_file(caf_file, FILE_TYPE_LENGTH_IN_BYTES)?;
    let version_bytes = read_bytes_from_file(caf_file, FILE_VERSION_LENGTH_IN_BYTES)?;
    skip_over_bytes_in_file(caf_file, FILE_FLAG_LENGTH_IN_BYTES)?;

    let mut version_array: [u8; FILE_VERSION_LENGTH_IN_BYTES] = Default::default();
    version_array.copy_from_slice(&version_bytes);
    let version = u16::from_be_bytes(version_array);

    let output_values: Value = upon::value! {
        version: version,
    };

    let formated_output = get_file_chunk_output(HEADER_TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Header,
        text: formated_output,
    })
}
