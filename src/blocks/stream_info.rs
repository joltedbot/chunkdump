use crate::byte_arrays::{
    take_first_number_of_bytes, take_first_three_bytes_as_32bit_unsigned_integer,
    take_first_two_bytes_as_unsigned_integer, Endian,
};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use upon::Value;

const MD5SUM_LENGTH_IN_BYTES: usize = 16;
const RATE_CHANNEL_AND_BITS_BYTE_ARRAY: usize = 8;

const TEMPLATE_CONTENT: &str = include_str!("../templates/blocks/stream_info.tmpl");

pub fn get_metadata(mut block_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let min_block_size = take_first_two_bytes_as_unsigned_integer(&mut block_data, Endian::Big)?;
    let max_block_size = take_first_two_bytes_as_unsigned_integer(&mut block_data, Endian::Big)?;
    let min_frame_size = take_first_three_bytes_as_32bit_unsigned_integer(&mut block_data, Endian::Big)?;
    let max_frame_size = take_first_three_bytes_as_32bit_unsigned_integer(&mut block_data, Endian::Big)?;

    let rate_channels_and_bits_bytes = take_first_number_of_bytes(&mut block_data, RATE_CHANNEL_AND_BITS_BYTE_ARRAY)?;
    let rate_channels_and_bits = get_rate_channels_and_bits_from_bytes(rate_channels_and_bits_bytes)?;
    let sample_rate = rate_channels_and_bits.0;
    let channels = rate_channels_and_bits.1;
    let bits_per_sample = rate_channels_and_bits.2;
    let samples = rate_channels_and_bits.3;

    let md5sum_bytes = take_first_number_of_bytes(&mut block_data, MD5SUM_LENGTH_IN_BYTES)?;
    let mut md5sum: [u8; 16] = [0; 16];
    md5sum.copy_from_slice(&md5sum_bytes);
    let formated_md5sum = format_md5_sum_from_bytes(md5sum);

    let output_values: Value = upon::value! {
        min_block_size: min_block_size,
        max_block_size: max_block_size,
        min_frame_size: min_frame_size,
        max_frame_size: max_frame_size,
        sample_rate: sample_rate as f64 / 1000.0,
        channels: channels,
        bits_per_sample: bits_per_sample,
        total_samples: samples,
        md5sum: formated_md5sum,
    };
    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Mandatory,
        text: formated_output,
    })
}

fn get_rate_channels_and_bits_from_bytes(data_bytes: Vec<u8>) -> Result<(u32, u8, u8, u64), Box<dyn Error>> {
    let mut sample_rate_raw_bytes: Vec<u8> = vec![0];
    sample_rate_raw_bytes.extend(data_bytes[0..=2].to_vec());
    let mut sample_rate_bytes: [u8; 4] = [0; 4];
    sample_rate_bytes.copy_from_slice(&sample_rate_raw_bytes);
    let sample_rate = u32::from_be_bytes(sample_rate_bytes) >> 4;

    let number_of_channels_byte: u8 = data_bytes[2];
    let number_of_channels = ((number_of_channels_byte >> 1) & 7) + 1;

    let bits_per_sample_first_bit = (number_of_channels_byte & 1) << 4;
    let bits_per_sample_rest_of_bits = data_bytes[3] >> 4;
    let bits_per_sample = bits_per_sample_first_bit + bits_per_sample_rest_of_bits + 1;

    let interchannel_samples_first_byte = data_bytes[3] & 15;
    let interchannel_samples_rest_of_the_bytes = data_bytes[4..=7].to_vec();
    let mut interchannel_samples_bytes: Vec<u8> = vec![0, 0, 0, interchannel_samples_first_byte];
    interchannel_samples_bytes.extend(interchannel_samples_rest_of_the_bytes);

    let mut byte_buffer: [u8; 8] = [0; 8];
    byte_buffer.copy_from_slice(&interchannel_samples_bytes);
    let ineterchannel_samples = u64::from_be_bytes(byte_buffer);

    Ok((sample_rate, number_of_channels, bits_per_sample, ineterchannel_samples))
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
    fn returns_the_correct_md5_format_from_passed_bytes() {
        let input_bytes: [u8; 16] = [0x1A; 16];
        let correct_md5_format = String::from("1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a");
        let result = format_md5_sum_from_bytes(input_bytes);
        assert_eq!(correct_md5_format, result);
    }
}
