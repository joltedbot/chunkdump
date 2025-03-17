use crate::byte_arrays::{
    take_first_eight_bytes_as_unsigned_integer, take_first_number_of_bytes,
    take_first_number_of_bytes_as_string, take_first_two_bytes_as_signed_integer,
    take_first_two_bytes_as_unsigned_integer, Endian,
};
use std::error::Error;

use crate::errors::LocalError;
use crate::formating::format_bytes_as_string_of_bytes;
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/bext.tmpl");

const DESCRIPTION_LENGTH_IN_BYTES: usize = 256;
const ORIGINATOR_LENGTH_IN_BYTES: usize = 32;
const ORIGINATOR_REFERENCE_LENGTH_IN_BYTES: usize = 32;
const ORIGINATOR_DATA_LENGTH_IN_BYTES: usize = 10;
const ORIGINATOR_TIME_LENGTH_IN_BYTES: usize = 8;
const RESERVED_FIELD_LENGTH_IN_BYTES: usize = 180;
const UMID_UNIVERSAL_LABEL_LENGTH_IN_BYTES: usize = 12;
const UMID_LENGTH_LENGTH_IN_BYTES: usize = 1;
const UMID_INSTANCE_NUMBER_LENGTH_IN_BYTES: usize = 3;
const UMID_MATERIAL_NUMBER_LEMGTH_IN_BYTES: usize = 16;
const UMID_TIME_AND_DATE_LENGTH_IN_BYTES: usize = 8;
const UMID_SPATIAL_COORDINATES_LENGTH_IN_BYTES: usize = 12;
const UMID_COUNTRY_LENGTH_IN_BYTES: usize = 4;
const UMID_ORGANIZATION_LENGTH_IN_BYTES: usize = 4;
const UMID_USER_LENGTH_IN_BYTES: usize = 4;

#[derive(Debug, PartialEq)]
struct UmidComponent {
    universal_label: Vec<u8>,
    length: Vec<u8>,
    instance_number: Vec<u8>,
    material_number: Vec<u8>,
    time_and_date: Vec<u8>,
    spatial_coordinates: Vec<u8>,
    country: Vec<u8>,
    organization: Vec<u8>,
    user: Vec<u8>,
}

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let description =
        take_first_number_of_bytes_as_string(&mut chunk_data, DESCRIPTION_LENGTH_IN_BYTES)?;
    let originator =
        take_first_number_of_bytes_as_string(&mut chunk_data, ORIGINATOR_LENGTH_IN_BYTES)?;
    let originator_reference = take_first_number_of_bytes_as_string(
        &mut chunk_data,
        ORIGINATOR_REFERENCE_LENGTH_IN_BYTES,
    )?;
    let originator_date =
        take_first_number_of_bytes_as_string(&mut chunk_data, ORIGINATOR_DATA_LENGTH_IN_BYTES)?;
    let originator_time =
        take_first_number_of_bytes_as_string(&mut chunk_data, ORIGINATOR_TIME_LENGTH_IN_BYTES)?;
    let time_reference =
        take_first_eight_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
    let version = take_first_two_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
    let umid = get_umid_from_bytes(&mut chunk_data)?;
    let loudness_value = take_first_two_bytes_as_signed_integer(&mut chunk_data, Endian::Little)?;
    let loudness_range = take_first_two_bytes_as_signed_integer(&mut chunk_data, Endian::Little)?;
    let max_true_peak_level =
        take_first_two_bytes_as_signed_integer(&mut chunk_data, Endian::Little)?;
    let max_momentary_loudness =
        take_first_two_bytes_as_signed_integer(&mut chunk_data, Endian::Little)?;
    let max_short_term_loudness =
        take_first_two_bytes_as_signed_integer(&mut chunk_data, Endian::Little)?;
    let reserved =
        take_first_number_of_bytes_as_string(&mut chunk_data, RESERVED_FIELD_LENGTH_IN_BYTES)?;
    let coding_history = get_coding_history_from_bytes(chunk_data)?;

    let wave_output_values: Value = upon::value! {
        description: description,
        originator: originator,
        originator_reference: originator_reference,
        originator_date: &originator_date,
        originator_time: &originator_time,
        time_reference: time_reference,
        version: version,
        loudness_value: loudness_value / 100,
        loudness_range: loudness_range / 100,
        max_true_peak_level: max_true_peak_level / 100,
        max_momentary_loudness: max_momentary_loudness / 100,
        max_short_term_loudness: max_short_term_loudness / 100,
        reserved: &reserved,
        coding_history: &coding_history,
        universal_label: format_bytes_as_string_of_bytes(&umid.universal_label),
        instance_number: format_bytes_as_string_of_bytes(&umid.instance_number),
        material_number: format_bytes_as_string_of_bytes(&umid.material_number),
        time_and_date: format_bytes_as_string_of_bytes(&umid.time_and_date),
        spatial_coordinates: format_bytes_as_string_of_bytes(&umid.spatial_coordinates),
        country: format_bytes_as_string_of_bytes(&umid.country),
        organization: format_bytes_as_string_of_bytes(&umid.organization),
        user: format_bytes_as_string_of_bytes(&umid.user),
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, wave_output_values)?;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}

fn get_coding_history_from_bytes(mut bext_data: Vec<u8>) -> Result<String, LocalError> {
    let mut coding_history = "".to_string();

    if !bext_data.is_empty() {
        let remaining_bytes = bext_data.len();
        coding_history = take_first_number_of_bytes_as_string(&mut bext_data, remaining_bytes)?;
    }

    Ok(coding_history)
}

fn get_umid_from_bytes(umid_data: &mut Vec<u8>) -> Result<UmidComponent, LocalError> {
    Ok(UmidComponent {
        universal_label: take_first_number_of_bytes(
            umid_data,
            UMID_UNIVERSAL_LABEL_LENGTH_IN_BYTES,
        )?,
        length: take_first_number_of_bytes(umid_data, UMID_LENGTH_LENGTH_IN_BYTES)?,
        instance_number: take_first_number_of_bytes(
            umid_data,
            UMID_INSTANCE_NUMBER_LENGTH_IN_BYTES,
        )?,
        material_number: take_first_number_of_bytes(
            umid_data,
            UMID_MATERIAL_NUMBER_LEMGTH_IN_BYTES,
        )?,
        time_and_date: take_first_number_of_bytes(umid_data, UMID_TIME_AND_DATE_LENGTH_IN_BYTES)?,
        spatial_coordinates: take_first_number_of_bytes(
            umid_data,
            UMID_SPATIAL_COORDINATES_LENGTH_IN_BYTES,
        )?,
        country: take_first_number_of_bytes(umid_data, UMID_COUNTRY_LENGTH_IN_BYTES)?,
        organization: take_first_number_of_bytes(umid_data, UMID_ORGANIZATION_LENGTH_IN_BYTES)?,
        user: take_first_number_of_bytes(umid_data, UMID_USER_LENGTH_IN_BYTES)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_umid_struct_from_bytes() {
        let mut input_byte_vector: Vec<u8> = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
        ];
        let expected_result: UmidComponent = UmidComponent {
            universal_label: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            length: vec![13],
            instance_number: vec![14, 15, 16],
            material_number: vec![
                17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
            ],
            time_and_date: vec![33, 34, 35, 36, 37, 38, 39, 40],
            spatial_coordinates: vec![41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52],
            country: vec![53, 54, 55, 56],
            organization: vec![57, 58, 59, 60],
            user: vec![61, 62, 63, 64],
        };

        let result = get_umid_from_bytes(&mut input_byte_vector).unwrap();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn return_correct_coding_history_from_valid_bytes() {
        let input_byte_vector: Vec<u8> = vec![
            84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 84, 101, 115, 116,
        ];
        let correct_result_string: String = "This is a Test".to_string();
        let result = get_coding_history_from_bytes(input_byte_vector).unwrap();

        assert_eq!(result, correct_result_string);
    }

    #[test]
    fn return_empty_coding_history_from_empty_bytes_collection() {
        let input_byte_vector: Vec<u8> = vec![];
        let correct_result_string: String = "".to_string();
        let result = get_coding_history_from_bytes(input_byte_vector).unwrap();
        assert_eq!(result, correct_result_string);
    }
}
