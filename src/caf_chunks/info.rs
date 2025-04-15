use crate::byte_arrays::{take_first_four_bytes_as_unsigned_integer, Endian};
use crate::formating::{set_key_value_pair_spacers, KeyValuePair as Entry};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/caf_chunks/info.tmpl");

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let number_of_entries =
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;

    let information_entries: Vec<Entry> = get_entries_from_bytes(chunk_data)?;

    let output_values: Value = upon::value! {
            number_of_entries: number_of_entries,
            information_entries: information_entries,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}

fn get_entries_from_bytes(chunk_data: Vec<u8>) -> Result<Vec<Entry>, Box<dyn Error>> {
    let keys_and_values: Vec<&[u8]> = chunk_data
        .split(|byte| *byte == 0x00)
        .filter(|value| !value.is_empty())
        .collect();
    let mut entries: Vec<Entry> = Vec::new();

    keys_and_values.chunks(2).for_each(|chunk| {
        if chunk.len() == 2 {
            entries.push(Entry {
                key: String::from_utf8_lossy(chunk[0]).to_string(),
                spacer: " ".to_string(),
                value: String::from_utf8_lossy(chunk[1]).to_string(),
            })
        }
    });

    set_key_value_pair_spacers(&mut entries);

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formating::KeyValuePair;

    #[test]
    fn return_correct_info_entries_from_valid_bytes_with_two_entry() {
        let chunk_data = vec![
            0x00, 0x00, 0x00, 0x02, b'k', b'e', b'y', 0x00, b'v', b'a', b'l', 0x00, b'k', b'e',
            b'y', b' ', b't', b'w', b'o', 0x00, b'v', b'a', b'l', b' ', b't', b'w', b'o', 0x00,
        ];
        let correct_result_text =  "\n---------------------------------\nInformation Layout Chunk Details:\n---------------------------------\nNumber of Information Entries:   2\nkey:     val\nkey two: val two\n";
        let correct_result = OutputEntry {
            section: Section::Optional,
            text: correct_result_text.to_string(),
        };
        let result = get_metadata(chunk_data).unwrap();
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_correct_info_entries_when_there_are_no_entries() {
        let chunk_data = vec![0x00, 0x00, 0x00, 0x00];
        let correct_result_text =  "\n---------------------------------\nInformation Layout Chunk Details:\n---------------------------------\nNumber of Information Entries:   0\n";
        let correct_result = OutputEntry {
            section: Section::Optional,
            text: correct_result_text.to_string(),
        };
        let result = get_metadata(chunk_data).unwrap();
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_two_correct_entries_from_valid_entry_bytes() {
        let chunk_data = vec![
            b'k', b'e', b'y', 0x00, b'v', b'a', b'l', 0x00, b'k', b'e', b'y', b' ', b't', b'w',
            b'o', 0x00, b'v', b'a', b'l', b' ', b't', b'w', b'o', 0x00,
        ];
        let correct_result = vec![
            KeyValuePair {
                key: "key".to_string(),
                spacer: "    ".to_string(),
                value: "val".to_string(),
            },
            KeyValuePair {
                key: "key two".to_string(),
                spacer: "".to_string(),
                value: "val two".to_string(),
            },
        ];
        let result = get_entries_from_bytes(chunk_data).unwrap();
        assert_eq!(result, correct_result);
    }
}
