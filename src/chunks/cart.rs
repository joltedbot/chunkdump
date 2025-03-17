use crate::byte_arrays::{
    take_first_four_bytes_as_unsigned_integer, take_first_number_of_bytes,
    take_first_number_of_bytes_as_string, Endian,
};
use crate::errors::LocalError;
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use serde::Serialize;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/cart.tmpl");

const VERSION_LENGTH_IN_BYTES: usize = 4;
const TITLE_LENGTH_IN_BYTES: usize = 64;
const ARTIST_LENGTH_IN_BYTES: usize = 64;
const CUE_ID_LENGTH_IN_BYTES: usize = 64;
const CLIENT_ID_LENGTH_IN_BYTES: usize = 64;
const CATEGORY_LENGTH_IN_BYTES: usize = 64;
const CLASSIFICATION_LENGTH_IN_BYTES: usize = 64;
const OUT_CUE_LENGTH_IN_BYTES: usize = 64;
const START_DATE_LENGTH_IN_BYTES: usize = 10;
const START_TIME_LENGTH_IN_BYTES: usize = 8;
const END_DATE_LENGTH_IN_BYTES: usize = 10;
const END_TIME_LENGTH_IN_BYTES: usize = 8;
const PRODUCER_APP_ID_LENGTH_IN_BYTES: usize = 64;
const PRODUCER_APP_VERSION_LENGTH_IN_BYTES: usize = 64;
const USER_DEF_LENGTH_IN_BYTES: usize = 64;
const POST_TIMER_LENGTH_IN_BYTES: usize = 64;
const NUMBER_OF_POST_TIMERS_PER_TIMER: usize = 8;
const DW_USAGE_LENGTH_IN_BYTES: usize = 4;
const RESERVED_LENGTH_IN_BYTES: usize = 276;
const URL_LENGTH_IN_BYTES: usize = 1024;
const VERSION_STRING_DECIMAL_POSITION: usize = 2;

#[derive(Debug, Serialize)]
struct CartTimer {
    dw_usage: String,
    dw_value: u32,
}

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let version = take_first_number_of_bytes_as_string(&mut chunk_data, VERSION_LENGTH_IN_BYTES)?;
    let title = take_first_number_of_bytes_as_string(&mut chunk_data, TITLE_LENGTH_IN_BYTES)?;
    let artist = take_first_number_of_bytes_as_string(&mut chunk_data, ARTIST_LENGTH_IN_BYTES)?;
    let cue_id = take_first_number_of_bytes_as_string(&mut chunk_data, CUE_ID_LENGTH_IN_BYTES)?;
    let client_id =
        take_first_number_of_bytes_as_string(&mut chunk_data, CLIENT_ID_LENGTH_IN_BYTES)?;
    let category = take_first_number_of_bytes_as_string(&mut chunk_data, CATEGORY_LENGTH_IN_BYTES)?;
    let classification =
        take_first_number_of_bytes_as_string(&mut chunk_data, CLASSIFICATION_LENGTH_IN_BYTES)?;
    let out_cue = take_first_number_of_bytes_as_string(&mut chunk_data, OUT_CUE_LENGTH_IN_BYTES)?;
    let start_date =
        take_first_number_of_bytes_as_string(&mut chunk_data, START_DATE_LENGTH_IN_BYTES)?;
    let start_time =
        take_first_number_of_bytes_as_string(&mut chunk_data, START_TIME_LENGTH_IN_BYTES)?;
    let end_date = take_first_number_of_bytes_as_string(&mut chunk_data, END_DATE_LENGTH_IN_BYTES)?;
    let end_time = take_first_number_of_bytes_as_string(&mut chunk_data, END_TIME_LENGTH_IN_BYTES)?;
    let producer_app_id =
        take_first_number_of_bytes_as_string(&mut chunk_data, PRODUCER_APP_ID_LENGTH_IN_BYTES)?;
    let producer_app_version = take_first_number_of_bytes_as_string(
        &mut chunk_data,
        PRODUCER_APP_VERSION_LENGTH_IN_BYTES,
    )?;
    let user_def = take_first_number_of_bytes_as_string(&mut chunk_data, USER_DEF_LENGTH_IN_BYTES)?;
    let dw_level_reference =
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
    let post_timer = get_post_timer_from_bytes(take_first_number_of_bytes(
        &mut chunk_data,
        POST_TIMER_LENGTH_IN_BYTES,
    )?)?;
    let reserved = take_first_number_of_bytes_as_string(&mut chunk_data, RESERVED_LENGTH_IN_BYTES)?;
    let url = take_first_number_of_bytes_as_string(&mut chunk_data, URL_LENGTH_IN_BYTES)?;

    let remaining_data_bytes = chunk_data.len();
    let tag_text = take_first_number_of_bytes_as_string(&mut chunk_data, remaining_data_bytes)?;

    let wave_output_values: Value = upon::value! {
       version: get_formated_version_from_version_string(version.clone()),
        title: &title,
        artist: &artist,
        cue_id: &cue_id,
        client_id: &client_id,
        category: &category,
        classification: &classification,
        out_cue: &out_cue,
        start_date: &start_date,
        start_time: &start_time,
        end_date: &end_date,
        end_time: &end_time,
        producer_app_id: &producer_app_id,
        producer_app_version: &producer_app_version,
        user_def: &user_def,
        dw_level_reference: dw_level_reference,
        url: &url,
        tag_text: &tag_text,
        reserved: &reserved,
        post_timer: &post_timer,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, wave_output_values)?;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}

fn get_post_timer_from_bytes(mut post_timer_data: Vec<u8>) -> Result<Vec<CartTimer>, LocalError> {
    let mut post_timer: Vec<CartTimer> = vec![];

    for _ in 0..NUMBER_OF_POST_TIMERS_PER_TIMER {
        let dw_usage =
            take_first_number_of_bytes_as_string(&mut post_timer_data, DW_USAGE_LENGTH_IN_BYTES)?;
        let dw_value =
            take_first_four_bytes_as_unsigned_integer(&mut post_timer_data, Endian::Little)?;

        if !dw_usage.is_empty() || dw_value != 0 {
            post_timer.push(CartTimer { dw_usage, dw_value });
        }
    }

    Ok(post_timer)
}

fn get_formated_version_from_version_string(mut version: String) -> String {
    if version.len() <= VERSION_STRING_DECIMAL_POSITION {
        return version;
    }

    version.insert(VERSION_STRING_DECIMAL_POSITION, '.');

    let formated_version = version.trim_start_matches("0").to_string();
    formated_version
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_formats_the_version_string_when_passed_a_four_digit_version() {
        let test_version = "0234".to_string();
        let expected_result = "2.34".to_string();
        let result = get_formated_version_from_version_string(test_version);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn returns_the_passed_version_unaltered_if_it_is_less_than_3_digits() {
        let test_version = "34".to_string();
        let expected_result = "34".to_string();
        let result = get_formated_version_from_version_string(test_version);
        assert_eq!(result, expected_result);
    }
}
