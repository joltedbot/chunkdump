use crate::byte_arrays::{
    take_first_byte_as_unsigned_integer, take_first_four_bytes_as_signed_integer, take_first_number_of_bytes_as_string,
    take_first_ten_bytes_as_an_apple_extended_integer, take_first_two_bytes_as_signed_integer, Endian,
};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/comm.tmpl");
const COMPRESSION_NAME_LENGTH_IN_BYTES: usize = 4;

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let number_of_channels = take_first_two_bytes_as_signed_integer(&mut chunk_data, Endian::Big)?;
    let sample_frames = take_first_four_bytes_as_signed_integer(&mut chunk_data, Endian::Big)?;
    let sample_size = take_first_two_bytes_as_signed_integer(&mut chunk_data, Endian::Big)?;
    let sample_rate = take_first_ten_bytes_as_an_apple_extended_integer(&mut chunk_data)?;

    let mut compression_type = String::new();
    let mut compression_name = String::new();

    if !chunk_data.is_empty() {
        compression_type = take_first_number_of_bytes_as_string(&mut chunk_data, COMPRESSION_NAME_LENGTH_IN_BYTES)?;

        let compression_name_size = take_first_byte_as_unsigned_integer(&mut chunk_data, Endian::Big)? as usize;
        compression_name = take_first_number_of_bytes_as_string(&mut chunk_data, compression_name_size)?;
    }

    let sample_rate = format!("{:#.1}", sample_rate.to_f64() / 1000.0);

    let aiff_output_values: Value = upon::value! {
        number_of_channels: number_of_channels,
        sample_frames: sample_frames,
        sample_size: sample_size,
        sample_rate: sample_rate,
        compression_name: &compression_name,
        compression_type: &compression_type,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, aiff_output_values)?;

    Ok(OutputEntry {
        section: Section::Mandatory,
        text: formated_output,
    })
}
