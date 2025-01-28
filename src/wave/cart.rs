use crate::byteio::{
    take_first_eight_bytes_as_integer, take_first_four_bytes_as_integer,
    take_first_number_of_bytes, take_first_number_of_bytes_as_string,
};
use crate::fileio::{read_bytes_from_file, read_four_byte_integer_from_file};
use std::error::Error;
use std::fs::File;

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

#[derive(Debug, Clone, Default)]
pub struct CartTimer {
    pub dw_usage: String,
    pub dw_value: u32,
}

#[derive(Debug, Clone, Default)]
pub struct CartData {
    pub version: String,
    pub title: String,
    pub artist: String,
    pub cue_id: String,
    pub client_id: String,
    pub category: String,
    pub classification: String,
    pub out_cue: String,
    pub start_date: String,
    pub start_time: String,
    pub end_date: String,
    pub end_time: String,
    pub producer_app_id: String,
    pub producer_app_version: String,
    pub user_def: String,
    pub dw_level_reference: u64,
    pub post_timer: Vec<CartTimer>,
    pub reserved: String,
    pub url: String,
    pub tag_text: String,
}

impl CartData {
    pub fn new(wave_file: &mut File) -> Result<Self, Box<dyn Error>> {
        let chunk_size = read_four_byte_integer_from_file(wave_file)?;
        let mut cart_data = read_bytes_from_file(wave_file, chunk_size as usize)?;

        Ok(Self {
            version: take_first_number_of_bytes_as_string(&mut cart_data, VERSION_LENGTH_IN_BYTES)?,
            title: take_first_number_of_bytes_as_string(&mut cart_data, TITLE_LENGTH_IN_BYTES)?,
            artist: take_first_number_of_bytes_as_string(&mut cart_data, ARTIST_LENGTH_IN_BYTES)?,
            cue_id: take_first_number_of_bytes_as_string(&mut cart_data, CUE_ID_LENGTH_IN_BYTES)?,
            client_id: take_first_number_of_bytes_as_string(
                &mut cart_data,
                CLIENT_ID_LENGTH_IN_BYTES,
            )?,
            category: take_first_number_of_bytes_as_string(
                &mut cart_data,
                CATEGORY_LENGTH_IN_BYTES,
            )?,
            classification: take_first_number_of_bytes_as_string(
                &mut cart_data,
                CLASSIFICATION_LENGTH_IN_BYTES,
            )?,
            out_cue: take_first_number_of_bytes_as_string(&mut cart_data, OUT_CUE_LENGTH_IN_BYTES)?,
            start_date: take_first_number_of_bytes_as_string(
                &mut cart_data,
                START_DATE_LENGTH_IN_BYTES,
            )?,
            start_time: take_first_number_of_bytes_as_string(
                &mut cart_data,
                START_TIME_LENGTH_IN_BYTES,
            )?,
            end_date: take_first_number_of_bytes_as_string(
                &mut cart_data,
                END_DATE_LENGTH_IN_BYTES,
            )?,
            end_time: take_first_number_of_bytes_as_string(
                &mut cart_data,
                END_TIME_LENGTH_IN_BYTES,
            )?,
            producer_app_id: take_first_number_of_bytes_as_string(
                &mut cart_data,
                PRODUCER_APP_ID_LENGTH_IN_BYTES,
            )?,
            producer_app_version: take_first_number_of_bytes_as_string(
                &mut cart_data,
                PRODUCER_APP_VERSION_LENGTH_IN_BYTES,
            )?,
            user_def: take_first_number_of_bytes_as_string(
                &mut cart_data,
                USER_DEF_LENGTH_IN_BYTES,
            )?,
            dw_level_reference: take_first_eight_bytes_as_integer(&mut cart_data)?,
            post_timer: get_post_timer_from_bytes(take_first_number_of_bytes(
                &mut cart_data,
                POST_TIMER_LENGTH_IN_BYTES,
            )?)?,
            reserved: take_first_number_of_bytes_as_string(
                &mut cart_data,
                RESERVED_LENGTH_IN_BYTES,
            )?,
            url: take_first_number_of_bytes_as_string(&mut cart_data, URL_LENGTH_IN_BYTES)?,
            tag_text: take_first_number_of_bytes_as_string(
                &mut cart_data,
                TAG_TEXT_LENGTH_IN_BYTES,
            )?,
        })
    }
}
fn get_post_timer_from_bytes(
    mut post_timer_data: Vec<u8>,
) -> Result<Vec<CartTimer>, Box<dyn Error>> {
    let mut post_timer: Vec<CartTimer> = vec![];

    for _ in 0..NUMBER_OF_POST_TIMERS_PER_TIMER {
        let dw_usage =
            take_first_number_of_bytes_as_string(&mut post_timer_data, DW_USAGE_LENGTH_IN_BYTES)?;
        let dw_value = take_first_four_bytes_as_integer(&mut post_timer_data)?;

        if !dw_usage.is_empty() || dw_value != 0 {
            post_timer.push(CartTimer { dw_usage, dw_value });
        }
    }

    Ok(post_timer)
}
