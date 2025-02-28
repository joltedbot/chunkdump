use crate::chunks::{Chunk, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/empty.tmpl");

pub fn get_metadata(chunk_id: String) -> Result<Chunk, Box<dyn Error>> {
    let wave_output_values: Value = upon::value! {
        chunk_id: chunk_id
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, wave_output_values)?;

    Ok(Chunk {
        section: Section::Empty,
        text: formated_output,
    })
}
