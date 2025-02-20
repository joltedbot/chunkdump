use crate::byteio::{take_first_four_bytes_as_signed_integer, Endian};
use crate::errors::LocalError;
use crate::template::Template;
use chrono::DateTime;
use upon::Value;

const TEMPLATE_NAME: &str = "fver"; // A short name to identify the template
const TEMPLATE_CONTENT: &str = include_str!("../templates/aiff/fver.tmpl"); // The file path where you placed the template
const BAD_TIMESTAMP_MESSAGE: &str = "Unexpected bad timestamp format";
const MAC_HFS_FORMAT_TIMESTAMP_OFFSET: i32 = 2082844800;

#[derive(Debug, Clone, Default)]
pub struct FormatVersionFields {
    timestamp: String,
}

impl FormatVersionFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, LocalError> {
        let mac_hfs_format_timestamp = take_first_four_bytes_as_signed_integer(&mut chunk_data, Endian::Big)?;

        let timestamp =
            match DateTime::from_timestamp((mac_hfs_format_timestamp + MAC_HFS_FORMAT_TIMESTAMP_OFFSET) as i64, 0) {
                Some(ts) => ts.to_string(),
                None => BAD_TIMESTAMP_MESSAGE.to_string(),
            };

        Ok(Self { timestamp })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        template.add_chunk_template(TEMPLATE_NAME, TEMPLATE_CONTENT)?;

        let aiff_output_values: Value = upon::value! {
            timestamp: &self.timestamp,
        };

        let formated_output = template.get_wave_chunk_output(TEMPLATE_NAME, TEMPLATE_CONTENT, aiff_output_values)?;
        Ok(formated_output)
    }
}
