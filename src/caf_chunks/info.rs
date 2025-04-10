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
    let keys_and_values: Vec<&[u8]> = chunk_data.split(|byte| *byte == 0x00).collect();
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
