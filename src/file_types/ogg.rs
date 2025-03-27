use crate::errors::LocalError;
use crate::fileio::{
    get_file_metadata, read_byte_from_file, read_bytes_from_file, skip_over_bytes_in_file,
};
use crate::formating::{set_key_value_pair_spacers, KeyValuePair as UserComment};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use byte_unit::rust_decimal::prelude::Zero;
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use upon::Value;

const OGG_CONTAINER_HEADER_LENGTH_IN_BYTES: usize = 26;
const VORBIS_COMMON_HEADER_LENGTH_IN_BYTES: usize = 7;
const FRAMING_FLAG_LENGTH_IN_BYTES: usize = 1;
const TEMPLATE_CONTENT: &str = include_str!("../templates/file_types/ogg.tmpl");
const HEADER_TEMPLATE_CONTENT: &str = include_str!("../templates/file_types/ogg_headers.tmpl");
const FIXED_BIT_RATE_OUTPUT_STRING: &str = "Fixed";
const VBR_ABR_BIT_RATE_OUTPUT_STRING: &str = "VBR or ABR";
const MAX_LIMITED_BIT_RATE_OUTPUT_STRING: &str = "Maximum Limited";
const MIN_LIMITED_BIT_RATE_OUTPUT_STRING: &str = "Minimum Limited";
const UNKNOWN_BIT_RATE_OUTPUT_STRING: &str = "Unknown";
const BAD_USER_COMMENT_KEY: &str = "XXXX";
const BAD_USER_COMMENT_VALUE: &str =
    "[Corrupt or Non-Standard Format User Comment. Halting processing comments.]";

#[derive(Debug, Default, Serialize)]
struct HeaderMetadata {
    vorbis_version: u32,
    audio_channels: u8,
    audio_sample_rate: u32,
    bitrate_maximum: u32,
    bitrate_nominal: u32,
    bitrate_minimum: u32,
    blocksizes: (u8, u8),
    bitrate_type: String,
    vendor_comment: String,
    user_comments: Vec<UserComment>,
}

pub fn get_metadata_from_file(ogg_file_path: &str) -> Result<Vec<OutputEntry>, Box<dyn Error>> {
    let mut ogg_file = File::open(ogg_file_path)?;
    let file_metadata = get_file_metadata(ogg_file_path, &ogg_file, TEMPLATE_CONTENT)?;
    let vorbis_metadata = get_metadata_from_headers(&mut ogg_file)?;
    let chunks = vec![file_metadata, vorbis_metadata];

    Ok(chunks)
}

fn get_metadata_from_headers(ogg_file: &mut File) -> Result<OutputEntry, Box<dyn Error>> {
    let mut header_metadata = get_identification_header_metadata_from_file(ogg_file)?;
    get_comment_header_metadata_from_file(ogg_file, &mut header_metadata)?;

    let output_values: Value = upon::value! {
        vorbis_version: header_metadata.vorbis_version,
        audio_channels: header_metadata.audio_channels,
        audio_sample_rate: header_metadata.audio_sample_rate as f64 / 1000.0,
        bitrate_maximum: header_metadata.bitrate_maximum/1000,
        bitrate_nominal: header_metadata.bitrate_nominal/1000,
        bitrate_minimum: header_metadata.bitrate_minimum/1000,
        blocksize_0: header_metadata.blocksizes.0,
        blocksize_1: header_metadata.blocksizes.1,
        bitrate_type: header_metadata.bitrate_type,
        vendor_comment: header_metadata.vendor_comment,
        user_comments: header_metadata.user_comments,
    };

    let formated_output = get_file_chunk_output(HEADER_TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Mandatory,
        text: formated_output,
    })
}

fn get_identification_header_metadata_from_file(
    ogg_file: &mut File,
) -> Result<HeaderMetadata, Box<dyn Error>> {
    skip_over_ogg_packet_and_vorbis_common_headers_in_file(ogg_file)?;

    let vorbis_version = get_4_byte_field_from_file(ogg_file)?;
    let audio_channels = read_byte_from_file(ogg_file)?;
    let audio_sample_rate = get_4_byte_field_from_file(ogg_file)?;
    let bitrate_maximum = get_4_byte_field_from_file(ogg_file)?;
    let bitrate_nominal = get_4_byte_field_from_file(ogg_file)?;
    let bitrate_minimum = get_4_byte_field_from_file(ogg_file)?;
    let blocksizes: (u8, u8) = get_blocksizes_from_byte(read_byte_from_file(ogg_file)?);
    let bitrate_type =
        get_bitrate_type_from_bitrate_values(bitrate_minimum, bitrate_nominal, bitrate_maximum);

    let header_metadata = HeaderMetadata {
        vorbis_version,
        audio_channels,
        audio_sample_rate,
        bitrate_maximum,
        bitrate_nominal,
        bitrate_minimum,
        blocksizes,
        bitrate_type,
        ..Default::default()
    };

    skip_over_bytes_in_file(ogg_file, FRAMING_FLAG_LENGTH_IN_BYTES)?;

    Ok(header_metadata)
}

fn get_comment_header_metadata_from_file(
    ogg_file: &mut File,
    header_metadata: &mut HeaderMetadata,
) -> Result<(), Box<dyn Error>> {
    skip_over_ogg_packet_and_vorbis_common_headers_in_file(ogg_file)?;

    let vendor_comment_length_in_bytes = get_4_byte_field_from_file(ogg_file)?;
    header_metadata.vendor_comment =
        get_string_from_file(ogg_file, vendor_comment_length_in_bytes as usize)?;

    let number_of_user_comments = get_4_byte_field_from_file(ogg_file)?;
    for _ in 0..number_of_user_comments {
        match get_user_comment_from_file(ogg_file) {
            Ok(comment) => header_metadata.user_comments.push(comment),
            Err(_) => header_metadata.user_comments.push(UserComment {
                key: BAD_USER_COMMENT_KEY.to_string(),
                spacer: " ".to_string(),
                value: BAD_USER_COMMENT_VALUE.to_string(),
            }),
        }
    }

    set_key_value_pair_spacers(&mut header_metadata.user_comments);

    Ok(())
}

fn skip_over_ogg_packet_and_vorbis_common_headers_in_file(
    ogg_file: &mut File,
) -> Result<(), Box<dyn Error>> {
    skip_over_bytes_in_file(ogg_file, OGG_CONTAINER_HEADER_LENGTH_IN_BYTES)?;
    let number_of_page_segments = read_byte_from_file(ogg_file)?;
    skip_over_bytes_in_file(ogg_file, number_of_page_segments as usize)?;

    skip_over_bytes_in_file(ogg_file, VORBIS_COMMON_HEADER_LENGTH_IN_BYTES)?;

    Ok(())
}

fn get_user_comment_from_file(ogg_file: &mut File) -> Result<UserComment, Box<dyn Error>> {
    let user_comment_length = get_4_byte_field_from_file(ogg_file)?;
    let user_comment = get_string_from_file(ogg_file, user_comment_length as usize)?;
    let user_comment_key_value = user_comment
        .split_once("=")
        .ok_or(LocalError::InvalidVorbisComment)?;

    Ok(UserComment {
        key: user_comment_key_value.0.to_string(),
        spacer: " ".to_string(),
        value: user_comment_key_value.1.to_string(),
    })
}

fn get_4_byte_field_from_file(file: &mut File) -> Result<u32, Box<dyn Error>> {
    let size_bytes = read_bytes_from_file(file, 4)?;
    let mut byte_array: [u8; 4] = Default::default();
    byte_array.copy_from_slice(size_bytes.as_slice());

    Ok(u32::from_le_bytes(byte_array))
}

fn get_string_from_file(
    file: &mut File,
    string_length_in_bytes: usize,
) -> Result<String, Box<dyn Error>> {
    let read_bytes = read_bytes_from_file(file, string_length_in_bytes)?;

    Ok(String::from_utf8(read_bytes)?)
}

fn get_blocksizes_from_byte(blocksize_byte: u8) -> (u8, u8) {
    let blocksize_1: u8 = blocksize_byte & 15;
    let blocksize_0: u8 = blocksize_byte >> 4;

    (blocksize_0, blocksize_1)
}

fn get_bitrate_type_from_bitrate_values(minimum: u32, nominal: u32, maximum: u32) -> String {
    if maximum == nominal && nominal == minimum {
        return FIXED_BIT_RATE_OUTPUT_STRING.to_string();
    }

    if !nominal.is_zero() && maximum.is_zero() && minimum.is_zero() {
        return VBR_ABR_BIT_RATE_OUTPUT_STRING.to_string();
    }

    if !maximum.is_zero() && nominal.is_zero() && minimum.is_zero() {
        return MAX_LIMITED_BIT_RATE_OUTPUT_STRING.to_string();
    }

    if !minimum.is_zero() && nominal.is_zero() && maximum.is_zero() {
        return MIN_LIMITED_BIT_RATE_OUTPUT_STRING.to_string();
    }

    UNKNOWN_BIT_RATE_OUTPUT_STRING.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_correct_blocksizes_from_valid_blocksize_byte() {
        let test_byte = 0b11010101;
        let correct_blocksizes = (13, 5);
        let blocksizes = get_blocksizes_from_byte(test_byte);

        assert_eq!(correct_blocksizes, blocksizes);
    }

    #[test]
    fn return_fixed_bit_rate_output_string_when_all_bitrates_are_equal() {
        let test_minimum = 123456;
        let test_nominal = 123456;
        let test_maximum = 123456;
        let output_string =
            get_bitrate_type_from_bitrate_values(test_minimum, test_nominal, test_maximum);
        assert_eq!(output_string, FIXED_BIT_RATE_OUTPUT_STRING.to_string());
    }

    #[test]
    fn return_variable_bit_rate_output_string_when_min_and_max_are_zero_but_nominal_is_not_zero() {
        let test_minimum = 0;
        let test_nominal = 123456;
        let test_maximum = 0;
        let output_string =
            get_bitrate_type_from_bitrate_values(test_minimum, test_nominal, test_maximum);
        assert_eq!(output_string, VBR_ABR_BIT_RATE_OUTPUT_STRING.to_string());
    }

    #[test]
    fn return_max_limited_bit_rate_output_string_when_min_and_nominal_are_zero_but_max_is_not_zero()
    {
        let test_minimum = 0;
        let test_nominal = 0;
        let test_maximum = 123456;
        let output_string =
            get_bitrate_type_from_bitrate_values(test_minimum, test_nominal, test_maximum);
        assert_eq!(
            output_string,
            MAX_LIMITED_BIT_RATE_OUTPUT_STRING.to_string()
        );
    }

    #[test]
    fn return_min_limited_bit_rate_output_string_when_max_and_nominal_are_zero_but_min_is_not_zero()
    {
        let test_minimum = 123456;
        let test_nominal = 0;
        let test_maximum = 0;
        let output_string =
            get_bitrate_type_from_bitrate_values(test_minimum, test_nominal, test_maximum);
        assert_eq!(
            output_string,
            MIN_LIMITED_BIT_RATE_OUTPUT_STRING.to_string()
        );
    }

    #[test]
    fn return_unknown_bit_rate_output_string_when_max_and_nominal_bitrates_are_set_and_min_is_zero()
    {
        let test_minimum = 0;
        let test_nominal = 123456;
        let test_maximum = 123456;
        let output_string =
            get_bitrate_type_from_bitrate_values(test_minimum, test_nominal, test_maximum);
        assert_eq!(output_string, UNKNOWN_BIT_RATE_OUTPUT_STRING.to_string());
    }

    #[test]
    fn return_unknown_bit_rate_output_string_when_min_and_nominal_bitrates_are_set_and_max_is_zero()
    {
        let test_minimum = 123456;
        let test_nominal = 123456;
        let test_maximum = 0;
        let output_string =
            get_bitrate_type_from_bitrate_values(test_minimum, test_nominal, test_maximum);
        assert_eq!(output_string, UNKNOWN_BIT_RATE_OUTPUT_STRING.to_string());
    }
}
