use crate::byte_arrays::{
    take_first_byte_as_signed_integer, take_first_eight_bytes_as_float,
    take_first_four_bytes_as_unsigned_integer, take_first_number_of_bytes_as_string, Endian,
};
use crate::caf_chunks::regn::get_time_type_from_smpte_type_number;
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use upon::Value;

const MARKER_TYPES: [(&str, &str); 19] = [
    ("", "Generic marker"),
    ("pbeg", "Start-of-program marker; used to delineate the start of a CD or other playlist."),
    ("pend", "End-of-program marker; used to delineate the end of a CD."),
    ("tbeg", "Start-of-track marker; used to delineate the start of a track for a CD."),
    ("tend", "End-of-track marker; used to delineate the end of a track for a CD."),
    ("indx", "Index marker for a Red Book compliant index."),
    ("rbeg", "Start-of-region marker. See Region Chunk."),
    ("rend", "End-of-region marker. See Region Chunk."),
    ("rsyc", "Region synchronization point marker; used to synchronize a point in (or external to) a region with an event, such as beat in the music."),
    ("sbeg", "Start-of-selection marker, for user selection of a portion of a displayed waveform."),
    ("send", "End-of-selection marker, for user selection of a portion of a displayed waveform."),
    ("cbeg", "Beginning-of-source marker for a copy or move operation."),
    ("cend", "End-of-source marker for a copy or move operation."),
    ("dbeg", "Beginning-of-destination marker for a copy or move operation."),
    ("dend", "End-of-destination marker for a copy or move operation."),
    ("slbg", "Start-of-sustain marker for a sustain loop."),
    ("slen", "End-of-sustain marker for a sustain loop."),
    ("rlbg", "Start-of-release marker for a sustain loop."),
    ("rlen", "End-of-release marker for a sustain loop."),
];

const MARKER_TYPE_LENGTH_IN_BYTES: usize = 4;
const TEMPLATE_CONTENT: &str = include_str!("../templates/caf_chunks/mark.tmpl");

#[derive(Default, Debug, Serialize)]
pub struct CafSmpteTimestamp {
    hours: i8,
    minutes: i8,
    seconds: i8,
    frames: i8,
    sub_frame_sample_offset: u32,
}

#[derive(Default, Debug, Serialize)]
pub struct Marker {
    marker_type: String,
    frame_position: f64,
    id: u32,
    smpte_time: String,
    channel: u32,
}

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let smpte_time_type = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
    let number_of_markers =
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
    let markers: Vec<Marker> =
        get_markers_from_bytes(&mut chunk_data, smpte_time_type, number_of_markers)?;

    let output_values: Value = upon::value! {
        smpte_time_type: get_time_type_from_smpte_type_number(smpte_time_type),
        number_of_markers: number_of_markers,
        markers: markers,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}

pub fn get_markers_from_bytes(
    chunk_data: &mut Vec<u8>,
    smpte_time_type: u32,
    number_of_markers: u32,
) -> Result<Vec<Marker>, Box<dyn Error>> {
    let mut markers: Vec<Marker> = Vec::new();

    for _ in 0..number_of_markers {
        markers.push(get_marker(smpte_time_type, chunk_data)?);
    }

    Ok(markers)
}

pub fn get_marker(
    smpte_time_type: u32,
    chunk_data: &mut Vec<u8>,
) -> Result<Marker, Box<dyn Error>> {
    let marker_types_list: HashMap<&str, &str> = HashMap::from(MARKER_TYPES);
    let marker_type_id =
        take_first_number_of_bytes_as_string(chunk_data, MARKER_TYPE_LENGTH_IN_BYTES)?;
    let frame_position = take_first_eight_bytes_as_float(chunk_data, Endian::Big)?;
    let id = take_first_four_bytes_as_unsigned_integer(chunk_data, Endian::Big)?;
    let smpte_time_components = get_smpte_time_from_bytes(chunk_data)?;
    let channel = take_first_four_bytes_as_unsigned_integer(chunk_data, Endian::Big)?;

    let marker_type = marker_types_list[marker_type_id.as_str()].to_string();
    let smpte_time = if smpte_time_type == 0 {
        String::new()
    } else {
        format_caf_smpte_timestamp(smpte_time_components)
    };

    Ok(Marker {
        marker_type,
        frame_position,
        id,
        smpte_time,
        channel,
    })
}

fn get_smpte_time_from_bytes(
    chunk_data: &mut Vec<u8>,
) -> Result<CafSmpteTimestamp, Box<dyn Error>> {
    let hours = take_first_byte_as_signed_integer(chunk_data)?;
    let minutes = take_first_byte_as_signed_integer(chunk_data)?;
    let seconds = take_first_byte_as_signed_integer(chunk_data)?;
    let frames = take_first_byte_as_signed_integer(chunk_data)?;
    let sub_frame_sample_offset =
        take_first_four_bytes_as_unsigned_integer(chunk_data, Endian::Big)?;

    Ok(CafSmpteTimestamp {
        hours,
        minutes,
        seconds,
        frames,
        sub_frame_sample_offset,
    })
}

fn format_caf_smpte_timestamp(timestamp: CafSmpteTimestamp) -> String {
    format!(
        "{}:{}:{}: {} frames & {} sub frame samples ",
        timestamp.hours,
        timestamp.minutes,
        timestamp.seconds,
        timestamp.frames,
        timestamp.sub_frame_sample_offset
    )
}
