use crate::byte_arrays::{
    take_first_four_bytes_as_unsigned_integer, take_first_two_bytes_as_unsigned_integer, Endian,
};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use byte_unit::rust_decimal::prelude::Zero;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/fmt.tmpl");
const FORMAT_CHUNK_SIZE_IF_NO_EXTENSION: usize = 16;
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
const SIZE_IF_EXTENSION_IS_PRESENT: u16 = 22;
const SPEAKER_POSITION_MASK_BIT_MEANING: [&str; 18] = [
    "Front Left",
    "Front Right",
    "Front Center",
    "Low Frequency",
    "Back Left",
    "Back Right",
    "Front Left Of Center",
    "Front Right Of Center",
    "Back Center",
    "Side Left",
    "Side Right",
    "Top Center",
    "Top Front Left",
    "Top Front Center",
    "Top Front Right",
    "Top Back Left",
    "Top Back Center",
    "Top Back Right",
];

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let chunk_size = chunk_data.len();

    let format_code = get_format_name_from_format_id(take_first_two_bytes_as_unsigned_integer(
        &mut chunk_data,
        Endian::Little,
    )?);
    let number_of_channels =
        take_first_two_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
    let samples_per_second =
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
    let average_data_rate =
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
    let data_block_size =
        take_first_two_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
    let bits_per_sample =
        take_first_two_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;

    let mut extension_size: u16 = Default::default();

    if chunk_size > FORMAT_CHUNK_SIZE_IF_NO_EXTENSION {
        extension_size = take_first_two_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
    }

    let mut valid_bits_per_sample: u16 = Default::default();
    let mut speaker_position_mask: u32 = Default::default();
    let mut subformat_guid: [u8; GUID_LENGTH_IN_BYTES] = Default::default();

    if extension_size == SIZE_IF_EXTENSION_IS_PRESENT {
        valid_bits_per_sample =
            take_first_two_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
        speaker_position_mask =
            take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
        subformat_guid.copy_from_slice(chunk_data.as_slice());
    }

    let output_values: Value = upon::value! {
        format_code: &format_code,
        number_of_channels: &number_of_channels,
        samples_per_second: &(samples_per_second as f64 / 1000.0),
        bits_per_sample: &bits_per_sample,
        average_data_rate: &(average_data_rate as f64 / 1000.0),
        data_block_size: data_block_size,
        valid_bits_per_sample: &valid_bits_per_sample,
        speaker_position_mask: format_speaker_position(speaker_position_mask),
        subformat_guid: format_guid(subformat_guid)
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Mandatory,
        text: formated_output,
    })
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

fn format_guid(guid_bytes: [u8; GUID_LENGTH_IN_BYTES]) -> String {
    let max_byte: &u8 = guid_bytes.iter().max().unwrap_or(&0);

    if max_byte.is_zero() {
        return String::new();
    }

    let formated_guid: Vec<String> = guid_bytes
        .iter()
        .map(|byte| format!("{:X}", byte))
        .collect();

    formated_guid.join("")
}

fn format_speaker_position(speaker_position_mask: u32) -> String {
    let mut positions: Vec<String> = Default::default();

    SPEAKER_POSITION_MASK_BIT_MEANING
        .iter()
        .enumerate()
        .for_each(|(index, mask)| {
            if (speaker_position_mask & (1 << index)) > 0 {
                positions.push(format!(" - {}", mask));
            }
        });

    if !positions.is_empty() {
        positions.insert(0, "".to_string());
    }

    positions.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::chunks::fmt::{
        format_guid, format_speaker_position, get_format_name_from_format_id, PCM_FORMAT_NAME,
        UNKOWN_FORMAT,
    };

    #[test]
    fn return_correct_format_name_from_valid_format_id_integer() {
        let test_integer: u16 = 1;
        let correct_result: String = PCM_FORMAT_NAME.to_string();
        let result = get_format_name_from_format_id(test_integer);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_correct_format_name_from_unknown_format_id_integer() {
        let test_integer: u16 = 0;
        let correct_result: String = format!("{} {}", UNKOWN_FORMAT.to_string(), test_integer);
        let result = get_format_name_from_format_id(test_integer);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_correct_guid_from_valid_guid_bytes() {
        let test_guid_bytes: [u8; 16] = [0x2A; 16];
        let correct_result: String = "2A".repeat(16).to_string();
        let result = format_guid(test_guid_bytes);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_empty_string_from_empty_guid_bytes() {
        let test_guid_bytes: [u8; 16] = [0; 16];
        let correct_result: String = "".to_string();
        let result = format_guid(test_guid_bytes);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_correct_speaker_positions_from_integer_with_all_position_bits_set() {
        let test_speaker_position_mask: u32 = 262143;
        let correct_result: String = "\n - Front Left\n - Front Right\n - Front Center\n - Low Frequency\n - Back Left\n - Back Right\n - Front Left Of Center\n - Front Right Of Center\n - Back Center\n - Side Left\n - Side Right\n - Top Center\n - Top Front Left\n - Top Front Center\n - Top Front Right\n - Top Back Left\n - Top Back Center\n - Top Back Right".to_string();
        let result = format_speaker_position(test_speaker_position_mask);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_correct_speaker_positions_from_integer_zero() {
        let test_speaker_position_mask: u32 = 0;
        let correct_result: String = "".to_string();
        let result = format_speaker_position(test_speaker_position_mask);
        assert_eq!(result, correct_result);
    }
}
