use crate::byteio::{
    take_first_four_bytes_as_unsigned_integer, take_first_number_of_bytes_as_string,
    take_first_two_bytes_as_signed_integer, take_first_two_bytes_as_unsigned_integer, Endian,
};
use crate::errors::LocalError;
use crate::formating::format_mac_hfs_timestamp_as_date_time_string;
use crate::template::Template;
use serde::Serialize;
use upon::Value;

const TEMPLATE_NAME: &str = "comt"; // A short name to identify the template
const TEMPLATE_CONTENT: &str = include_str!("../templates/aiff/comt.tmpl"); // The file path where you placed the template
const EMPTY_COMMENT_MESSAGE: &str = "[No Timestamp]";

#[derive(Debug, Clone, Serialize)]
pub struct Comment {
    timestamp: String,
    marker_id: i16,
    length: u16,
    comment: String,
}
// Rename the struct to reflect your new chunk nmae
#[derive(Debug, Clone, Default)]
pub struct CommentFields {
    comments: Vec<Comment>,
}

impl CommentFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, LocalError> {
        let number_of_comments = take_first_two_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
        let mut comments: Vec<Comment> = vec![];

        for _ in 0..number_of_comments {
            let mac_hfs_format_timestamp = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
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

        Ok(Self { comments })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        template.add_chunk_template(TEMPLATE_NAME, TEMPLATE_CONTENT)?;

        let aiff_output_values: Value = upon::value! {
            comments: &self.comments,
        };

        let formated_output = template.get_wave_chunk_output(TEMPLATE_NAME, TEMPLATE_CONTENT, aiff_output_values)?;
        Ok(formated_output)
    }
}
