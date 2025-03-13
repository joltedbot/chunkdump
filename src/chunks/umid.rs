use crate::byte_arrays::take_first_number_of_bytes;
use crate::formating::format_bytes_as_string_of_bytes;
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/umid.tmpl");

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let chunk_size = chunk_data.len();

    let umid = take_first_number_of_bytes(&mut chunk_data, chunk_size)?;

    let wave_output_values: Value = upon::value! {
        umid: format_bytes_as_string_of_bytes(&umid),
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, wave_output_values)?;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}
