use crate::byte_arrays::{
    skip_over_bytes, take_first_byte, take_first_eight_bytes_as_unsigned_integer, take_first_number_of_bytes_as_string,
    Endian,
};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use byte_unit::rust_decimal::prelude::Zero;
use serde::Serialize;
use std::error::Error;
use upon::Value;

#[derive(Serialize)]
struct CueTrack {
    track_offset: u64,
    track_number: u8,
    track_isrc: String,
    is_audio: bool,
    pre_emphasis: bool,
    number_of_index_points: u8,
    points: Vec<IndexPoint>,
}

#[derive(Serialize)]
struct IndexPoint {
    offset_samples: u64,
    point_number: u8,
}

const TEMPLATE_CONTENT: &str = include_str!("../templates/blocks/cuesheet.tmpl");
const TRACK_ISRC_FIELD_LENGTH_IN_BYTES: usize = 12;
const CUESHEET_RESERVED_BYTES: usize = 258;
const POINT_RESERVED_BYTES: usize = 3;

pub fn get_metadata(mut block_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let media_catalog_number = take_first_number_of_bytes_as_string(&mut block_data, 128)?;
    let number_of_lead_in_samples = take_first_eight_bytes_as_unsigned_integer(&mut block_data, Endian::Big)?;
    let is_cdda = take_first_byte(&mut block_data)?;
    skip_over_bytes(&mut block_data, CUESHEET_RESERVED_BYTES)?;
    let number_of_tracks = take_first_byte(&mut block_data)?;

    let tracks = get_cuesheet_tracks_from_block_data(&mut block_data, number_of_tracks)?;

    let output_values: Value = upon::value! {
        media_catalog_number: media_catalog_number,
        number_of_lead_in_samples: number_of_lead_in_samples,
        is_cdda: is_cdda,
        number_of_tracks: number_of_tracks,
        tracks: tracks,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}

fn get_cuesheet_tracks_from_block_data(
    block_data: &mut Vec<u8>,
    number_of_tracks: u8,
) -> Result<Vec<CueTrack>, Box<dyn Error>> {
    let mut tracks: Vec<CueTrack> = Vec::new();

    for _ in 0..number_of_tracks {
        let track_offset = take_first_eight_bytes_as_unsigned_integer(block_data, Endian::Big)?;
        let track_number = take_first_byte(block_data)?;
        let track_isrc = take_first_number_of_bytes_as_string(block_data, TRACK_ISRC_FIELD_LENGTH_IN_BYTES)?;
        let flag_byte = take_first_byte(block_data)?;
        let number_of_index_points = take_first_byte(block_data)?;

        let flags: (bool, bool) = get_cue_track_flags_from_flag_byte(flag_byte)?;
        let points = get_index_points_from_block_data(block_data, number_of_index_points)?;

        tracks.push(CueTrack {
            track_offset,
            track_number,
            track_isrc,
            is_audio: flags.0,
            pre_emphasis: flags.1,
            number_of_index_points,
            points,
        });
    }
    Ok(tracks)
}

fn get_index_points_from_block_data(
    block_data: &mut Vec<u8>,
    number_of_index_points: u8,
) -> Result<Vec<IndexPoint>, Box<dyn Error>> {
    let mut points: Vec<IndexPoint> = Vec::new();

    if !number_of_index_points.is_zero() {
        let offset_samples = take_first_eight_bytes_as_unsigned_integer(block_data, Endian::Big)?;
        let point_number = take_first_byte(block_data)?;
        skip_over_bytes(block_data, POINT_RESERVED_BYTES)?;

        points.push(IndexPoint {
            offset_samples,
            point_number,
        })
    }

    Ok(points)
}

fn get_cue_track_flags_from_flag_byte(block_data: u8) -> Result<(bool, bool), Box<dyn Error>> {
    let is_audio_bit = (block_data >> 7) & 1;
    let is_audio = is_audio_bit == 1;

    let pre_emphasis_bit = (block_data >> 6) & 1;
    let pre_emphasis = pre_emphasis_bit == 1;

    Ok((is_audio, pre_emphasis))
}
