#![allow(dead_code)]

use crate::byteio::{
    take_first_four_bytes_as_integer, take_first_number_of_bytes,
    take_first_number_of_bytes_as_string, take_first_two_bytes_as_integer,
};
use crate::fileio::{read_bytes_from_file, read_four_byte_integer_from_file};
use std::error::Error;
use std::fs::File;

const DESCRIPTION_LENGTH_IN_BYTES: usize = 256;
const ORIGINATOR_LENGTH_IN_BYTES: usize = 32;
const ORIGINATOR_REFERENCE_LENGTH_IN_BYTES: usize = 32;
const ORIGINATOR_DATA_LENGTH_IN_BYTES: usize = 10;
const ORIGINATOR_TIME_LENGTH_IN_BYTES: usize = 8;
const UMID_LENGTH_IN_BYTES: usize = 64;
const RESERVED_FIELD_LENGTH_IN_BYTES: usize = 180;

#[derive(Debug, Clone, Default)]
pub struct BextData {
    pub description: String,
    pub originator: String,
    pub originator_reference: String,
    pub originator_date: String,
    pub originator_time: String,
    pub time_reference_low: u32,
    pub time_reference_high: u32,
    pub version: u16,
    pub umid: Vec<u8>,
    pub loudness_value: u16,
    pub loudness_range: u16,
    pub max_true_peak_level: u16,
    pub max_momentary_loudness: u16,
    pub max_short_term_loudness: u16,
    pub reserved: String,
    pub coding_history: String,
}

pub fn read_bext_chunk(wave_file: &mut File) -> Result<BextData, Box<dyn Error>> {
    let chunk_size = read_four_byte_integer_from_file(wave_file)?;
    let mut bext_data = read_bytes_from_file(wave_file, chunk_size as usize)?;

    Ok(BextData {
        description: take_first_number_of_bytes_as_string(
            &mut bext_data,
            DESCRIPTION_LENGTH_IN_BYTES,
        )?,
        originator: take_first_number_of_bytes_as_string(
            &mut bext_data,
            ORIGINATOR_LENGTH_IN_BYTES,
        )?,
        originator_reference: take_first_number_of_bytes_as_string(
            &mut bext_data,
            ORIGINATOR_REFERENCE_LENGTH_IN_BYTES,
        )?,
        originator_date: take_first_number_of_bytes_as_string(
            &mut bext_data,
            ORIGINATOR_DATA_LENGTH_IN_BYTES,
        )?,
        originator_time: take_first_number_of_bytes_as_string(
            &mut bext_data,
            ORIGINATOR_TIME_LENGTH_IN_BYTES,
        )?,
        time_reference_low: take_first_four_bytes_as_integer(&mut bext_data)?,
        time_reference_high: take_first_four_bytes_as_integer(&mut bext_data)?,
        version: take_first_two_bytes_as_integer(&mut bext_data)?,
        umid: take_first_number_of_bytes(&mut bext_data, UMID_LENGTH_IN_BYTES)?,
        loudness_value: take_first_two_bytes_as_integer(&mut bext_data)?,
        loudness_range: take_first_two_bytes_as_integer(&mut bext_data)?,
        max_true_peak_level: take_first_two_bytes_as_integer(&mut bext_data)?,
        max_momentary_loudness: take_first_two_bytes_as_integer(&mut bext_data)?,
        max_short_term_loudness: take_first_two_bytes_as_integer(&mut bext_data)?,
        reserved: take_first_number_of_bytes_as_string(
            &mut bext_data,
            RESERVED_FIELD_LENGTH_IN_BYTES,
        )?,
        coding_history: get_coding_history_from_bytes(bext_data)?,
    })
}

fn get_coding_history_from_bytes(mut bext_data: Vec<u8>) -> Result<String, Box<dyn Error>> {
    let mut coding_history = "".to_string();

    if !bext_data.is_empty() {
        let bext_data_remaining_bytes = bext_data.len();
        coding_history =
            take_first_number_of_bytes_as_string(&mut bext_data, bext_data_remaining_bytes)?;
    }

    Ok(coding_history)
}
