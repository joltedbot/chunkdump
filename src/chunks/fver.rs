use crate::bytes::{take_first_four_bytes_as_unsigned_integer, Endian};
use crate::chunks::{Chunk, Section};
use crate::formating::format_mac_hfs_timestamp_as_date_time_string;
use crate::template::get_file_chunk_output;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/fver.tmpl");

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<Chunk, Box<dyn Error>> {
    let mac_hfs_format_timestamp = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
    let timestamp = format_mac_hfs_timestamp_as_date_time_string(mac_hfs_format_timestamp)?;

    let aiff_output_values: Value = upon::value! {
        timestamp: timestamp,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, aiff_output_values)?;

    Ok(Chunk {
        section: Section::Optional,
        text: formated_output,
    })
}
