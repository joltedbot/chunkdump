use crate::byte_arrays::{take_first_byte, take_first_byte_as_signed_integer, Endian};
use crate::errors::LocalError;
use byte_unit::{Byte, UnitType};
use chrono::DateTime;
use serde::Serialize;
use std::error::Error;
use std::path::Path;

const NOTE_NAMES_WITHOUT_OCTAVES: [&str; 12] = [
    "C", "C#/Db", "D", "D#/Eb", "E", "F", "F#/Gb", "G", "G#/Ab", "A", "A#/Bb", "B",
];
const BAD_TIMESTAMP_MESSAGE: &str = "Unexpected bad timestamp format";
const MAC_HFS_FORMAT_TIMESTAMP_OFFSET: u32 = 2082844800;
const DEFAULT_SPACER_LENGTH: usize = 5;

#[derive(Debug, PartialEq, Serialize)]
pub struct KeyValuePair {
    pub key: String,
    pub spacer: String,
    pub value: String,
}

pub fn format_file_size_as_string(file_size_in_bytes: u64) -> String {
    format!(
        "{:#.2}",
        Byte::from_u64(file_size_in_bytes).get_appropriate_unit(UnitType::Binary)
    )
}

pub fn format_bytes_as_string_of_bytes(bytes: &[u8]) -> String {
    let output_string = bytes.iter().fold("".to_string(), |umid: String, byte| {
        format!("{} {:02x?}", umid, byte)
    });

    output_string.trim().to_string()
}

pub fn format_bytes_as_string(byte_data: Vec<u8>) -> Result<String, LocalError> {
    let cleaned_bytes: Vec<u8> = byte_data
        .into_iter()
        .filter(|byte| byte.is_ascii() && *byte != 0x00 && !byte.is_ascii_control())
        .collect();

    Ok(String::from_utf8_lossy(cleaned_bytes.as_slice()).to_string())
}

pub fn format_mac_hfs_timestamp_as_date_time_string(timestamp: u32) -> Result<String, LocalError> {
    if timestamp < MAC_HFS_FORMAT_TIMESTAMP_OFFSET {
        return Err(LocalError::HFSTimestampTooSmall);
    }

    let date =
        match DateTime::from_timestamp((timestamp - MAC_HFS_FORMAT_TIMESTAMP_OFFSET) as i64, 0) {
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

pub fn canonicalize_file_path(file_path: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new(file_path).canonicalize()?;

    let canonical_path = match path.to_str() {
        Some(path) => path.to_string(),
        None => return Err(Box::new(LocalError::InvalidPath(file_path.to_string()))),
    };

    Ok(canonical_path)
}

pub fn get_file_name_from_file_path(file_path: &str) -> Result<String, LocalError> {
    let path = Path::new(file_path);

    let file_name = match path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => return Err(LocalError::InvalidFileName),
    };

    Ok(file_name)
}

pub fn format_smpte_offset(
    smpte_offset_bytes: &mut Vec<u8>,
    endianness: Endian,
) -> Result<String, LocalError> {
    if endianness == Endian::Little {
        let hours = take_first_byte_as_signed_integer(smpte_offset_bytes, Endian::Little)?;
        let minutes = take_first_byte(smpte_offset_bytes)?;
        let seconds = take_first_byte(smpte_offset_bytes)?;
        let samples = take_first_byte(smpte_offset_bytes)?;
        Ok(format!(
            "{}h:{}m:{}s & {} samples",
            hours, minutes, seconds, samples
        ))
    } else {
        let samples = take_first_byte(smpte_offset_bytes)?;
        let seconds = take_first_byte(smpte_offset_bytes)?;
        let minutes = take_first_byte(smpte_offset_bytes)?;
        let hours = take_first_byte_as_signed_integer(smpte_offset_bytes, Endian::Big)?;
        Ok(format!(
            "{}h:{}m:{}s & {} samples",
            hours, minutes, seconds, samples
        ))
    }
}

pub fn set_key_value_pair_spacers(key_value_pairs: &mut Vec<KeyValuePair>) {
    let longest_key = match key_value_pairs.iter().max_by_key(|tag| tag.key.len()) {
        Some(tag) => tag.key.len(),
        None => DEFAULT_SPACER_LENGTH,
    };

    for kv in key_value_pairs {
        if longest_key > kv.key.len() {
            kv.spacer = " ".repeat(longest_key - kv.key.len());
        } else {
            kv.spacer = String::new();
        }
    }
}

pub fn add_one_if_byte_size_is_odd(mut byte_size: u32) -> u32 {
    if byte_size % 2 > 0 {
        byte_size += 1;
    }

    byte_size
}

pub fn format_bit_as_bool_string(bit: u8) -> String {
    if bit == 1 {
        "True".to_string()
    } else {
        "False".to_string()
    }
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
    fn correct_string_of_bytes_is_returned_from_bytes() {
        let input_byte_array_in_decimal: &[u8] = &[1, 2, 58, 75];
        let correct_result_string: String = "01 02 3a 4b".to_string();
        let result_string: String = format_bytes_as_string_of_bytes(input_byte_array_in_decimal);
        assert_eq!(result_string, correct_result_string);
    }

    #[test]
    fn correct_string_is_returned_from_bytes() {
        let input_byte_array_in_decimal: Vec<u8> = vec![0x43, 0x4F, 0x52, 0x52, 0x45, 0x43, 0x54];
        let correct_result_string: String = "CORRECT".to_string();
        let result_string: String = format_bytes_as_string(input_byte_array_in_decimal).unwrap();
        assert_eq!(result_string, correct_result_string);
    }

    #[test]
    fn return_correct_data_time_string_from_mac_hfs_timestamp() {
        let correct_date_time_string = "2001-09-09 01:46:40 UTC";
        let result = format_mac_hfs_timestamp_as_date_time_string(3082844800).unwrap();
        assert_eq!(result, correct_date_time_string);
    }

    #[test]
    fn return_error_if_mac_hfs_timestamp_is_below_the_valid_range() {
        let result = format_mac_hfs_timestamp_as_date_time_string(1)
            .err()
            .unwrap();
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

    #[test]
    fn return_correct_canonicalize_path_when_given_path_is_valid() {
        let correct_result = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            + "/src/main.rs";
        let result = canonicalize_file_path("./src/main.rs").unwrap();

        assert_eq!(result, correct_result);
    }

    #[test]
    fn canonicalize_file_path_throws_error_when_given_path_is_invalid() {
        let invalid_test_path = "/not/a/real/path".to_string();
        let result = canonicalize_file_path(&invalid_test_path);

        assert!(result.is_err());
    }

    #[test]
    fn get_file_name_from_file_path_returns_correct_result() {
        let result = get_file_name_from_file_path("/test/path/filename.wav").unwrap();
        assert_eq!(result, "filename.wav")
    }

    #[test]
    fn errors_when_geting_filename_from_filepath_if_path_is_invalid() {
        let result = get_file_name_from_file_path("/");
        assert_eq!(result.unwrap_err(), LocalError::InvalidFileName);
    }

    #[test]
    fn returns_the_correctly_format_le_smpte_offset_bytes() {
        let mut test_manufacturer_id_bytes = vec![0x01, 0x02, 0x03, 0x04];
        let id = format_smpte_offset(&mut test_manufacturer_id_bytes, Endian::Little).unwrap();
        assert_eq!(id, "1h:2m:3s & 4 samples");
    }

    #[test]
    fn returns_the_correctly_format_be_smpte_offset_bytes() {
        let mut test_manufacturer_id_bytes = vec![0x04, 0x03, 0x02, 0x01];
        let id = format_smpte_offset(&mut test_manufacturer_id_bytes, Endian::Big).unwrap();
        assert_eq!(id, "1h:2m:3s & 4 samples");
    }

    #[test]
    fn correctly_adds_one_if_byte_size_is_odd() {
        let test_size = 3;
        let correct_size = test_size + 1;

        assert_eq!(add_one_if_byte_size_is_odd(test_size), correct_size);
    }

    #[test]
    fn does_not_add_one_if_byte_size_is_even() {
        let test_size = 4;
        assert_eq!(add_one_if_byte_size_is_odd(test_size), test_size);
    }

    #[test]
    fn correctly_set_tag_spacers() {
        let mut test_tags: Vec<KeyValuePair> = vec![
            KeyValuePair {
                key: "k".to_string(),
                spacer: "".to_string(),
                value: "none".to_string(),
            },
            KeyValuePair {
                key: "ke".to_string(),
                spacer: "".to_string(),
                value: "none".to_string(),
            },
            KeyValuePair {
                key: "key".to_string(),
                spacer: "".to_string(),
                value: "none".to_string(),
            },
            KeyValuePair {
                key: "keyvaluepair".to_string(),
                spacer: "".to_string(),
                value: "none".to_string(),
            },
        ];
        let correct_tags: Vec<KeyValuePair> = vec![
            KeyValuePair {
                key: "k".to_string(),
                spacer: "           ".to_string(),
                value: "none".to_string(),
            },
            KeyValuePair {
                key: "ke".to_string(),
                spacer: "          ".to_string(),
                value: "none".to_string(),
            },
            KeyValuePair {
                key: "key".to_string(),
                spacer: "         ".to_string(),
                value: "none".to_string(),
            },
            KeyValuePair {
                key: "keyvaluepair".to_string(),
                spacer: "".to_string(),
                value: "none".to_string(),
            },
        ];
        set_key_value_pair_spacers(&mut test_tags);
        assert_eq!(test_tags, correct_tags);
    }
}
