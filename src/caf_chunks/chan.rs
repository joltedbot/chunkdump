use crate::byte_arrays::{
    take_first_four_bytes_as_float, take_first_four_bytes_as_unsigned_integer, Endian,
};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use serde::Serialize;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/caf_chunks/chan.tmpl");
const CHANNEL_LAYOUT: [&str; 18] = [
    "Left",
    "Right",
    "Center",
    "LFE Screen",
    "Left Surround",
    "Right Surround",
    "Left Center",
    "Right Center",
    "Center Surround",
    "Left Surround Direct",
    "Right Surround Direct",
    "Top Center Surround",
    "Vertical Height Left",
    "Vertical Height Center",
    "Vertical Height Right",
    "Top Back Left",
    "Top Back Center",
    "Top Back Right",
];

const CHANNEL_LAYOUT_TAGS_CHANNEL_ORDER: [&str; 12] = [
    "A Standard Mono Stream",
    "A Standard Stereo Stream (L R) - Implies headphone playback.",
    "A matrix encoded stereo stream (Lt, Rt)",
    "Stereo - Mid/Side Recording",
    "Stereo - Coincident mic pair (often 2 figure 8's)",
    "Stereo - binaural stereo (Left, Right)",
    "Symmetric Ambisonic B Format - W, X, Y, Z",
    "Quadraphonic - Front Left, Front Right, Back Left, Back Right",
    "Pentagonal - Left, Right, Rear Left, Rear Right, Center",
    "Hexagonal - Left, Right, Rear Left, Rear Right, Center, Rear",
    "Octagonal - Front Left, Front Right, Rear Left, Rear Right, Front Center, Rear Center, Side Left, Side Right",
    "Cube - Left, Right, Rear Left, Rear Right, Top Left, Top Right, Top Rear Left, Top Rear Right",
];

const CHANNEL_LAYOUT_FLAGS: [&str; 5] = [
    "All Off",
    "Rectangular Coordinates",
    "Spherical Coordinates",
    "Unknown",
    "Meters",
];

const CHANNEL_LAYOUT_TAG_LOW_BIT_MASK: u32 = 65535;
const CHANNEL_LAYOUT_USE_BITMASK_VALUE: u32 = 65536;
const CHANNEL_LAYOUT_USE_DESCRIPTIONS_MESSAGE: &str = "Use Audio Channel Descriptions";
const CHANNEL_LAYOUT_USE_BITMASK_MESSAGE: &str = "Use Audio Channel Bitmask";
const CHANNEL_LAYOUT_TAG_NUMBER_OF_LOW_BITS: u32 = 16;
const CHANNEL_LAYOUT_HIGH_BITS_CHANNEL_ORDER_INDEX_OFFSET: u32 = 100;
const COORDINATE_NUMBER_OF_COMPONENTS: usize = 3;

#[derive(PartialEq, Debug, Serialize)]
struct ChannelDescription {
    channel_label: u32,
    channel_flags: String,
    coordinates: Vec<f32>,
}

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let channel_layout_tag =
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
    let channel_bitmap = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
    let number_of_channel_descriptions =
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
    let channel_descriptions =
        take_channel_descriptions_from_bytes(&mut chunk_data, number_of_channel_descriptions)?;
    let channel_layout = get_channel_layout_tags_from_integer(channel_layout_tag);

    let output_values: Value = upon::value! {
        channel_layout_tag: channel_layout.0,
        number_of_channels: channel_layout.1,
        channel_bitmap: get_channel_layout_from_bitmap(channel_bitmap),
        number_of_channel_descriptions: number_of_channel_descriptions,
        channel_descriptions: channel_descriptions,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Mandatory,
        text: formated_output,
    })
}

fn take_channel_descriptions_from_bytes(
    chunk_data: &mut Vec<u8>,
    number_of_channel_descriptions: u32,
) -> Result<Vec<ChannelDescription>, Box<dyn Error>> {
    let mut channel_descriptions: Vec<ChannelDescription> = Vec::new();
    for _ in 0..number_of_channel_descriptions {
        channel_descriptions.push(get_channel_description(chunk_data)?);
    }
    Ok(channel_descriptions)
}

fn get_channel_description(chunk_data: &mut Vec<u8>) -> Result<ChannelDescription, Box<dyn Error>> {
    let channel_label = take_first_four_bytes_as_unsigned_integer(chunk_data, Endian::Big)?;
    let channel_flag_raw = take_first_four_bytes_as_unsigned_integer(chunk_data, Endian::Big)?;
    let mut coordinates: Vec<f32> = Default::default();

    let channel_flags = CHANNEL_LAYOUT_FLAGS[channel_flag_raw as usize].to_string();

    for _ in 0..COORDINATE_NUMBER_OF_COMPONENTS {
        coordinates.push(take_first_four_bytes_as_float(chunk_data, Endian::Big)?);
    }

    Ok(ChannelDescription {
        channel_label,
        channel_flags,
        coordinates,
    })
}

fn get_channel_layout_from_bitmap(bitmap: u32) -> Vec<String> {
    let mut channels: Vec<String> = Vec::new();

    CHANNEL_LAYOUT
        .iter()
        .enumerate()
        .for_each(|(bit_index, description)| {
            if bitmap & (1 << bit_index) != 0 {
                channels.push(description.to_string());
            }
        });

    channels
}

fn get_channel_layout_tags_from_integer(layout_tag: u32) -> (String, u32) {
    if layout_tag == 0 {
        return (CHANNEL_LAYOUT_USE_DESCRIPTIONS_MESSAGE.to_string(), 0);
    }

    if layout_tag == CHANNEL_LAYOUT_USE_BITMASK_VALUE {
        return (CHANNEL_LAYOUT_USE_BITMASK_MESSAGE.to_string(), 0);
    }

    let number_of_channels = layout_tag & CHANNEL_LAYOUT_TAG_LOW_BIT_MASK;
    let channel_order_index = (layout_tag >> CHANNEL_LAYOUT_TAG_NUMBER_OF_LOW_BITS)
        - CHANNEL_LAYOUT_HIGH_BITS_CHANNEL_ORDER_INDEX_OFFSET;

    (
        CHANNEL_LAYOUT_TAGS_CHANNEL_ORDER[channel_order_index as usize].to_string(),
        number_of_channels,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_correct_channel_desctiptions_from_valid_bytes() {
        let mut test_bytes = vec![
            0, 0, 0, 1, 0, 0, 0, 1, 63, 128, 0, 0, 64, 0, 0, 0, 64, 64, 0, 0, 0, 0, 0, 2, 0, 0, 0,
            2, 64, 128, 0, 0, 64, 160, 0, 0, 64, 192, 0, 0,
        ];
        let number_of_channel_descriptions = 2;
        let correct_channel_descriptions: Vec<ChannelDescription> = vec![
            ChannelDescription {
                channel_label: 1,
                channel_flags: "Rectangular Coordinates".to_string(),
                coordinates: vec![1.0, 2.0, 3.0],
            },
            ChannelDescription {
                channel_label: 2,
                channel_flags: "Spherical Coordinates".to_string(),
                coordinates: vec![4.0, 5.0, 6.0],
            },
        ];
        let channel_descriptions =
            take_channel_descriptions_from_bytes(&mut test_bytes, number_of_channel_descriptions)
                .unwrap();

        assert_eq!(channel_descriptions, correct_channel_descriptions);
    }

    #[test]
    fn test_get_channel_description() {
        let mut test_bytes = vec![
            0, 0, 0, 1, 0, 0, 0, 1, 63, 128, 0, 0, 64, 0, 0, 0, 64, 64, 0, 0,
        ];
        let expected_description = ChannelDescription {
            channel_label: 1,
            channel_flags: "Rectangular Coordinates".to_string(),
            coordinates: vec![1.0, 2.0, 3.0],
        };

        let channel_description = get_channel_description(&mut test_bytes).unwrap();
        assert_eq!(channel_description, expected_description);
    }

    #[test]
    fn test_get_channel_layout_from_bitmap() {
        let bitmap = 0b00000000000000000000000000000111; // First three channels active
        let expected_layout = vec![
            "Left".to_string(),
            "Right".to_string(),
            "Center".to_string(),
        ];

        let channel_layout = get_channel_layout_from_bitmap(bitmap);
        assert_eq!(channel_layout, expected_layout);
    }

    #[test]
    fn test_get_channel_layout_tags_from_integer() {
        let layout_tag = (100 + 1) << 16 | 2; // Second channel order, 2 channels
        let expected_layout = (
            "A Standard Stereo Stream (L R) - Implies headphone playback.".to_string(),
            2,
        );

        let channel_layout = get_channel_layout_tags_from_integer(layout_tag);
        assert_eq!(channel_layout, expected_layout);
    }

    #[test]
    fn test_get_channel_layout_tags_from_integer_use_descriptions() {
        let layout_tag = 0; // Early return for descriptions
        let expected_layout = ("Use Audio Channel Descriptions".to_string(), 0);

        let channel_layout = get_channel_layout_tags_from_integer(layout_tag);
        assert_eq!(channel_layout, expected_layout);
    }

    #[test]
    fn test_get_channel_layout_tags_from_integer_use_bitmask() {
        let layout_tag = 65536; // Early return for bitmask
        let expected_layout = ("Use Audio Channel Bitmask".to_string(), 0);

        let channel_layout = get_channel_layout_tags_from_integer(layout_tag);
        assert_eq!(channel_layout, expected_layout);
    }

    #[test]
    fn return_correct_metdata_output_entry_from_valid_bytes() {
        let chunk_data = vec![
            0, 105, 0, 0, // channel_layout_tag
            0, 0, 0, 7, // channel_bitmap
            0, 0, 0, 2, // number_of_channel_descriptions
            0, 0, 0, 1, 0, 0, 0, 1, 63, 128, 0, 0, 64, 0, 0, 0, 64, 64, 0,
            0, // First channel description
            0, 0, 0, 2, 0, 0, 0, 2, 64, 128, 0, 0, 64, 160, 0, 0, 64, 192, 0,
            0, // Second channel description
        ];

        let correct_result_text = "\n-----------------------------\nChannel Layout Chunk Details:\n-----------------------------\nChannel Layout Method:  Stereo - binaural stereo (Left, Right)\nnumber_of_channel_descriptions: 2\n\nBitmask Channels Present:\n-------------------------\n  - Left\n  - Right\n  - Center\n\nChannel Descriptions:\n---------------------\n   ---------------------\n   Channel Lable: 1\n   Channel Flags: Rectangular Coordinates\n   Co-ordinates:\n    - 1\n    - 2\n    - 3\n   ---------------------\n   Channel Lable: 2\n   Channel Flags: Spherical Coordinates\n   Co-ordinates:\n    - 4\n    - 5\n    - 6\n";
        let correct_result = OutputEntry {
            section: Section::Mandatory,
            text: correct_result_text.to_string(),
        };

        let result = get_metadata(chunk_data).unwrap();

        assert_eq!(result, correct_result);
    }
}
