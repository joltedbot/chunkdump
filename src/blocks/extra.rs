use crate::byte_arrays::take_first_number_of_bytes_as_string;
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/blocks/padding.tmpl");

pub fn get_metadata(block_type: u32, mut block_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let data_length_in_bytes = block_data.len();

    let output_values: Value = upon::value! {
        block_type: block_type,
        value: take_first_number_of_bytes_as_string(&mut block_data, data_length_in_bytes)?,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, output_values)?;

    let result = OutputEntry {
        section: Section::Unsupported,
        text: formated_output.trim().to_string(),
    };

    Ok(result)
}
