use crate::file_types::midi::{
    get_header_metadata_from_midi_data, get_midi_meta_events_from_track_data,
    get_track_data_from_midi_data, MetaEvent,
};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/caf_chunks/midi.tmpl");
const HEADER_TEMPLATE_CONTENT: &str = include_str!("../templates/midi/header.tmpl");
const META_EVENTS_TEMPLATE_CONTENT: &str = include_str!("../templates/midi/meta_events.tmpl");

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let header = get_header_metadata_from_midi_data(&mut chunk_data)?;

    let mut meta_events: Vec<MetaEvent> = vec![];

    for track_number in 1..=header.number_of_tracks {
        let track_data = get_track_data_from_midi_data(&mut chunk_data)?;
        meta_events.extend(get_midi_meta_events_from_track_data(
            track_number,
            track_data,
        )?);
    }

    let mut formated_output = TEMPLATE_CONTENT.to_string();

    let header_output_values: Value = upon::value! {
        format: &header.format,
        number_of_tracks: header.number_of_tracks,
        ppqn: header.division.ppqn,
        timecode: header.division.timecode,
        ticks_per_frame: header.division.ticks_per_frame,
    };

    formated_output +=
        get_file_chunk_output(HEADER_TEMPLATE_CONTENT, header_output_values)?.as_str();

    let meta_events_output_values: Value = upon::value! {
        meta_event_data: meta_events,
    };

    formated_output +=
        get_file_chunk_output(META_EVENTS_TEMPLATE_CONTENT, meta_events_output_values)?.as_str();

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}
