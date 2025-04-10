use crate::byte_arrays::{take_first_four_bytes_as_unsigned_integer, Endian};
use crate::caf_chunks::mark::{get_markers_from_bytes, Marker};
use crate::formating::format_bit_as_bool_string;
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use serde::Serialize;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/caf_chunks/regn.tmpl");
pub const TIMECODE_TYPE: [&str; 9] = [
    "No timecode type is assigned. Use this value if you are not specifying a SMPTE time in the marker.",
    "24 video frames per second—standard for 16mm and 35mm film.",
    "25 video frames per second—standard for PAL and SECAM video.",
    "30 video frames per second, with video-frame-number counts adjusted to ensure that the timecode matches elapsed clock time.",
    "30 video frames per second.",
    "29.97 video frames per second—standard for NTSC video.",
    "29.97 video frames per second—standard for NTSC video—with video-frame-number counts adjusted to ensure that the timecode matches elapsed clock time.",
    "60 video frames per second.",
    "59.94 video frames per second.",
];

pub const TIMECODE_TYPE_UKNOWN_MESSAGE: &str = "The Timcode Type is unknown or invalid";

#[derive(Default, Debug, Serialize)]
struct Region {
    id: u32,
    loop_enable: String,
    play_forward: String,
    play_backward: String,
    number_of_markers: u32,
    markers: Vec<Marker>,
}

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let smpte_time_type = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
    let number_of_regions =
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;

    let regions: Vec<Region> = get_regions_from_bytes(&mut chunk_data, number_of_regions)?;

    let output_values: Value = upon::value! {
        smpte_time_type: get_time_type_from_smpte_type_number(smpte_time_type),
        number_regions: number_of_regions,
        regions: regions,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}

pub fn get_time_type_from_smpte_type_number(smpte_time_type: u32) -> String {
    if smpte_time_type >= TIMECODE_TYPE.len() as u32 {
        return TIMECODE_TYPE_UKNOWN_MESSAGE.to_string();
    }

    TIMECODE_TYPE[smpte_time_type as usize].to_string()
}

fn get_regions_from_bytes(
    chunk_data: &mut Vec<u8>,
    number_of_regions: u32,
) -> Result<Vec<Region>, Box<dyn Error>> {
    let mut regions: Vec<Region> = Vec::new();

    for _ in 0..number_of_regions {
        regions.push(get_region(chunk_data)?);
    }

    Ok(regions)
}

fn get_region(chunk_data: &mut Vec<u8>) -> Result<Region, Box<dyn Error>> {
    let id = take_first_four_bytes_as_unsigned_integer(chunk_data, Endian::Big)?;
    let flag_bytes = take_first_four_bytes_as_unsigned_integer(chunk_data, Endian::Big)?;
    let number_of_markers = take_first_four_bytes_as_unsigned_integer(chunk_data, Endian::Big)?;
    let markers: Vec<Marker> = get_markers_from_bytes(chunk_data, number_of_markers)?;

    let flags = get_flags_from_flag_bytes(flag_bytes);

    Ok(Region {
        id,
        loop_enable: format_bit_as_bool_string(flags.0),
        play_forward: format_bit_as_bool_string(flags.1),
        play_backward: format_bit_as_bool_string(flags.2),
        number_of_markers,
        markers,
    })
}

fn get_flags_from_flag_bytes(flag_bytes: u32) -> (u8, u8, u8) {
    let loop_enable = (flag_bytes & 1) as u8;
    let play_forward = ((flag_bytes >> 1) & 1) as u8;
    let play_backward = ((flag_bytes >> 2) & 1) as u8;

    (loop_enable, play_forward, play_backward)
}
