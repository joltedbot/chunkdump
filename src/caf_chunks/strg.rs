use crate::byte_arrays::{
    take_first_eight_bytes_as_signed_integer, take_first_four_bytes_as_unsigned_integer, Endian,
};
use crate::formating::{set_key_value_pair_spacers, KeyValuePair as StringEntry};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/caf_chunks/strg.tmpl");

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let number_of_entries =
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;

    let string_ids: Vec<(u32, i64)> =
        get_string_ids_from_bytes(&mut chunk_data, number_of_entries as usize)?;

    let string_entries: Vec<StringEntry> = get_string_entries_from_bytes(chunk_data, string_ids)?;

    let output_values: Value = upon::value! {
        string_entries: string_entries,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}

fn get_string_entries_from_bytes(
    chunk_data: Vec<u8>,
    string_ids: Vec<(u32, i64)>,
) -> Result<Vec<StringEntry>, Box<dyn Error>> {
    let values: Vec<&[u8]> = chunk_data
        .split(|byte| *byte == 0x00)
        .filter(|value| !value.is_empty())
        .collect();

    let mut entries: Vec<StringEntry> = Vec::new();

    for i in 0..values.len() {
        entries.push(StringEntry {
            key: format!("{}", string_ids[i].0),
            spacer: " ".to_string(),
            value: String::from_utf8_lossy(values[i]).to_string(),
        })
    }

    set_key_value_pair_spacers(&mut entries);

    Ok(entries)
}

fn get_string_ids_from_bytes(
    chunk_data: &mut Vec<u8>,
    number_of_entries: usize,
) -> Result<Vec<(u32, i64)>, Box<dyn Error>> {
    let mut string_ids: Vec<(u32, i64)> = Vec::new();

    for _ in 0..number_of_entries {
        let string_id = take_first_four_bytes_as_unsigned_integer(chunk_data, Endian::Big)?;
        let string_start_byte_offset =
            take_first_eight_bytes_as_signed_integer(chunk_data, Endian::Big)?;

        string_ids.push((string_id, string_start_byte_offset));
    }

    Ok(string_ids)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn return_correct_string_id_and_offset_values_from_valid_entry_bytes() {
        // Create test data with 2 entries
        let mut test_bytes = vec![
            // Entry 1: ID = 1, offset = 10
            0x00, 0x00, 0x00, 0x01, // string ID (4 bytes)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0A, // offset (8 bytes)
            // Entry 2: ID = 2, offset = 20
            0x00, 0x00, 0x00, 0x02, // string ID (4 bytes)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, // offset (8 bytes)
        ];
        let number_of_entries = 2;
        let correct_result = vec![(1, 10), (2, 20)];
        let result = get_string_ids_from_bytes(&mut test_bytes, number_of_entries).unwrap();

        assert_eq!(result, correct_result);
    }

    #[test]
    fn get_string_entries_from_bytes_returns_correct_entries() {
        let test_bytes = "Hello\0World\0".as_bytes().to_vec();
        let string_ids = vec![(1, 0), (2, 6)];

        let result = get_string_entries_from_bytes(test_bytes, string_ids).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].key, "1");
        assert_eq!(result[0].value, "Hello");
        assert_eq!(result[1].key, "2");
        assert_eq!(result[1].value, "World");
    }

    #[test]
    fn get_metadata_returns_correct_output_entry() {
        let test_bytes = vec![
            0x00, 0x00, 0x00, 0x02, // number of entries (2)
            // First entry
            0x00, 0x00, 0x00, 0x01, // string ID (1)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // offset (0)
            // Second entry
            0x00, 0x00, 0x00, 0x02, // string ID (2)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, // offset (6)
            // String data
            b'H', b'e', b'l', b'l', b'o', 0x00, // "Hello"
            b'W', b'o', b'r', b'l', b'd', 0x00, // "World"
        ];

        let result = get_metadata(test_bytes).unwrap();

        assert_eq!(result.section, Section::Optional);
        assert!(result.text.contains("Hello"));
        assert!(result.text.contains("World"));
    }
}
