use crate::byte_arrays::{take_first_four_bytes_as_unsigned_integer, take_first_number_of_bytes_as_string, Endian};
use crate::formating::format_file_size_as_string;
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::collections::HashMap;
use std::error::Error;
use upon::Value;

const PICTURE_TYPE: [(u32, &str); 21] = [
    (0, "Other"),
    (1, "PNG file icon of 32x32 pixels (see [RFC2083])"),
    (2, "General file icon"),
    (3, "Front cover"),
    (4, "Back cover"),
    (5, "Liner notes page"),
    (6, "Media label (e.g., CD, Vinyl or Cassette label)"),
    (7, "Lead artist, lead performer, or soloist"),
    (8, "Artist or performer"),
    (9, "Conductor"),
    (10, "Band or orchestra"),
    (11, "Composer"),
    (12, "Lyricist or text writer"),
    (13, "Recording location"),
    (14, "During recording"),
    (15, "During performance"),
    (16, "Movie or video screen capture"),
    (17, "A bright colored fish"),
    (18, "Illustration"),
    (19, "Band or artist logotype"),
    (20, "Publisher or studio logotype"),
];

const TEMPLATE_CONTENT: &str = include_str!("../templates/blocks/picture.tmpl");

pub fn get_metadata(mut block_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let picture_type_names: HashMap<u32, &str> = HashMap::from(PICTURE_TYPE);
    let picture_type_id = take_first_four_bytes_as_unsigned_integer(&mut block_data, Endian::Big)?;
    let picture_type = picture_type_names.get(&picture_type_id).unwrap_or(&"Other");

    let media_type_length_in_bytes = take_first_four_bytes_as_unsigned_integer(&mut block_data, Endian::Big)?;
    let media_type = take_first_number_of_bytes_as_string(&mut block_data, media_type_length_in_bytes as usize)?;
    let description_length_in_bytes = take_first_four_bytes_as_unsigned_integer(&mut block_data, Endian::Big)?;
    let description = take_first_number_of_bytes_as_string(&mut block_data, description_length_in_bytes as usize)?;
    let picture_width_in_pixels = take_first_four_bytes_as_unsigned_integer(&mut block_data, Endian::Big)?;
    let picture_height_in_pixels = take_first_four_bytes_as_unsigned_integer(&mut block_data, Endian::Big)?;
    let colour_depth_in_bits_per_pixel = take_first_four_bytes_as_unsigned_integer(&mut block_data, Endian::Big)?;
    let number_of_colours = take_first_four_bytes_as_unsigned_integer(&mut block_data, Endian::Big)?;
    let picture_length_in_bytes = take_first_four_bytes_as_unsigned_integer(&mut block_data, Endian::Big)?;

    let output_values: Value = upon::value! {
        picture_type: picture_type,
        media_type: media_type,
        description: description,
        width: picture_width_in_pixels,
        height: picture_height_in_pixels,
        colour_depth: colour_depth_in_bits_per_pixel,
        number_of_colours: number_of_colours,
        picture_length_in_bytes: format_file_size_as_string(picture_length_in_bytes as u64),
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}
