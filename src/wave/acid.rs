#![allow(dead_code)]

use crate::byteio::{
    take_first_four_bytes_as_integer, take_first_four_bytes_float, take_first_two_bytes_as_integer,
};
use crate::fileio::{read_bytes_from_file, read_four_byte_integer_from_file};
use std::error::Error;
use std::fs::File;

const FILE_TYPE_MASK_NUMBER_OF_BITS: u8 = 5;
const FILE_TYPE_BIT_POSITION: u8 = 1;
const ROOT_NOTE_BIT_POSITION: u8 = 2;
const STRETCH_BIT_POSITION: u8 = 3;
const DISK_BASED_BIT_POSITION: u8 = 4;
const ACIDIZER_BIT_POSITION: u8 = 5;

#[derive(Debug, Clone, Default)]
pub struct FileType {
    pub one_shot: bool,
    pub root_note: bool,
    pub stretch: bool,
    pub disk_based: bool,
    pub acidizer: bool,
}

#[derive(Debug, Clone, Default)]
pub struct AcidData {
    file_type: FileType,
    root_note: u16,
    mystery_one: u16,
    mystery_two: f32,
    number_of_beats: u32,
    meter_denominator: u16,
    meter_numerator: u16,
    tempo: f32,
}

pub fn read_acid_chunk(wave_file: &mut File) -> Result<AcidData, Box<dyn Error>> {
    let chunk_size = read_four_byte_integer_from_file(wave_file)?;
    let mut acid_data = read_bytes_from_file(wave_file, chunk_size as usize)?;

    Ok(AcidData {
        file_type: get_file_type_from_bytes(take_first_four_bytes_as_integer(&mut acid_data)?)?,
        root_note: take_first_two_bytes_as_integer(&mut acid_data)?,
        mystery_one: take_first_two_bytes_as_integer(&mut acid_data)?,
        mystery_two: take_first_four_bytes_float(&mut acid_data)?,
        number_of_beats: take_first_four_bytes_as_integer(&mut acid_data)?,
        meter_denominator: take_first_two_bytes_as_integer(&mut acid_data)?,
        meter_numerator: take_first_two_bytes_as_integer(&mut acid_data)?,
        tempo: take_first_four_bytes_float(&mut acid_data)?,
    })
}

fn get_file_type_from_bytes(file_type: u32) -> Result<FileType, Box<dyn Error>> {
    Ok(FileType {
        one_shot: check_bit_mask_position(file_type, FILE_TYPE_BIT_POSITION),
        root_note: check_bit_mask_position(file_type, ROOT_NOTE_BIT_POSITION),
        stretch: check_bit_mask_position(file_type, STRETCH_BIT_POSITION),
        disk_based: check_bit_mask_position(file_type, DISK_BASED_BIT_POSITION),
        acidizer: check_bit_mask_position(file_type, ACIDIZER_BIT_POSITION),
    })
}

fn check_bit_mask_position(bit_mask: u32, position: u8) -> bool {
    if (bit_mask & (1 << position)) > 0 {
        return true;
    }

    false
}
