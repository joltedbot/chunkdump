use crate::byte_arrays::{
    take_first_four_bytes_as_unsigned_integer, take_first_number_of_bytes_as_string,
    take_first_two_bytes_as_signed_integer, take_first_two_bytes_as_unsigned_integer, Endian,
};
use crate::formating::format_mac_hfs_timestamp_as_date_time_string;
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use serde::Serialize;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/comt.tmpl");
const EMPTY_COMMENT_MESSAGE: &str = "[No Timestamp]";

#[derive(Serialize)]
pub struct Comment {
    timestamp: String,
    marker_id: i16,
    length: u16,
    comment: String,
}

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let number_of_comments =
        take_first_two_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
    let mut comments: Vec<Comment> = vec![];

    for _ in 0..number_of_comments {
        let mac_hfs_format_timestamp =
            take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
        let mut timestamp = String::from(EMPTY_COMMENT_MESSAGE);
        if mac_hfs_format_timestamp != 0 {
            timestamp = format_mac_hfs_timestamp_as_date_time_string(mac_hfs_format_timestamp)?;
        }
        let marker_id = take_first_two_bytes_as_signed_integer(&mut chunk_data, Endian::Big)?;
        let length = take_first_two_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
        let comment = take_first_number_of_bytes_as_string(&mut chunk_data, length as usize)?
            .trim()
            .to_string();

        comments.push(Comment {
            timestamp,
            marker_id,
            length,
            comment,
        })
    }

    let output_values: Value = upon::value! {
        comments: comments,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}
