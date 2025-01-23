use super::{take_first_four_bytes_as_integer, take_first_two_bytes_as_integer};
use crate::fileio::{read_bytes_from_file, read_four_byte_integer_from_file};
use std::error::Error;
use std::fs::File;

const FORMAT_CHUNK_SIZE_IF_NO_EXTENSION: u32 = 16;
const WAVE_FORMAT_PCM_ID: u16 = 1;
const WAVE_FORMAT_PCM_NAME: &str = "PCM";
const WAVE_FORMAT_IEEE_FLOAT_ID: u16 = 3;
const WAVE_FORMAT_IEEE_FLOAT_NAME: &str = "IEEE float";
const WAVE_FORMAT_ALAW_ID: u16 = 6;
const WAVE_FORMAT_ALAW_NAME: &str = "8-bit ITU-T G.711 A-law";
const WAVE_FORMAT_MULAW_ID: u16 = 7;
const WAVE_FORMAT_MULAW_NAME: &str = "8-bit ITU-T G.711 µ-law";
const WAVE_FORMAT_EXTENSIBLE_ID: u16 = 65279;
const WAVE_FORMAT_EXTENSIBLE_NAME: &str = "Determined by SubFormat";
const WAVE_FORMAT_UNKNOWN: &str = "Unknown Format ID: ";

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

pub fn read_fmt_chunk(wave_file: &mut File) -> Result<FmtFields, Box<dyn Error>> {
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

    Ok(FmtFields {
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

fn get_format_name_from_format_id(format_id: u16) -> String {
    match format_id {
        WAVE_FORMAT_PCM_ID => WAVE_FORMAT_PCM_NAME.to_string(),
        WAVE_FORMAT_IEEE_FLOAT_ID => WAVE_FORMAT_IEEE_FLOAT_NAME.to_string(),
        WAVE_FORMAT_ALAW_ID => WAVE_FORMAT_ALAW_NAME.to_string(),
        WAVE_FORMAT_MULAW_ID => WAVE_FORMAT_MULAW_NAME.to_string(),
        WAVE_FORMAT_EXTENSIBLE_ID => WAVE_FORMAT_EXTENSIBLE_NAME.to_string(),
        _ => format!("{} {}", WAVE_FORMAT_UNKNOWN, format_id),
    }
}
