use crate::byteio::{
    take_first_byte_as_unsigned_integer, take_first_four_bytes_as_unsigned_integer,
    take_first_number_of_bytes_as_string, take_first_two_bytes_as_signed_integer,
    take_first_two_bytes_as_unsigned_integer, Endian,
};
use crate::errors::LocalError;
use crate::fileio::add_one_if_byte_size_is_odd;
use crate::template::Template;
use serde::Serialize;
use upon::Value;

const TEMPLATE_NAME: &str = "mark"; // A short name to identify the template
const TEMPLATE_CONTENT: &str = include_str!("../templates/aiff/mark.tmpl"); // The file path where you placed the template
const PSTRING_TERMINATOR_BYTE_LENGTH: u32 = 1;

#[derive(Debug, Clone, Serialize)]
pub struct Marker {
    marker_id: i16,
    name: String,
    position: u32,
}
// Rename the struct to reflect your new chunk nmae
#[derive(Debug, Clone, Default)]
pub struct MarkerFields {
    markers: Vec<Marker>,
}

impl MarkerFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, LocalError> {
        let number_of_markers = take_first_two_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
        let mut markers: Vec<Marker> = vec![];

        for _ in 0..number_of_markers {
            let marker_id = take_first_two_bytes_as_signed_integer(&mut chunk_data, Endian::Big)?;
            let position = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;

            let name_size_unpadded = take_first_byte_as_unsigned_integer(&mut chunk_data, Endian::Big)? as u32;
            let name_size = add_one_if_byte_size_is_odd(name_size_unpadded) + PSTRING_TERMINATOR_BYTE_LENGTH;
            let name = take_first_number_of_bytes_as_string(&mut chunk_data, name_size as usize)?;

            markers.push(Marker {
                marker_id,
                name,
                position,
            })
        }

        Ok(Self { markers })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        let aiff_output_values: Value = upon::value! {
            markers: &self.markers,
        };

        let formated_output = template.get_wave_chunk_output(TEMPLATE_NAME, TEMPLATE_CONTENT, aiff_output_values)?;
        Ok(formated_output)
    }
}
