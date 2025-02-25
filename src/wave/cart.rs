use crate::bytes::{
    take_first_four_bytes_as_signed_integer, take_first_four_bytes_as_unsigned_integer, take_first_number_of_bytes,
    take_first_number_of_bytes_as_string, Endian,
};

use crate::errors::LocalError;
use crate::template::Template;
use serde::Serialize;
use upon::Value;

const TEMPLATE_NAME: &str = "cart";
const TEMPLATE_CONTENT: &str = include_str!("../templates/wave/cart.tmpl");

const VERSION_LENGTH_IN_BYTES: usize = 4;
const TITLE_LENGTH_IN_BYTES: usize = 64;
const ARTIST_LENGTH_IN_BYTES: usize = 64;
const CUE_ID_LENGTH_IN_BYTES: usize = 64;
const CLIENT_ID_LENGTH_IN_BYTES: usize = 64;
const CATEGORY_LENGTH_IN_BYTES: usize = 64;
const CLASSIFICATION_LENGTH_IN_BYTES: usize = 64;
const OUT_CUE_LENGTH_IN_BYTES: usize = 64;
const START_DATE_LENGTH_IN_BYTES: usize = 64;
const START_TIME_LENGTH_IN_BYTES: usize = 64;
const END_DATE_LENGTH_IN_BYTES: usize = 64;
const END_TIME_LENGTH_IN_BYTES: usize = 64;
const PRODUCER_APP_ID_LENGTH_IN_BYTES: usize = 64;
const PRODUCER_APP_VERSION_LENGTH_IN_BYTES: usize = 64;
const USER_DEF_LENGTH_IN_BYTES: usize = 64;
const POST_TIMER_LENGTH_IN_BYTES: usize = 64;
const NUMBER_OF_POST_TIMERS_PER_TIMER: usize = 8;
const DW_USAGE_LENGTH_IN_BYTES: usize = 4;
const RESERVED_LENGTH_IN_BYTES: usize = 64;
const URL_LENGTH_IN_BYTES: usize = 64;
const TAG_TEXT_LENGTH_IN_BYTES: usize = 64;
const VERSION_STRING_DECIMAL_POSITION: usize = 2;

#[derive(Debug, Clone, Default, Serialize)]
struct CartTimer {
    dw_usage: String,
    dw_value: u32,
}

#[derive(Debug, Clone, Default)]
pub struct CartFields {
    version: String,
    title: String,
    artist: String,
    cue_id: String,
    client_id: String,
    category: String,
    classification: String,
    out_cue: String,
    start_date: String,
    start_time: String,
    end_date: String,
    end_time: String,
    producer_app_id: String,
    producer_app_version: String,
    user_def: String,
    dw_level_reference: i32,
    post_timer: Vec<CartTimer>,
    reserved: String,
    url: String,
    tag_text: String,
}

impl CartFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, LocalError> {
        Ok(Self {
            version: take_first_number_of_bytes_as_string(&mut chunk_data, VERSION_LENGTH_IN_BYTES)?,
            title: take_first_number_of_bytes_as_string(&mut chunk_data, TITLE_LENGTH_IN_BYTES)?,
            artist: take_first_number_of_bytes_as_string(&mut chunk_data, ARTIST_LENGTH_IN_BYTES)?,
            cue_id: take_first_number_of_bytes_as_string(&mut chunk_data, CUE_ID_LENGTH_IN_BYTES)?,
            client_id: take_first_number_of_bytes_as_string(&mut chunk_data, CLIENT_ID_LENGTH_IN_BYTES)?,
            category: take_first_number_of_bytes_as_string(&mut chunk_data, CATEGORY_LENGTH_IN_BYTES)?,
            classification: take_first_number_of_bytes_as_string(&mut chunk_data, CLASSIFICATION_LENGTH_IN_BYTES)?,
            out_cue: take_first_number_of_bytes_as_string(&mut chunk_data, OUT_CUE_LENGTH_IN_BYTES)?,
            start_date: take_first_number_of_bytes_as_string(&mut chunk_data, START_DATE_LENGTH_IN_BYTES)?,
            start_time: take_first_number_of_bytes_as_string(&mut chunk_data, START_TIME_LENGTH_IN_BYTES)?,
            end_date: take_first_number_of_bytes_as_string(&mut chunk_data, END_DATE_LENGTH_IN_BYTES)?,
            end_time: take_first_number_of_bytes_as_string(&mut chunk_data, END_TIME_LENGTH_IN_BYTES)?,
            producer_app_id: take_first_number_of_bytes_as_string(&mut chunk_data, PRODUCER_APP_ID_LENGTH_IN_BYTES)?,
            producer_app_version: take_first_number_of_bytes_as_string(
                &mut chunk_data,
                PRODUCER_APP_VERSION_LENGTH_IN_BYTES,
            )?,
            user_def: take_first_number_of_bytes_as_string(&mut chunk_data, USER_DEF_LENGTH_IN_BYTES)?,
            dw_level_reference: take_first_four_bytes_as_signed_integer(&mut chunk_data, Endian::Little)?,
            post_timer: get_post_timer_from_bytes(take_first_number_of_bytes(
                &mut chunk_data,
                POST_TIMER_LENGTH_IN_BYTES,
            )?)?,
            reserved: take_first_number_of_bytes_as_string(&mut chunk_data, RESERVED_LENGTH_IN_BYTES)?,
            url: take_first_number_of_bytes_as_string(&mut chunk_data, URL_LENGTH_IN_BYTES)?,
            tag_text: take_first_number_of_bytes_as_string(&mut chunk_data, TAG_TEXT_LENGTH_IN_BYTES)?,
        })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        let wave_output_values: Value = upon::value! {
           version: get_formated_version_from_version_string(self.version.clone()),
            title: &self.title,
            artist: &self.artist,
            cue_id: &self.cue_id,
            client_id: &self.client_id,
            category: &self.category,
            classification: &self.classification,
            out_cue: &self.out_cue,
            start_date: &self.start_date,
            start_time: &self.start_time,
            end_date: &self.end_date,
            end_time: &self.end_time,
            producer_app_id: &self.producer_app_id,
            producer_app_version: &self.producer_app_version,
            user_def: &self.user_def,
            dw_level_reference: self.dw_level_reference,
            url: &self.url,
            tag_text: &self.tag_text,
            reserved: &self.reserved,
            post_timer: &self.post_timer,
        };

        let formated_output = template.get_wave_chunk_output(TEMPLATE_NAME, TEMPLATE_CONTENT, wave_output_values)?;
        Ok(formated_output)
    }
}

fn get_post_timer_from_bytes(mut post_timer_data: Vec<u8>) -> Result<Vec<CartTimer>, LocalError> {
    let mut post_timer: Vec<CartTimer> = vec![];

    for _ in 0..NUMBER_OF_POST_TIMERS_PER_TIMER {
        let dw_usage = take_first_number_of_bytes_as_string(&mut post_timer_data, DW_USAGE_LENGTH_IN_BYTES)?;
        let dw_value = take_first_four_bytes_as_unsigned_integer(&mut post_timer_data, Endian::Little)?;

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
        assert_eq!(
            get_formated_version_from_version_string("0234".to_string()),
            "2.34".to_string()
        );
    }

    #[test]
    fn returns_the_passed_version_unaltered_if_it_is_less_than_3_digits() {
        assert_eq!(
            get_formated_version_from_version_string("34".to_string()),
            "34".to_string()
        );
    }
}
