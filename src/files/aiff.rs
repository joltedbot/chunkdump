use crate::byte_arrays::Endian;
use crate::chunks::get_metadata_from_chunks;
use crate::fileio::{get_file_metadata, read_bytes_from_file, skip_over_bytes_in_file};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use std::fs::File;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/files/aiff.tmpl");
const FORM_TEMPLATE_CONTENT: &str = include_str!("../templates/files/aiff-form.tmpl");
const CHUNK_ID_LENGTH_IN_BYTES: usize = 4;
const AIFF_CHUNK_SIZE_LENGTH_IN_BYTES: usize = 4;
const AIFF_FORM_TYPE_LENGTH_IN_BYTES: usize = 4;

pub fn get_metadata_from_file(file_path: &str) -> Result<Vec<OutputEntry>, Box<dyn Error>> {
    let mut aiff_file = File::open(file_path)?;

    let file_metadata = get_file_metadata(file_path, &aiff_file, TEMPLATE_CONTENT)?;
    let form_metadata = get_form_metadata_from_file(&mut aiff_file)?;
    let chunk_metadata = get_metadata_from_chunks(&mut aiff_file, file_path, Endian::Big)?;

    let mut output = vec![file_metadata, form_metadata];
    output.extend(chunk_metadata);

    Ok(output)
}

fn get_form_metadata_from_file(aiff_file: &mut File) -> Result<OutputEntry, Box<dyn Error>> {
    skip_over_bytes_in_file(aiff_file, CHUNK_ID_LENGTH_IN_BYTES + AIFF_CHUNK_SIZE_LENGTH_IN_BYTES)?;
    let form_type_bytes = read_bytes_from_file(aiff_file, AIFF_FORM_TYPE_LENGTH_IN_BYTES)?;

    let aiff_output_values: Value = upon::value! {
        form_type: String::from_utf8(form_type_bytes)?,
    };

    let formated_aiff_output: String = get_file_chunk_output(FORM_TEMPLATE_CONTENT, aiff_output_values)?;

    let form_chunk = OutputEntry {
        section: Section::Header,
        text: formated_aiff_output,
    };

    Ok(form_chunk)
}
