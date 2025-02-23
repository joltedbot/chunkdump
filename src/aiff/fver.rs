use crate::byteio::{take_first_four_bytes_as_unsigned_integer, Endian};
use crate::errors::LocalError;
use crate::formating::format_mac_hfs_timestamp_as_date_time_string;
use crate::template::Template;
use upon::Value;

const TEMPLATE_NAME: &str = "fver"; // A short name to identify the template
const TEMPLATE_CONTENT: &str = include_str!("../templates/aiff/fver.tmpl"); // The file path where you placed the template

#[derive(Debug, Clone, Default)]
pub struct FormatVersionFields {
    timestamp: String,
}

impl FormatVersionFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, LocalError> {
        let mac_hfs_format_timestamp = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
        let timestamp = format_mac_hfs_timestamp_as_date_time_string(mac_hfs_format_timestamp)?;
        Ok(Self { timestamp })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        let aiff_output_values: Value = upon::value! {
            timestamp: &self.timestamp,
        };

        let formated_output = template.get_wave_chunk_output(TEMPLATE_NAME, TEMPLATE_CONTENT, aiff_output_values)?;
        Ok(formated_output)
    }
}
