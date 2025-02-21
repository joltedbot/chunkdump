use crate::byteio::{
    take_first_four_bytes_as_float, take_first_four_bytes_as_unsigned_integer,
    take_first_two_bytes_as_unsigned_integer, Endian,
};

use crate::errors::LocalError;
use crate::midi::note_name_from_midi_note_number;
use crate::template::Template;
use upon::Value;

const TEMPLATE_NAME: &str = "acid";
const TEMPLATE_CONTENT: &str = include_str!("../templates/wave/acid.tmpl");

const ONE_SHOT_FILE_TYPE_MESSAGE: &str = "OneShot";
const LOOP_FILE_TYPE_MESSAGE: &str = "Loop";
const ROOT_NOTE_SET_FILE_TYPE_MESSAGE: &str = "Root Note Set";
const ROOT_NOTE_NOT_SET_FILE_TYPE_MESSAGE: &str = "Root Note Not Set";
const STRETCH_ON_FILE_TYPE_MESSAGE: &str = "Stretch is On";
const STRETCH_OFF_FILE_TYPE_MESSAGE: &str = "Stretch is Off";
const DISK_FILE_TYPE_MESSAGE: &str = "Disk Based";
const RAM_FILE_TYPE_MESSAGE: &str = "Ram Based";
const ACIDIZER_ON_FILE_TYPE_MESSAGE: &str = "Acidizer is On";
const ACIDIZER_OFF_FILE_TYPE_MESSAGE: &str = "Acidizer is Off";

const FILE_TYPE_BIT_POSITION: u8 = 0;
const ROOT_NOTE_BIT_POSITION: u8 = 1;
const STRETCH_BIT_POSITION: u8 = 2;
const DISK_BASED_BIT_POSITION: u8 = 3;
const ACIDIZER_BIT_POSITION: u8 = 4;

#[derive(Debug, Clone, PartialEq, Default)]
struct FileType {
    one_shot: bool,
    root_note: bool,
    stretch: bool,
    disk_based: bool,
    acidizer: bool,
}

#[derive(Debug, Clone, Default)]
pub struct AcidFields {
    template_name: &'static str,
    template_content: &'static str,
    file_type: FileType,
    root_note: String,
    mystery_one: u16,
    mystery_two: f32,
    number_of_beats: u32,
    meter_denominator: u16,
    meter_numerator: u16,
    tempo: f32,
}

impl AcidFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, LocalError> {
        Ok(Self {
            template_name: TEMPLATE_NAME,
            template_content: TEMPLATE_CONTENT,
            file_type: get_file_type_from_bytes(take_first_four_bytes_as_unsigned_integer(
                &mut chunk_data,
                Endian::Little,
            )?),
            root_note: note_name_from_midi_note_number(take_first_two_bytes_as_unsigned_integer(
                &mut chunk_data,
                Endian::Little,
            )? as u32),
            mystery_one: take_first_two_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?,
            mystery_two: take_first_four_bytes_as_float(&mut chunk_data)?,
            number_of_beats: take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?,
            meter_denominator: take_first_two_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?,
            meter_numerator: take_first_two_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?,
            tempo: take_first_four_bytes_as_float(&mut chunk_data)?,
        })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        template.add_chunk_template(self.template_name, self.template_content)?;

        let loop_on = match self.file_type.one_shot {
            true => ONE_SHOT_FILE_TYPE_MESSAGE,
            false => LOOP_FILE_TYPE_MESSAGE,
        };

        let root_note_set = match self.file_type.root_note {
            true => ROOT_NOTE_SET_FILE_TYPE_MESSAGE,
            false => ROOT_NOTE_NOT_SET_FILE_TYPE_MESSAGE,
        };

        let stretch = match self.file_type.stretch {
            true => STRETCH_ON_FILE_TYPE_MESSAGE,
            false => STRETCH_OFF_FILE_TYPE_MESSAGE,
        };

        let disk_based = match self.file_type.disk_based {
            true => DISK_FILE_TYPE_MESSAGE,
            false => RAM_FILE_TYPE_MESSAGE,
        };

        let acidizer = match self.file_type.acidizer {
            true => ACIDIZER_ON_FILE_TYPE_MESSAGE,
            false => ACIDIZER_OFF_FILE_TYPE_MESSAGE,
        };

        let wave_output_values: Value = upon::value! {
            loop_on: loop_on,
            root_note_set: root_note_set,
            stretch: stretch,
            disk_based: disk_based,
            acidizer: acidizer,
            root_note: &self.root_note,
            mystery_one: self.mystery_one,
            mystery_two: self.mystery_two,
            number_of_beats: self.number_of_beats,
            meter_denominator: self.meter_denominator,
            meter_numerator: self.meter_numerator,
            tempo: format!("{:2}", self.tempo),
        };

        let formated_output =
            template.get_wave_chunk_output(self.template_name, self.template_content, wave_output_values)?;
        Ok(formated_output)
    }
}

fn get_file_type_from_bytes(file_type: u32) -> FileType {
    FileType {
        one_shot: check_bit_mask_position(file_type, FILE_TYPE_BIT_POSITION),
        root_note: check_bit_mask_position(file_type, ROOT_NOTE_BIT_POSITION),
        stretch: check_bit_mask_position(file_type, STRETCH_BIT_POSITION),
        disk_based: check_bit_mask_position(file_type, DISK_BASED_BIT_POSITION),
        acidizer: check_bit_mask_position(file_type, ACIDIZER_BIT_POSITION),
    }
}

fn check_bit_mask_position(bit_mask: u32, position: u8) -> bool {
    if (bit_mask & (1 << position)) > 0 {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_file_type_struct_is_returned_from_file_type_integers() {
        assert_eq!(
            get_file_type_from_bytes(0),
            FileType {
                one_shot: false,
                root_note: false,
                stretch: false,
                disk_based: false,
                acidizer: false,
            }
        );

        assert_eq!(
            get_file_type_from_bytes(31),
            FileType {
                one_shot: true,
                root_note: true,
                stretch: true,
                disk_based: true,
                acidizer: true,
            }
        );
    }

    #[test]
    fn check_bit_mask_position_is_returned_from_file_type_integers() {
        let correct_disk_based_bit_position_filetype = 8;
        let incorrect_disk_based_bit_position_filetype = 7;
        assert!(check_bit_mask_position(
            correct_disk_based_bit_position_filetype,
            DISK_BASED_BIT_POSITION
        ));
        assert!(!check_bit_mask_position(
            incorrect_disk_based_bit_position_filetype,
            DISK_BASED_BIT_POSITION
        ));
    }
}
