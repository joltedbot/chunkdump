use crate::byteio::{
    take_first_four_bytes_as_integer, take_first_four_bytes_float, take_first_two_bytes_as_integer,
};
use crate::fileio::{read_bytes_from_file, read_chunk_size_from_file};
use byte_unit::rust_decimal::prelude::Zero;
use std::error::Error;
use std::fs::File;

const FILE_TYPE_BIT_POSITION: u8 = 0;
const ROOT_NOTE_BIT_POSITION: u8 = 1;
const STRETCH_BIT_POSITION: u8 = 2;
const DISK_BASED_BIT_POSITION: u8 = 3;
const ACIDIZER_BIT_POSITION: u8 = 4;

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

impl AcidData {
    pub fn new(wave_file: &mut File) -> Result<Self, Box<dyn Error>> {
        let chunk_size = read_chunk_size_from_file(wave_file)?;
        let mut acid_data = read_bytes_from_file(wave_file, chunk_size as usize)?;

        Ok(Self {
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

    pub fn get_metadata_output(&self) -> Vec<String> {
        let mut acid_data: Vec<String> = vec![];

        acid_data.push("\n-------------\nACID Chunk Details:\n-------------".to_string());
        acid_data.push("File Type:\n-------------".to_string());

        match self.file_type.one_shot {
            true => acid_data.push(" > OneShot".to_string()),
            false => acid_data.push(" > Loop".to_string()),
        };

        match self.file_type.root_note {
            true => acid_data.push(" > Root Note Set".to_string()),
            false => acid_data.push(" > Root Note Not Set".to_string()),
        }

        match self.file_type.stretch {
            true => acid_data.push(" > Stretch is On".to_string()),
            false => acid_data.push(" > Stretch is Off".to_string()),
        }

        match self.file_type.disk_based {
            true => acid_data.push(" > Disk Based".to_string()),
            false => acid_data.push(" > Ram Based".to_string()),
        }

        match self.file_type.acidizer {
            true => acid_data.push(" > Acidizer is On".to_string()),
            false => acid_data.push(" > Acidizer is Off".to_string()),
        }

        acid_data.push("-------------".to_string());

        if !self.root_note.is_zero() {
            acid_data.push(format!("Root Note: {:#?}", self.root_note));
        }

        if !self.mystery_one.is_zero() {
            acid_data.push(format!("Mystery Value One: {:#?}", self.mystery_one));
        }

        if !self.mystery_two.is_zero() {
            acid_data.push(format!("Mystery Value Two: {:#?}", self.mystery_two));
        }

        acid_data.push(format!(
            "Time Signature (Likely Incorrect): {}/{}",
            self.meter_numerator, self.meter_denominator
        ));

        if !self.number_of_beats.is_zero() {
            acid_data.push(format!(
                "Number of Beats: {}",
                self.number_of_beats.to_string()
            ));
        }

        if !self.tempo.is_zero() {
            acid_data.push(format!("Tempo: {}bpm", self.tempo.to_string()));
        }

        acid_data
    }
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
