use crate::byteio::{
    take_first_four_bytes_as_float, take_first_four_bytes_as_unsigned_integer, take_first_two_bytes_as_unsigned_integer,
};
use crate::fileio::{read_bytes_from_file, read_chunk_size_from_file};
use crate::template::Template;
use std::error::Error;
use std::fs::File;
use upon::Value;

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
            file_type: get_file_type_from_bytes(take_first_four_bytes_as_unsigned_integer(&mut acid_data)?)?,
            root_note: take_first_two_bytes_as_unsigned_integer(&mut acid_data)?,
            mystery_one: take_first_two_bytes_as_unsigned_integer(&mut acid_data)?,
            mystery_two: take_first_four_bytes_as_float(&mut acid_data)?,
            number_of_beats: take_first_four_bytes_as_unsigned_integer(&mut acid_data)?,
            meter_denominator: take_first_two_bytes_as_unsigned_integer(&mut acid_data)?,
            meter_numerator: take_first_two_bytes_as_unsigned_integer(&mut acid_data)?,
            tempo: take_first_four_bytes_as_float(&mut acid_data)?,
        })
    }

    pub fn get_metadata_outputs(&self, template: &Template, template_name: &str) -> Result<String, Box<dyn Error>> {
        let loop_on = match self.file_type.one_shot {
            true => "OneShot".to_string(),
            false => "Loop".to_string(),
        };

        let root_note_set = match self.file_type.root_note {
            true => "Root Note Set".to_string(),
            false => "Root Note Not Set".to_string(),
        };

        let stretch = match self.file_type.stretch {
            true => "Stretch is On".to_string(),
            false => "Stretch is Off".to_string(),
        };

        let disk_based = match self.file_type.disk_based {
            true => "Disk Based".to_string(),
            false => "Ram Based".to_string(),
        };

        let acidizer = match self.file_type.acidizer {
            true => "Acidizer is On".to_string(),
            false => "Acidizer is Off".to_string(),
        };

        let wave_output_values: Value = upon::value! {
            loop_on: loop_on,
            root_note_set: root_note_set,
            stretch: stretch,
            disk_based: disk_based,
            acidizer: acidizer,
            root_note: self.root_note,
            mystery_one: self.mystery_one,
            mystery_two: self.mystery_two,
            number_of_beats: self.number_of_beats,
            meter_denominator: self.meter_denominator,
            meter_numerator: self.meter_numerator,
            tempo: self.tempo,
        };

        Ok(template.get_wave_chunk_output(template_name, wave_output_values)?)
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
