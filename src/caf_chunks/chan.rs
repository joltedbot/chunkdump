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
    channel_flags: u32,
    coordinates: [f32; 3],
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
    let channel_flags = take_first_four_bytes_as_unsigned_integer(chunk_data, Endian::Big)?;
    let mut coordinates: [f32; COORDINATE_NUMBER_OF_COMPONENTS] = Default::default();

    for i in 0..COORDINATE_NUMBER_OF_COMPONENTS {
        coordinates[i] = take_first_four_bytes_as_float(chunk_data, Endian::Big)?;
    }

    Ok(ChannelDescription {
        channel_label,
        channel_flags,
        coordinates,
    })
}

fn get_channel_layout_from_bitmap(bitmap: u32) -> Vec<String> {
    let mut channels: Vec<String> = Vec::new();

    for i in 0..CHANNEL_LAYOUT.len() {
        if bitmap & (1 << i) != 0 {
            channels.push(CHANNEL_LAYOUT[i].to_string());
        }
    }

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
