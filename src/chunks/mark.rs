use crate::byte_arrays::{
    take_first_byte_as_unsigned_integer, take_first_four_bytes_as_unsigned_integer,
    take_first_number_of_bytes_as_string, take_first_two_bytes_as_signed_integer,
    take_first_two_bytes_as_unsigned_integer, Endian,
};
use crate::formating::add_one_if_byte_size_is_odd;
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use serde::Serialize;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/mark.tmpl");
const PSTRING_TERMINATOR_BYTE_LENGTH: u32 = 1;

#[derive(Serialize)]
pub struct Marker {
    marker_id: i16,
    name: String,
    position: u32,
}

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let number_of_markers = take_first_two_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
    let mut markers: Vec<Marker> = vec![];

    for _ in 0..number_of_markers {
        let marker_id = take_first_two_bytes_as_signed_integer(&mut chunk_data, Endian::Big)?;
        let position = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;

        let name_size_unpadded =
            take_first_byte_as_unsigned_integer(&mut chunk_data, Endian::Big)? as u32;
        let name_size =
            add_one_if_byte_size_is_odd(name_size_unpadded) + PSTRING_TERMINATOR_BYTE_LENGTH;
        let name = take_first_number_of_bytes_as_string(&mut chunk_data, name_size as usize)?;

        markers.push(Marker {
            marker_id,
            name,
            position,
        })
    }

    let aiff_output_values: Value = upon::value! {
        markers: markers,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, aiff_output_values)?;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}
