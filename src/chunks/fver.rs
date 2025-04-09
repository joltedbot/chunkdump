use crate::byte_arrays::{take_first_four_bytes_as_unsigned_integer, Endian};
use crate::formating::format_mac_hfs_timestamp_as_date_time_string;
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/fver.tmpl");

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let mac_hfs_format_timestamp =
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
    let timestamp = format_mac_hfs_timestamp_as_date_time_string(mac_hfs_format_timestamp)?;

    let output_values: Value = upon::value! {
        timestamp: timestamp,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}
