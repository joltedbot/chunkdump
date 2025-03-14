use crate::byte_arrays::take_first_number_of_bytes_as_string;
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use upon::Value;

const EMPTY_TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/empty.tmpl");
const TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/text.tmpl");

pub fn get_metadata(title: &str, mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let chunk_size = chunk_data.len();
    let raw_body = take_first_number_of_bytes_as_string(&mut chunk_data, chunk_size)?;
    let body = raw_body.trim().to_string();

    let mut section = Section::Optional;
    let formated_output;

    if body.is_empty() {
        section = Section::Empty;

        let wave_output_values: Value = upon::value! {
            body: title.to_string(),
        };

        formated_output = get_file_chunk_output(EMPTY_TEMPLATE_CONTENT, wave_output_values)?;
    } else {
        let wave_output_values: Value = upon::value! {
            title: title,
            body: &body,
        };

        formated_output = get_file_chunk_output(TEMPLATE_CONTENT, wave_output_values)?;
    }

    Ok(OutputEntry {
        section,
        text: formated_output,
    })
}
