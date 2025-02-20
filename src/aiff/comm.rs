use crate::byteio::{
    take_first_byte_as_unsigned_integer, take_first_four_bytes_as_signed_integer, take_first_number_of_bytes_as_string,
    take_first_ten_bytes_as_an_apple_extended_integer, take_first_two_bytes_as_signed_integer, Endian,
};
use crate::errors::LocalError;
use crate::template::Template;
use extended::Extended;
use upon::Value;

const TEMPLATE_NAME: &str = "comm"; // A short name to identify the template
const TEMPLATE_CONTENT: &str = include_str!("../templates/aiff/comm.tmpl"); // The file path where you placed the template
const COMPRESSION_NAME_LENGTH_IN_BYTES: usize = 4;

// Rename the struct to reflect your new chunk nmae
#[derive(Debug, Clone)]
pub struct CommonFields {
    number_of_channels: i16,
    sample_frames: i32,
    sample_size: i16,
    sample_rate: Extended,
    compression_type: String,
    compression_name: String,
}

impl Default for CommonFields {
    fn default() -> Self {
        CommonFields {
            number_of_channels: 0,
            sample_frames: 0,
            sample_size: 0,
            sample_rate: Extended {
                sign_exponent: 0,
                fraction: 0,
            },
            compression_type: String::new(),
            compression_name: String::new(),
        }
    }
}

impl CommonFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, LocalError> {
        let number_of_channels = take_first_two_bytes_as_signed_integer(&mut chunk_data, Endian::Big)?;
        let sample_frames = take_first_four_bytes_as_signed_integer(&mut chunk_data, Endian::Big)?;
        let sample_size = take_first_two_bytes_as_signed_integer(&mut chunk_data, Endian::Big)?;
        let sample_rate = take_first_ten_bytes_as_an_apple_extended_integer(&mut chunk_data)?;
        let compression_type = take_first_number_of_bytes_as_string(&mut chunk_data, COMPRESSION_NAME_LENGTH_IN_BYTES)?;
        let compression_name_size = take_first_byte_as_unsigned_integer(&mut chunk_data, Endian::Big)? as usize;
        let compression_name = take_first_number_of_bytes_as_string(&mut chunk_data, compression_name_size)?;

        Ok(Self {
            number_of_channels,
            sample_frames,
            sample_size,
            sample_rate,
            compression_name,
            compression_type,
        })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        template.add_chunk_template(TEMPLATE_NAME, TEMPLATE_CONTENT)?;

        let sample_rate = format!("{:#.1}", self.sample_rate.to_f64() / 1000.0);

        let aiff_output_values: Value = upon::value! {
            number_of_channels: self.number_of_channels,
            sample_frames: self.sample_frames,
            sample_size: self.sample_size,
            sample_rate: sample_rate,
            compression_name: &self.compression_name,
            compression_type: &self.compression_type,
        };

        let formated_output = template.get_wave_chunk_output(TEMPLATE_NAME, TEMPLATE_CONTENT, aiff_output_values)?;
        Ok(formated_output)
    }
}
