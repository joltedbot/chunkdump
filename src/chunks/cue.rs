use crate::byte_arrays::{take_first_four_bytes_as_unsigned_integer, take_first_number_of_bytes_as_string, Endian};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use serde::Serialize;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/cue.tmpl");
const DATA_CHUNK_ID_LENGTH_IN_BYTES: usize = 4;

#[derive(Serialize)]
struct CuePoint {
    id: u32,
    position: u32,
    data_chunk_id: String,
    chunk_start: u32,
    block_start: u32,
    sample_start: u32,
}

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let mut cue_points: Vec<CuePoint> = vec![];
    let number_of_cue_points: u32 = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;

    for _ in 0..number_of_cue_points {
        cue_points.push(CuePoint {
            id: take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?,
            position: take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?,
            data_chunk_id: take_first_number_of_bytes_as_string(&mut chunk_data, DATA_CHUNK_ID_LENGTH_IN_BYTES)?,
            chunk_start: take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?,
            block_start: take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?,
            sample_start: take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?,
        })
    }

    let wave_output_values: Value = upon::value! {
            number_of_cue_points: &number_of_cue_points,
            cue_points: &cue_points
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, wave_output_values)?;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}
