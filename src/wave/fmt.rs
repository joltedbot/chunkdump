use crate::bytes::{
    read_bytes_from_file, read_four_byte_integer_from_file, read_two_byte_integer_from_file,
};
use std::error::Error;
use std::fs::File;

const FORMAT_CHUNK_SIZE_IF_NO_EXTENSION: u32 = 16;
const SUBFORMAT_FIELD_LENGTH_IN_BYTES: usize = 16;

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

pub fn read_fmt_chunk_fields(wave_file: &mut File) -> Result<FmtFields, Box<dyn Error>> {
    let chunk_size = read_four_byte_integer_from_file(wave_file)?;

    let format_code = read_two_byte_integer_from_file(wave_file)?;
    let number_of_channels = read_two_byte_integer_from_file(wave_file)?;
    let samples_per_second = read_four_byte_integer_from_file(wave_file)?;
    let average_data_rate = read_four_byte_integer_from_file(wave_file)?;
    let data_block_size = read_two_byte_integer_from_file(wave_file)?;
    let bits_per_sample = read_two_byte_integer_from_file(wave_file)?;

    let mut extension_size: u16 = Default::default();
    let mut valid_bits_per_sample: u16 = Default::default();
    let mut speaker_position_mask: u32 = Default::default();
    let mut subformat_guid: Vec<u8> = vec![];

    if chunk_size > FORMAT_CHUNK_SIZE_IF_NO_EXTENSION {
        extension_size = read_two_byte_integer_from_file(wave_file)?;
    }

    if extension_size > 0 {
        valid_bits_per_sample = read_two_byte_integer_from_file(wave_file)?;
        speaker_position_mask = read_four_byte_integer_from_file(wave_file)?;
        subformat_guid = read_bytes_from_file(wave_file, SUBFORMAT_FIELD_LENGTH_IN_BYTES)?;
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
