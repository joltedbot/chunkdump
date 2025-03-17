use crate::byte_arrays::{take_first_four_bytes_as_unsigned_integer, Endian};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/fact.tmpl");

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let samples_per_channel =
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;

    let wave_output_values: Value = upon::value! {
        samples_per_channel: samples_per_channel,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, wave_output_values)?;

    Ok(OutputEntry {
        section: Section::Mandatory,
        text: formated_output,
    })
}
