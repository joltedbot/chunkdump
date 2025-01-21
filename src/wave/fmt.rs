use super::{take_first_four_bytes_as_integer, take_first_two_bytes_as_integer};
use crate::fileio::{read_bytes_from_file, read_four_byte_integer_from_file};
use std::error::Error;
use std::fs::File;

const FORMAT_CHUNK_SIZE_IF_NO_EXTENSION: u32 = 16;

#[derive(Debug, Clone, Default)]
pub struct FmtFields {
    pub format_code: u16,
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

    let format_code = take_first_two_bytes_as_integer(&mut fmt_data)?;
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
