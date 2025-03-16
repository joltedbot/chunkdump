use crate::byte_arrays::{
    take_first_number_of_bytes, take_first_three_bytes_as_32bit_unsigned_integer,
    take_first_two_bytes_as_unsigned_integer, Endian,
};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use upon::Value;

const MD5SUM_LENGTH_IN_BYTES: usize = 16;
const RATE_CHANNEL_AND_BITS_LENGTH_IN_BYTES: usize = 8;

const TEMPLATE_CONTENT: &str = include_str!("../templates/blocks/stream_info.tmpl");

pub fn get_metadata(mut block_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let min_block_size = take_first_two_bytes_as_unsigned_integer(&mut block_data, Endian::Big)?;
    let max_block_size = take_first_two_bytes_as_unsigned_integer(&mut block_data, Endian::Big)?;
    let min_frame_size = take_first_three_bytes_as_32bit_unsigned_integer(&mut block_data, Endian::Big)?;
    let max_frame_size = take_first_three_bytes_as_32bit_unsigned_integer(&mut block_data, Endian::Big)?;
    let sub_byte_fields_bytes = take_first_number_of_bytes(&mut block_data, RATE_CHANNEL_AND_BITS_LENGTH_IN_BYTES)?;
    let md5sum_bytes = take_first_number_of_bytes(&mut block_data, MD5SUM_LENGTH_IN_BYTES)?;

    let sample_rate = get_sample_rate_from_bytes(&sub_byte_fields_bytes);
    let number_of_channels = get_number_of_channels_from_byte(&sub_byte_fields_bytes);
    let bits_per_sample = get_bits_per_sample_from_bytes(&sub_byte_fields_bytes);
    let interchannel_samples = get_interchannel_samples_from_bytes(sub_byte_fields_bytes);
    let md5sum = get_md5sum_from_bytes(&md5sum_bytes);

    let output_values: Value = upon::value! {
        min_block_size: min_block_size,
        max_block_size: max_block_size,
        min_frame_size: min_frame_size,
        max_frame_size: max_frame_size,
        sample_rate: sample_rate as f64 / 1000.0,
        channels: number_of_channels,
        bits_per_sample: bits_per_sample,
        total_samples: interchannel_samples,
        md5sum: md5sum,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Mandatory,
        text: formated_output,
    })
}

fn get_md5sum_from_bytes(md5sum_bytes: &[u8]) -> String {
    let mut md5sum: [u8; 16] = [0; 16];
    md5sum.copy_from_slice(md5sum_bytes);
    format_md5_sum_from_bytes(md5sum)
}

fn get_interchannel_samples_from_bytes(data_bytes: Vec<u8>) -> u64 {
    let interchannel_samples_first_byte = data_bytes[3] & 15;
    let interchannel_samples_remaining_bytes = data_bytes[4..=7].to_vec();

    let mut interchannel_samples_bytes: Vec<u8> = vec![0, 0, 0, interchannel_samples_first_byte];
    interchannel_samples_bytes.extend(interchannel_samples_remaining_bytes);

    let mut interchannel_samples_buffer: [u8; 8] = [0; 8];
    interchannel_samples_buffer.copy_from_slice(&interchannel_samples_bytes);

    u64::from_be_bytes(interchannel_samples_buffer)
}

fn get_sample_rate_from_bytes(data_bytes: &[u8]) -> u32 {
    let mut sample_rate_raw_bytes: Vec<u8> = vec![0];
    sample_rate_raw_bytes.extend(data_bytes[0..=2].to_vec());

    let mut sample_rate_bytes: [u8; 4] = [0; 4];
    sample_rate_bytes.copy_from_slice(&sample_rate_raw_bytes);

    u32::from_be_bytes(sample_rate_bytes) >> 4
}

fn get_bits_per_sample_from_bytes(data_bytes: &[u8]) -> u8 {
    let bits_per_sample_minus_one_first_bit_byte: u8 = data_bytes[2];
    let bits_per_sample_minus_one_remaining_bits = data_bytes[3] >> 4;

    let bits_per_sample_minus_one_first_bit = (bits_per_sample_minus_one_first_bit_byte & 1) << 4;
    let bits_per_sample_minus_one = bits_per_sample_minus_one_first_bit + bits_per_sample_minus_one_remaining_bits + 1;

    bits_per_sample_minus_one + 1
}

fn get_number_of_channels_from_byte(data_bytes: &[u8]) -> u8 {
    let number_of_channels_minus_one_byte: u8 = data_bytes[2];
    let number_of_channels_minus_one = (number_of_channels_minus_one_byte >> 1) & 7;
    number_of_channels_minus_one + 1
}

fn format_md5_sum_from_bytes(bytes: [u8; 16]) -> String {
    bytes
        .iter()
        .fold(String::new(), |acc, byte| acc + format!("{:02x}", byte).as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_correct_md5sum_from_valid_bytes() {
        let test_bytes: [u8; 16] = [0x1A; 16];
        let correct_md5_format = String::from("1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a");
        let result = get_md5sum_from_bytes(&test_bytes);
        assert_eq!(correct_md5_format, result);
    }

    #[test]
    fn return_correct_interchannel_samples_from_valid_bytes() {
        let test_byte: Vec<u8> = vec![0x01, 0x02, 0x03, 0x0F, 0x20, 0x06, 0x07, 0x08];
        let correct_interchannel_samples = 64961775368;
        let result = get_interchannel_samples_from_bytes(test_byte);
        assert_eq!(result, correct_interchannel_samples);
    }

    #[test]
    fn return_correct_sample_rate_from_valid_bytes() {
        let test_byte: Vec<u8> = vec![0x0B, 0xB8, 0x00, 0x60, 0x05, 0x06];
        let correct_sample_rate = 48000;
        let result = get_sample_rate_from_bytes(&test_byte);
        assert_eq!(result, correct_sample_rate);
    }

    #[test]
    fn return_the_correct_bits_per_sample_from_valid_bytes() {
        let test_byte: Vec<u8> = vec![0x01, 0x02, 0x03, 0x60, 0x05, 0x06];
        let correct_bits_per_sample = 24;
        let result = get_bits_per_sample_from_bytes(&test_byte);
        assert_eq!(result, correct_bits_per_sample);
    }

    #[test]
    fn return_the_correct_number_of_channels_from_valid_bytes() {
        let test_byte: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let correct_number_of_channels = 2;
        let result = get_number_of_channels_from_byte(&test_byte);
        assert_eq!(result, correct_number_of_channels);
    }

    #[test]
    fn returns_the_correct_md5_format_from_passed_bytes() {
        let test_bytes: [u8; 16] = [0x1A; 16];
        let correct_md5_format = String::from("1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a");
        let result = format_md5_sum_from_bytes(test_bytes);
        assert_eq!(correct_md5_format, result);
    }
}
