use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/skipped.tmpl");

pub fn get_metadata(chunk_id: String) -> Result<OutputEntry, Box<dyn Error>> {
    let wave_output_values: Value = upon::value! {
        chunk_id: chunk_id
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, wave_output_values)?;

    Ok(OutputEntry {
        section: Section::Skipped,
        text: formated_output,
    })
}
