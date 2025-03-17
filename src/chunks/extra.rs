use crate::byte_arrays::take_first_number_of_bytes_as_string;
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/extra.tmpl");
const EMPTY_DATA_MESSAGE: &str = "[The chunk exists but is empty]";

pub fn get_metadata(
    chunk_id: String,
    mut chunk_data: Vec<u8>,
) -> Result<OutputEntry, Box<dyn Error>> {
    let chunk_size = chunk_data.len();
    let mut chunk_data = take_first_number_of_bytes_as_string(&mut chunk_data, chunk_size)?;

    if chunk_data.is_empty() {
        chunk_data = EMPTY_DATA_MESSAGE.to_string();
    }

    let wave_output_values: Value = upon::value! {
        chunk_id: chunk_id,
        chunk_data: chunk_data,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, wave_output_values)?;

    Ok(OutputEntry {
        section: Section::Unsupported,
        text: formated_output,
    })
}
