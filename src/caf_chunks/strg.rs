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
