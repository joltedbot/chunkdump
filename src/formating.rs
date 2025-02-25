use crate::errors::LocalError;
use byte_unit::{Byte, UnitType};
use chrono::DateTime;

const NOTE_NAMES_WITHOUT_OCTAVES: [&str; 12] = [
    "C", "C#/Db", "D", "D#/Eb", "E", "F", "F#/Gb", "G", "G#/Ab", "A", "A#/Bb", "B",
];
const BAD_TIMESTAMP_MESSAGE: &str = "Unexpected bad timestamp format";
const MAC_HFS_FORMAT_TIMESTAMP_OFFSET: u32 = 2082844800;

pub fn format_file_size_as_string(file_size_in_bytes: u64) -> String {
    format!(
        "{:#.2}",
        Byte::from_u64(file_size_in_bytes).get_appropriate_unit(UnitType::Binary)
    )
}

pub fn format_bytes_as_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .fold("".to_string(), |umid: String, byte| format!("{} {:02x?}", umid, byte))
}

pub fn format_mac_hfs_timestamp_as_date_time_string(timestamp: u32) -> Result<String, LocalError> {
    if timestamp < MAC_HFS_FORMAT_TIMESTAMP_OFFSET {
        return Err(LocalError::HFSTimestampTooSmall);
    }

    let date = match DateTime::from_timestamp((timestamp - MAC_HFS_FORMAT_TIMESTAMP_OFFSET) as i64, 0) {
        Some(ts) => ts.to_string(),
        None => BAD_TIMESTAMP_MESSAGE.to_string(),
    };

    Ok(date)
}

pub fn format_midi_note_number_as_note_name(midi_note_number: u32) -> String {
    let note_offset_from_c: usize = midi_note_number as usize % 12;
    let note_name = NOTE_NAMES_WITHOUT_OCTAVES[note_offset_from_c].to_string();
    let note_octave = ((midi_note_number as f32 - note_offset_from_c as f32) / 12.0) - 2.0;
    format!("{}{}", note_name, note_octave)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_correct_human_readable_file_size_format_from_millions_of_bytes() {
        let megabyte_scale_size_in_bytes = 123456789;
        let correct_formated_size = "117.74 MiB";
        let formated_size = format_file_size_as_string(megabyte_scale_size_in_bytes);
        assert_eq!(formated_size, correct_formated_size);
    }

    #[test]
    fn correct_string_is_returned_from_bytes() {
        let input_byte_array_in_decimal: &[u8] = &[1, 2, 58, 75];
        let correct_result_string: String = " 01 02 3a 4b".to_string();
        assert_eq!(
            format_bytes_as_string(input_byte_array_in_decimal),
            correct_result_string
        );
    }

    #[test]
    fn return_correct_data_time_string_from_mac_hfs_timestamp() {
        let correct_date_time_string = "2001-09-09 01:46:40 UTC";
        let result = format_mac_hfs_timestamp_as_date_time_string(3082844800).unwrap();
        assert_eq!(result, correct_date_time_string);
    }

    #[test]
    fn return_error_if_mac_hfs_timestamp_is_below_the_valid_range() {
        let result = format_mac_hfs_timestamp_as_date_time_string(1).err().unwrap();
        assert_eq!(result, LocalError::HFSTimestampTooSmall);
    }

    #[test]
    fn return_note_c_minus_2_when_midi_note_number_is_0() {
        assert_eq!(format_midi_note_number_as_note_name(0), "C-2");
    }

    #[test]
    fn return_note_gb_3_when_midi_note_number_is_66() {
        assert_eq!(format_midi_note_number_as_note_name(66), "F#/Gb3");
    }

    #[test]
    fn return_note_g8_when_midi_note_number_is_127() {
        assert_eq!(format_midi_note_number_as_note_name(127), "G8");
    }
}
