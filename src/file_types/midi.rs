use crate::byte_arrays::{
    take_first_byte, take_first_byte_as_signed_integer, take_first_byte_as_unsigned_integer,
    take_first_four_bytes_as_unsigned_integer, take_first_number_of_bytes,
    take_first_two_bytes_as_unsigned_integer, Endian,
};
use crate::chunks::CHUNK_SIZE_FIELD_LENGTH_IN_BYTES;
use crate::formating::{
    format_bytes_as_string, format_bytes_as_string_of_bytes, format_smpte_offset,
};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use byte_unit::rust_decimal::prelude::Zero;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use upon::Value;

const MAJOR_KEY_FLATS: [(i8, &str); 15] = [
    (7, "B"),
    (6, "Gb"),
    (5, "Db"),
    (4, "Ab"),
    (3, "Eb"),
    (2, "Bb"),
    (1, "F"),
    (0, "C"),
    (-1, "G"),
    (-2, "D"),
    (-3, "A"),
    (-4, "E"),
    (-5, "B"),
    (-6, "F#"),
    (-7, "C#"),
];
const MINOR_KEY_FLATS: [(i8, &str); 15] = [
    (7, "Ab"),
    (6, "Eb"),
    (5, "Bb"),
    (4, "F"),
    (3, "C"),
    (2, "G"),
    (1, "D"),
    (0, "A"),
    (-1, "E"),
    (-2, "B"),
    (-3, "F#"),
    (-4, "C#"),
    (-5, "G#"),
    (-6, "D#"),
    (-7, "A#"),
];

pub const MIDI_HEADER_CHUNK_DIVISION_LENGTH_IN_BYTES: usize = 2;
const MIDI_FORMAT_TYPE_ZERO: &str = "A single multi-channel track";
const MIDI_FORMAT_TYPE_ONE: &str = "One or more simultaneous tracks of a sequence";
const MIDI_FORMAT_TYPE_TWO: &str = "One or more sequential tracks of a sequence";
const METRONOME_CLICK_MESSAGE: &str = "Midi Clock Ticks Per Metronome Click";
const THIRTYSECOND_NOTES_PER_BEAT_MESSAGE: &str = "32nd Notes Per Beat";
const META_EVENT_SEQUENCE_NUMBER: &str = "Sequence Number";
const META_EVENT_TEXT: &str = "Text";
const META_EVENT_COPYRIGHT_NOTICE: &str = "Copyright Notice";
const META_EVENT_TRACK_NAME: &str = "Track Name";
const META_EVENT_INSTRUMENT_NAME: &str = "Instrument Name";
const META_EVENT_LYRICS: &str = "Lyrics";
const META_EVENT_MARKER: &str = "Marker";
const META_EVENT_CUE_POINT: &str = "Cue Point";
const META_EVENT_CHANNEL_PREFIX: &str = "Channel Prefix";
const META_EVENT_MIDI_PORT: &str = "MIDI Port";
const META_EVENT_DEVICE_NAME: &str = "Device Name";
const META_EVENT_SET_TEMPO: &str = "Set Tempo";
const META_EVENT_SMPTE_OFFSET: &str = "SMPTE Offset";
const META_EVENT_TIME_SIGNATURE: &str = "Time Signature";
const META_EVENT_KEY_SIGNATURE: &str = "Key Signature";
const META_EVENT_SEQUENCER_SPECIFIC: &str = "Sequencer specific Message";
const FIRST_BIT_AS_U8_DECIMAL_INTEGER: u8 = 128;
const META_EVENT_UNKNOWN: &str = "Unknown MetaEvent";
const MAJOR_SCALE_TITLE: &str = "Major";
const MINOR_SCALE_TITLE: &str = "minor";
const MICROSECONDS_PER_MINUTE: u32 = 60000000;
const META_EVENT_ID_BYTE_VALUE: u8 = 0xFF;
const SYSEX_EVENT_ID_BYTE_VALUE: u8 = 0xF0;
const META_EVENT_END_OFF_TRACK_BYTE_VALUE: u8 = 0x2F;
const HEADER_TEMPLATE_CONTENT: &str = include_str!("../templates/midi/header.tmpl");
const META_EVENT_TEMPLATE_CONTENT: &str = include_str!("../templates/midi/meta_events.tmpl");

#[derive(Default, Debug, PartialEq, Serialize)]
pub struct Division {
    pub ppqn: u16,
    pub timecode: u8,
    pub ticks_per_frame: u8,
}

#[derive(Default)]
pub struct Header {
    pub format: String,
    pub number_of_tracks: u16,
    pub division: Division,
}

#[derive(Serialize)]
pub struct MetaEvent {
    pub track_number: u16,
    pub delta_time: u32,
    pub name: String,
    pub value: String,
}

pub fn get_metadata_from_midi_data(
    midi_data: &mut Vec<u8>,
) -> Result<Vec<OutputEntry>, Box<dyn Error>> {
    let header = get_header_metadata_from_midi_data(midi_data)?;
    let header_chunk = get_header_chunk_from_header_metadata(&header)?;
    let meta_events_chunk = get_meta_events_from_track_data(midi_data, header.number_of_tracks)?;
    Ok(vec![header_chunk, meta_events_chunk])
}

pub fn get_header_metadata_from_midi_data(
    midi_data: &mut Vec<u8>,
) -> Result<Header, Box<dyn Error>> {
    let mut header_track_data = get_track_data_from_midi_data(midi_data)?;

    let raw_format = take_first_two_bytes_as_unsigned_integer(&mut header_track_data, Endian::Big)?;
    let format = get_file_format_from_raw_format_number(raw_format);

    let number_of_tracks =
        take_first_two_bytes_as_unsigned_integer(&mut header_track_data, Endian::Big)?;

    let raw_division = take_first_number_of_bytes(
        &mut header_track_data,
        MIDI_HEADER_CHUNK_DIVISION_LENGTH_IN_BYTES,
    )?;
    let division = get_division_output_values_from_raw_division_bytes(raw_division);

    Ok(Header {
        format,
        number_of_tracks,
        division,
    })
}

fn get_header_chunk_from_header_metadata(header: &Header) -> Result<OutputEntry, Box<dyn Error>> {
    let midi_output_values: Value = upon::value! {
        format: &header.format,
        number_of_tracks: header.number_of_tracks,
        ppqn: header.division.ppqn,
        timecode: header.division.timecode,
        ticks_per_frame: header.division.ticks_per_frame,
    };

    Ok(OutputEntry {
        section: Section::Mandatory,
        text: get_file_chunk_output(HEADER_TEMPLATE_CONTENT, midi_output_values)?,
    })
}

fn get_meta_events_from_track_data(
    midi_data: &mut Vec<u8>,
    number_of_tracks: u16,
) -> Result<OutputEntry, Box<dyn Error>> {
    let mut meta_events: Vec<MetaEvent> = vec![];

    for track_number in 1..=number_of_tracks {
        let track_data = get_track_data_from_midi_data(midi_data)?;
        meta_events.extend(get_midi_meta_events_from_track_data(
            track_number,
            track_data,
        )?);
    }

    let midi_output_values: Value = upon::value! {
        meta_event_data: meta_events,
    };

    let output = OutputEntry {
        section: Section::Optional,
        text: get_file_chunk_output(META_EVENT_TEMPLATE_CONTENT, midi_output_values)?,
    };

    Ok(output)
}

fn get_track_data_from_midi_data(midi_data: &mut Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let _ = take_first_number_of_bytes(midi_data, CHUNK_SIZE_FIELD_LENGTH_IN_BYTES)?;
    let track_chunk_size = take_first_four_bytes_as_unsigned_integer(midi_data, Endian::Big)?;
    let track_track_data = take_first_number_of_bytes(midi_data, track_chunk_size as usize)?;
    Ok(track_track_data)
}

pub fn get_midi_meta_events_from_track_data(
    track_number: u16,
    mut track_data: Vec<u8>,
) -> Result<Vec<MetaEvent>, Box<dyn Error>> {
    let mut meta_events: Vec<MetaEvent> = Vec::new();

    loop {
        let delta_time = get_variable_length_unsigned_integer_from_track_data(&mut track_data);
        let meta_event_id = take_first_byte(&mut track_data)?;
        let meta_event_type = take_first_byte(&mut track_data)?;

        if meta_event_type == META_EVENT_END_OFF_TRACK_BYTE_VALUE
            && meta_event_id == META_EVENT_ID_BYTE_VALUE
        {
            break;
        }

        let meta_event_data_length = if meta_event_id == META_EVENT_ID_BYTE_VALUE
            || meta_event_type == SYSEX_EVENT_ID_BYTE_VALUE
        {
            get_variable_length_unsigned_integer_from_track_data(&mut track_data)
        } else {
            take_first_byte_as_unsigned_integer(&mut track_data, Endian::Big)? as u32
        };

        if meta_event_data_length == 0 || track_data.len() < meta_event_data_length as usize {
            break;
        }

        let meta_event_data =
            take_first_number_of_bytes(&mut track_data, meta_event_data_length as usize)?;

        if meta_event_id == META_EVENT_ID_BYTE_VALUE {
            let meta_event =
                get_meta_event_from_event_data_by_type_byte(meta_event_type, meta_event_data)?;
            meta_events.push(MetaEvent {
                track_number,
                delta_time,
                name: meta_event.0,
                value: meta_event.1,
            });
        }
    }

    Ok(meta_events)
}

fn get_meta_event_from_event_data_by_type_byte(
    type_byte: u8,
    mut bytes: Vec<u8>,
) -> Result<(String, String), Box<dyn Error>> {
    let event: (String, String) = match type_byte {
        0x00 => (
            META_EVENT_SEQUENCE_NUMBER.to_string(),
            format!(
                "{}",
                take_first_two_bytes_as_unsigned_integer(&mut bytes, Endian::Big)?
            ),
        ),
        0x01 => (
            META_EVENT_TEXT.to_string(),
            String::from_utf8_lossy(bytes.as_slice()).to_string(),
        ),
        0x02 => (
            META_EVENT_COPYRIGHT_NOTICE.to_string(),
            format_bytes_as_string(bytes)?,
        ),
        0x03 => (
            META_EVENT_TRACK_NAME.to_string(),
            format_bytes_as_string(bytes)?,
        ),
        0x04 => (
            META_EVENT_INSTRUMENT_NAME.to_string(),
            format_bytes_as_string(bytes)?,
        ),
        0x05 => (
            META_EVENT_LYRICS.to_string(),
            format_bytes_as_string(bytes)?,
        ),
        0x06 => (
            META_EVENT_MARKER.to_string(),
            format_bytes_as_string(bytes)?,
        ),
        0x07 => (
            META_EVENT_CUE_POINT.to_string(),
            format_bytes_as_string(bytes)?,
        ),
        0x09 => (
            META_EVENT_DEVICE_NAME.to_string(),
            format_bytes_as_string(bytes)?,
        ),
        0x20 => (
            META_EVENT_CHANNEL_PREFIX.to_string(),
            format!(
                "{}",
                take_first_byte_as_unsigned_integer(&mut bytes, Endian::Big)?
            ),
        ),
        0x21 => (
            META_EVENT_MIDI_PORT.to_string(),
            format!(
                "{}",
                take_first_byte_as_unsigned_integer(&mut bytes, Endian::Big)?
            ),
        ),
        0x51 => (
            META_EVENT_SET_TEMPO.to_string(),
            get_bpm_from_bytes(&mut bytes),
        ),
        0x54 => (
            META_EVENT_SMPTE_OFFSET.to_string(),
            format_smpte_offset(&mut bytes, Endian::Big)?,
        ),
        0x58 => (
            META_EVENT_TIME_SIGNATURE.to_string(),
            get_time_signature_from_bytes(&mut bytes),
        ),
        0x59 => (
            META_EVENT_KEY_SIGNATURE.to_string(),
            get_key_from_number_of_flats(
                take_first_byte_as_signed_integer(&mut bytes, Endian::Big)?,
                take_first_byte_as_unsigned_integer(&mut bytes, Endian::Big)?.is_zero(),
            ),
        ),
        0x7F => (
            META_EVENT_SEQUENCER_SPECIFIC.to_string(),
            get_sequencer_specific_field_from_bytes(&mut bytes),
        ),
        _ => {
            let unknown_as_string = format_bytes_as_string(bytes)?;
            let value = format!("Byte: 0x{:02X?} - {}", type_byte, unknown_as_string);
            (META_EVENT_UNKNOWN.to_string(), value)
        }
    };

    Ok(event)
}

fn get_bpm_from_bytes(bytes: &mut Vec<u8>) -> String {
    const MAX_INTEGER_BYTES: usize = 4;
    let mut bpm_bytes: Vec<u8> = vec![0x00];
    bpm_bytes.extend(bytes.as_slice());

    let mut byte_array: [u8; MAX_INTEGER_BYTES] = [0; MAX_INTEGER_BYTES];
    byte_array.copy_from_slice(bpm_bytes.as_slice());

    let microseconds_per_beat = u32::from_be_bytes(byte_array);

    if microseconds_per_beat == 0 {
        return "0".to_string();
    }

    let bpm = MICROSECONDS_PER_MINUTE / microseconds_per_beat;
    bpm.to_string()
}

pub fn get_division_output_values_from_raw_division_bytes(mut division_bytes: Vec<u8>) -> Division {
    let first_byte = take_first_byte(&mut division_bytes).unwrap_or(0x00);
    let second_byte = take_first_byte(&mut division_bytes).unwrap_or(0x00);

    let mut division: Division = Default::default();

    let check_byte = first_byte;
    if (check_byte >> 7) == 1 {
        let timecode_byte = first_byte - FIRST_BIT_AS_U8_DECIMAL_INTEGER;
        let timecode = i8::from_be_bytes([timecode_byte]).abs();
        division.timecode = timecode as u8;
        division.ticks_per_frame = u8::from_be_bytes([second_byte]);
    } else {
        division.ppqn = u16::from_be_bytes([first_byte, second_byte]);
    }

    division
}

pub fn get_file_format_from_raw_format_number(format_number: u16) -> String {
    match format_number {
        0 => MIDI_FORMAT_TYPE_ZERO.to_string(),
        1 => MIDI_FORMAT_TYPE_ONE.to_string(),
        2 => MIDI_FORMAT_TYPE_TWO.to_string(),
        _ => String::new(),
    }
}

fn get_variable_length_unsigned_integer_from_track_data(bytes: &mut Vec<u8>) -> u32 {
    const MAX_INTEGER_BYTES: usize = 4;
    let mut integer_bytes: Vec<u8> = Vec::new();

    for _ in 0..MAX_INTEGER_BYTES {
        let current_byte = take_first_byte(bytes).unwrap_or_default();
        integer_bytes.push(current_byte);

        if (current_byte >> 7) == 0 {
            break;
        }
    }

    let mut buffer: [u8; MAX_INTEGER_BYTES] = [0; MAX_INTEGER_BYTES];
    let difference = buffer.len() - integer_bytes.len();
    let mut output_bytes: Vec<u8> = vec![0; difference];
    output_bytes.extend(&integer_bytes);

    buffer.copy_from_slice(output_bytes.as_slice());
    u32::from_be_bytes(buffer)
}

fn get_sequencer_specific_field_from_bytes(bytes: &mut Vec<u8>) -> String {
    let manufacturer_id =
        take_first_byte_as_unsigned_integer(bytes, Endian::Big).unwrap_or_default();
    let remaining_bytes = format_bytes_as_string_of_bytes(bytes);
    format!("{} : {}", manufacturer_id, remaining_bytes)
}

fn get_time_signature_from_bytes(bytes: &mut Vec<u8>) -> String {
    let numerator = take_first_byte_as_unsigned_integer(bytes, Endian::Big).unwrap_or(4);
    let denominator = take_first_byte_as_unsigned_integer(bytes, Endian::Big).unwrap_or(2);
    let clock_ticks_per_metronome_click =
        take_first_byte_as_unsigned_integer(bytes, Endian::Big).unwrap_or_default();
    let thirtysecond_notes_per_beat =
        take_first_byte_as_unsigned_integer(bytes, Endian::Big).unwrap_or_default();

    format!(
        "{}/{}\n{}: {}\n{}: {}",
        numerator,
        denominator << 1,
        METRONOME_CLICK_MESSAGE,
        clock_ticks_per_metronome_click,
        THIRTYSECOND_NOTES_PER_BEAT_MESSAGE,
        thirtysecond_notes_per_beat
    )
}

fn get_key_from_number_of_flats(flats: i8, is_major: bool) -> String {
    let major_flats: HashMap<i8, &str> = HashMap::from(MAJOR_KEY_FLATS);
    let minor_flats: HashMap<i8, &str> = HashMap::from(MINOR_KEY_FLATS);

    let key_center: (&str, &str) = match is_major {
        true => (major_flats[&flats], MAJOR_SCALE_TITLE),
        false => (minor_flats[&flats], MINOR_SCALE_TITLE),
    };

    format!("{} {}", key_center.0, key_center.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_f_minor_key_from_4_flats() {
        let correct_result = "F minor".to_string();
        let result = get_key_from_number_of_flats(4, false);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn returns_b_major_key_from_7_flats() {
        let correct_result = "B Major".to_string();
        let result = get_key_from_number_of_flats(7, true);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_the_correct_time_signature_from_valid_bytes() {
        let mut test_bytes: Vec<u8> = vec![0x07, 0x08, 0x09, 0x10];
        let correct_result =
            "7/16\nMidi Clock Ticks Per Metronome Click: 9\n32nd Notes Per Beat: 16".to_string();
        let result = get_time_signature_from_bytes(&mut test_bytes);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_the_correct_sequencer_specific_field_from_valid_bytes() {
        let mut test_bytes: Vec<u8> = vec![0x07, 0x43, 0x4F, 0x52, 0x52, 0x45, 0x43, 0x54];
        let correct_result = "7 :  43 4f 52 52 45 43 54".to_string();
        let result = get_sequencer_specific_field_from_bytes(&mut test_bytes);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_the_correct_file_format_string_from_valid_format_numbers() {
        let format_numbers: [(u16, &str); 3] = [
            (0, MIDI_FORMAT_TYPE_ZERO),
            (1, MIDI_FORMAT_TYPE_ONE),
            (2, MIDI_FORMAT_TYPE_TWO),
        ];
        for number in format_numbers {
            let result = get_file_format_from_raw_format_number(number.0);
            assert_eq!(result, number.1);
        }
    }

    #[test]
    fn return_an_empty_file_format_string_from_invalid_format_number() {
        let test_number = u16::MAX;
        let correct_result = "".to_string();
        let result = get_file_format_from_raw_format_number(test_number);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_correct_variable_length_integer_from_no_bytes() {
        let mut test_bytes = vec![];
        let correct_result = 0;
        let result = get_variable_length_unsigned_integer_from_track_data(&mut test_bytes);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_correct_variable_length_integer_from_1_valid_byte() {
        let mut test_bytes = vec![7];
        let correct_result = 7;
        let result = get_variable_length_unsigned_integer_from_track_data(&mut test_bytes);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_correct_variable_length_integer_when_no_bytes_have_list_byte_bit_set() {
        let mut test_bytes = vec![135, 129, 140, 200, 129, 243];
        let correct_result = 2273414344;
        let result = get_variable_length_unsigned_integer_from_track_data(&mut test_bytes);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_correct_ppqn_division_from_valid_bytes() {
        let test_bytes: Vec<u8> = vec![0x00, 0x18];
        let correct_result = Division {
            ppqn: 24,
            timecode: 0,
            ticks_per_frame: 0,
        };
        let result = get_division_output_values_from_raw_division_bytes(test_bytes);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_correct_timecode_division_from_valid_bytes() {
        let test_bytes: Vec<u8> = vec![0xF0, 0x18];
        let correct_result = Division {
            ppqn: 0,
            timecode: 112,
            ticks_per_frame: 24,
        };
        let result = get_division_output_values_from_raw_division_bytes(test_bytes);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_correct_bpm_from_bytes() {
        let mut test_bytes: Vec<u8> = vec![0x05, 0x24, 0xB6];
        let correct_result = "178".to_string();
        let result = get_bpm_from_bytes(&mut test_bytes);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_zero_bpm_when_passed_zero_value_bytes() {
        let mut test_bytes: Vec<u8> = vec![0x00, 0x00, 0x00];
        let correct_result = "0".to_string();
        let result = get_bpm_from_bytes(&mut test_bytes);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_correct_byte_vec_from_valid_bytes() {
        let mut test_bytes: Vec<u8> = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
        ];
        let correct_result = vec![0x01, 0x02, 0x03, 0x04];
        let result = get_track_data_from_midi_data(&mut test_bytes).unwrap();
        assert_eq!(result, correct_result);
    }
}
