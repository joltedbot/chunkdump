use crate::chunks::{Chunk, Section};
use crate::errors::LocalError;
use crate::template::get_file_chunk_output;
use flate2::read::ZlibDecoder;
use std::error::Error;
use std::io::prelude::*;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/resu.tmpl");

pub fn get_metadata(chunk_data: Vec<u8>) -> Result<Chunk, Box<dyn Error>> {
    let mut zlib = ZlibDecoder::new(chunk_data.as_slice());
    let mut resu_json = String::new();
    zlib.read_to_string(&mut resu_json)
        .map_err(|e| LocalError::InvalidZipDataFound(e.to_string()))?;

    let wave_output_values: Value = upon::value! {
        resu_json: resu_json.clone(),
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, wave_output_values)?;

    Ok(Chunk {
        section: Section::Optional,
        text: formated_output,
    })
}
