#![allow(dead_code)]

use crate::byteio::{
    take_first_eight_bytes_as_integer, take_first_number_of_bytes,
    take_first_number_of_bytes_as_string, take_first_two_bytes_as_integer,
    take_first_two_bytes_as_signed_integer,
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
    pub time_reference: u64,
    pub version: u16,
    pub umid: Vec<u8>,
    pub loudness_value: i16,
    pub loudness_range: i16,
    pub max_true_peak_level: i16,
    pub max_momentary_loudness: i16,
    pub max_short_term_loudness: i16,
    pub reserved: String,
    pub coding_history: String,
}

impl BextData {
    pub fn new(wave_file: &mut File) -> Result<Self, Box<dyn Error>> {
        let chunk_size = read_four_byte_integer_from_file(wave_file)?;
        let mut bext_data = read_bytes_from_file(wave_file, chunk_size as usize)?;

        Ok(Self {
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
            time_reference: take_first_eight_bytes_as_integer(&mut bext_data)?,
            version: take_first_two_bytes_as_integer(&mut bext_data)?,
            umid: take_first_number_of_bytes(&mut bext_data, UMID_LENGTH_IN_BYTES)?,
            loudness_value: take_first_two_bytes_as_signed_integer(&mut bext_data)?,
            loudness_range: take_first_two_bytes_as_signed_integer(&mut bext_data)?,
            max_true_peak_level: take_first_two_bytes_as_signed_integer(&mut bext_data)?,
            max_momentary_loudness: take_first_two_bytes_as_signed_integer(&mut bext_data)?,
            max_short_term_loudness: take_first_two_bytes_as_signed_integer(&mut bext_data)?,
            reserved: take_first_number_of_bytes_as_string(
                &mut bext_data,
                RESERVED_FIELD_LENGTH_IN_BYTES,
            )?,
            coding_history: get_coding_history_from_bytes(bext_data)?,
        })
    }

    pub fn get_metadata_output(&self) -> Vec<String> {
        let mut bext_data: Vec<String> = vec![];

        bext_data.push(
            "\n-------------\nBroadcast Wave Format (BWF) bext Chunk Details:\n-------------"
                .to_string(),
        );

        bext_data.push(format!("Description: {}", self.description));
        bext_data.push(format!("Originator: {}", self.originator));
        bext_data.push(format!(
            "Originator Reference: {}",
            self.originator_reference
        ));
        bext_data.push(format!("Originator Date: {}", self.originator_date));
        bext_data.push(format!("Originator Time {}", self.originator_time));
        bext_data.push(format!(
            "Time Reference: {} First Samples Since Midnight",
            self.time_reference
        ));
        bext_data.push(format!("Version: {}", self.version));
        bext_data.push(format!("UMID: {}", format_umid(self.umid.clone())));
        bext_data.push(format!(
            "Loudness Value: {} LUFS",
            self.loudness_value / 100
        ));
        bext_data.push(format!(
            "loudness Range: {} LUFS",
            self.loudness_range / 100
        ));
        bext_data.push(format!(
            "Max True Peak Level: {} dbTP",
            self.max_true_peak_level / 100
        ));
        bext_data.push(format!(
            "Max Momentary loudness: {} LUFS",
            self.max_momentary_loudness / 100
        ));
        bext_data.push(format!(
            "Max Short Term Loudness: {} LUFS",
            self.max_short_term_loudness / 100
        ));
        bext_data.push(format!("Reserved: {}", self.reserved));
        bext_data.push(format!("Coding History: {}", self.coding_history));

        bext_data
    }
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

fn format_umid(umid_data: Vec<u8>) -> String {
    let umid_string: String = umid_data.iter().fold("".to_string(), |umid: String, byte| {
        format!("{}{:02X}", umid, byte)
    });

    umid_string
}
