#![allow(dead_code)]

use crate::byteio::{take_first_four_bytes_as_integer, take_first_two_bytes_as_integer};
use crate::fileio::{read_bytes_from_file, read_four_byte_integer_from_file};
use std::error::Error;
use std::fs::File;

const FORMAT_CHUNK_SIZE_IF_NO_EXTENSION: u32 = 16;
const PCM_FORMAT_ID: u16 = 1;
const PCM_FORMAT_NAME: &str = "PCM";
const IEEE_FORMAT_FLOAT_ID: u16 = 3;
const IEEE_FORMAT_FLOAT_NAME: &str = "IEEE float";
const ALAW_FORMAT_ID: u16 = 6;
const ALAW_FORMAT_NAME: &str = "8-bit ITU-T G.711 A-law";
const MULAW_FORMAT_ID: u16 = 7;
const MULAW_FORMAT_NAME: &str = "8-bit ITU-T G.711 µ-law";
const EXTENSIBLE_FORMAT_ID: u16 = 65279;
const EXTENSIBLE_FORMAT_NAME: &str = "Determined by SubFormat";
const UNKOWN_FORMAT: &str = "Unknown Format ID: ";
const GUID_LENGTH_IN_BYTES: usize = 16;
const NO_GUID_FOUND_MESSAGE: &str = "N/A";

#[derive(Debug, Clone, Default)]
pub struct FmtFields {
    pub format_code: String,
    pub number_of_channels: u16,
    pub samples_per_second: u32,
    pub average_data_rate: u32,
    pub data_block_size: u16,
    pub bits_per_sample: u16,
    pub valid_bits_per_sample: u16,
    pub speaker_position_mask: u32,
    pub subformat_guid: Vec<u8>,
}

impl FmtFields {
    pub fn new(wave_file: &mut File) -> Result<Self, Box<dyn Error>> {
        let chunk_size = read_four_byte_integer_from_file(wave_file)?;
        let mut fmt_data = read_bytes_from_file(wave_file, chunk_size as usize)?;

        let format_code =
            get_format_name_from_format_id(take_first_two_bytes_as_integer(&mut fmt_data)?);
        let number_of_channels = take_first_two_bytes_as_integer(&mut fmt_data)?;
        let samples_per_second = take_first_four_bytes_as_integer(&mut fmt_data)?;
        let average_data_rate = take_first_four_bytes_as_integer(&mut fmt_data)?;
        let data_block_size = take_first_two_bytes_as_integer(&mut fmt_data)?;
        let bits_per_sample = take_first_two_bytes_as_integer(&mut fmt_data)?;

        let mut extension_size: u16 = Default::default();

        if chunk_size > FORMAT_CHUNK_SIZE_IF_NO_EXTENSION {
            extension_size = take_first_two_bytes_as_integer(&mut fmt_data)?;
        }

        let mut valid_bits_per_sample: u16 = Default::default();
        let mut speaker_position_mask: u32 = Default::default();
        let mut subformat_guid: Vec<u8> = vec![];

        if extension_size > 0 {
            valid_bits_per_sample = take_first_two_bytes_as_integer(&mut fmt_data)?;
            speaker_position_mask = take_first_four_bytes_as_integer(&mut fmt_data)?;
            subformat_guid = fmt_data;
        }

        Ok(Self {
            format_code,
            number_of_channels,
            samples_per_second,
            average_data_rate,
            data_block_size,
            bits_per_sample,
            valid_bits_per_sample,
            speaker_position_mask,
            subformat_guid,
        })
    }

    pub fn get_metadata_output(&self) -> Vec<String> {
        let mut fmt_data: Vec<String> = vec![];

        fmt_data.push("\n-------------\nFMT Chunk Details:\n-------------".to_string());
        fmt_data.push(format!("Format Code: {}", self.format_code));
        fmt_data.push(format!("Number of Channels: {}", self.number_of_channels));
        fmt_data.push(format!(
            "Samples Rate: {:} kHz",
            self.samples_per_second as f64 / 1000.0
        ));
        fmt_data.push(format!("Bit Depth: {} bit", self.bits_per_sample));
        fmt_data.push(format!(
            "Average Data Rate: {} KB/Second",
            self.average_data_rate as f64 / 1000.0
        ));
        fmt_data.push(format!("Data Block Size: {} bytes", self.data_block_size));

        fmt_data.push(format!(
            "Valid Bits per Sample: {} bits",
            self.valid_bits_per_sample
        ));
        fmt_data.push(format!(
            "Speaker Position Mask: {:#?}",
            self.speaker_position_mask
        ));
        fmt_data.push(format!(
            "GUID: {:#?}",
            format_guid(self.subformat_guid.clone())
        ));

        fmt_data
    }
}

fn get_format_name_from_format_id(format_id: u16) -> String {
    match format_id {
        PCM_FORMAT_ID => PCM_FORMAT_NAME.to_string(),
        IEEE_FORMAT_FLOAT_ID => IEEE_FORMAT_FLOAT_NAME.to_string(),
        ALAW_FORMAT_ID => ALAW_FORMAT_NAME.to_string(),
        MULAW_FORMAT_ID => MULAW_FORMAT_NAME.to_string(),
        EXTENSIBLE_FORMAT_ID => EXTENSIBLE_FORMAT_NAME.to_string(),
        _ => format!("{} {}", UNKOWN_FORMAT, format_id),
    }
}

fn format_guid(guid_bytes: Vec<u8>) -> String {
    if guid_bytes.len() != GUID_LENGTH_IN_BYTES {
        return NO_GUID_FOUND_MESSAGE.to_string();
    }

    let formated_guid: Vec<String> = guid_bytes
        .iter()
        .map(|byte| format!("{:X}", byte))
        .collect();

    formated_guid.join("")
}
