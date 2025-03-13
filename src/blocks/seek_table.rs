use crate::byte_arrays::{
    take_first_eight_bytes_as_unsigned_integer, take_first_two_bytes_as_unsigned_integer, Endian,
};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use serde::Serialize;
use std::error::Error;
use upon::Value;

const SEEK_TABLE_FIRST_SAMPLE_LENGTH_IN_BYTES: usize = 8;

#[derive(Serialize)]
struct Point {
    first_sample: u64,
    is_placeholder: bool,
    offset_in_bytes: u64,
    number_of_samples: u16,
}

const TEMPLATE_CONTENT: &str = include_str!("../templates/blocks/seek_table.tmpl");
const PLACEHOLDER_POINT_IDENTIFIER_BYTES: u64 = 0xFFFFFFFFFFFFFFFF;

pub fn get_metadata(mut block_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let mut seek_points: Vec<Point> = Vec::new();

    loop {
        if block_data.len() < SEEK_TABLE_FIRST_SAMPLE_LENGTH_IN_BYTES {
            break;
        }

        let first_sample = take_first_eight_bytes_as_unsigned_integer(&mut block_data, Endian::Big)?;
        let is_placeholder = first_sample == PLACEHOLDER_POINT_IDENTIFIER_BYTES;
        let mut offset_in_bytes: u64 = 0;
        let mut number_of_samples: u16 = 0;

        if !is_placeholder {
            offset_in_bytes = take_first_eight_bytes_as_unsigned_integer(&mut block_data, Endian::Big)?;
            number_of_samples = take_first_two_bytes_as_unsigned_integer(&mut block_data, Endian::Big)?;
        }

        let point = Point {
            first_sample,
            is_placeholder,
            offset_in_bytes,
            number_of_samples,
        };

        seek_points.push(point);
    }

    let output_values: Value = upon::value! {
        seek_points: seek_points,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}
