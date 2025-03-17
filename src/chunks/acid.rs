use crate::byte_arrays::{
    take_first_four_bytes_as_float, take_first_four_bytes_as_unsigned_integer,
    take_first_two_bytes_as_unsigned_integer, Endian,
};
use crate::formating::format_midi_note_number_as_note_name;
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/acid.tmpl");

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

#[derive(Debug, PartialEq)]
struct FileType {
    one_shot: bool,
    root_note: bool,
    stretch: bool,
    disk_based: bool,
    acidizer: bool,
}

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let file_type = get_file_type_from_file_type_integer(
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?,
    );

    let loop_on = match file_type.one_shot {
        true => ONE_SHOT_FILE_TYPE_MESSAGE,
        false => LOOP_FILE_TYPE_MESSAGE,
    };

    let root_note_set = match file_type.root_note {
        true => ROOT_NOTE_SET_FILE_TYPE_MESSAGE,
        false => ROOT_NOTE_NOT_SET_FILE_TYPE_MESSAGE,
    };

    let stretch = match file_type.stretch {
        true => STRETCH_ON_FILE_TYPE_MESSAGE,
        false => STRETCH_OFF_FILE_TYPE_MESSAGE,
    };

    let disk_based = match file_type.disk_based {
        true => DISK_FILE_TYPE_MESSAGE,
        false => RAM_FILE_TYPE_MESSAGE,
    };

    let acidizer = match file_type.acidizer {
        true => ACIDIZER_ON_FILE_TYPE_MESSAGE,
        false => ACIDIZER_OFF_FILE_TYPE_MESSAGE,
    };

    let root_note = format_midi_note_number_as_note_name(take_first_two_bytes_as_unsigned_integer(
        &mut chunk_data,
        Endian::Little,
    )? as u32);
    let mystery_one = take_first_two_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
    let mystery_two = take_first_four_bytes_as_float(&mut chunk_data)?;
    let number_of_beats =
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
    let meter_denominator =
        take_first_two_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
    let meter_numerator =
        take_first_two_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
    let tempo = take_first_four_bytes_as_float(&mut chunk_data)?;

    let wave_output_values: Value = upon::value! {
        loop_on: loop_on,
        root_note_set: root_note_set,
        stretch: stretch,
        disk_based: disk_based,
        acidizer: acidizer,
        root_note: &root_note,
        mystery_one: mystery_one,
        mystery_two: mystery_two,
        number_of_beats: number_of_beats,
        meter_denominator: meter_denominator,
        meter_numerator: meter_numerator,
        tempo: format!("{:2}", tempo),
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, wave_output_values)?;
    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}

fn get_file_type_from_file_type_integer(file_type: u32) -> FileType {
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
    fn return_correct_file_type_struct_is_returned_from_file_type_integers_when_no_bits_are_set() {
        let correct_result = FileType {
            one_shot: false,
            root_note: false,
            stretch: false,
            disk_based: false,
            acidizer: false,
        };
        let result = get_file_type_from_file_type_integer(0);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_correct_file_type_struct_is_returned_from_file_type_integers_when_all_bits_are_set() {
        let correct_result = FileType {
            one_shot: true,
            root_note: true,
            stretch: true,
            disk_based: true,
            acidizer: true,
        };
        let result = get_file_type_from_file_type_integer(31);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_true_when_a_specific_bit_position_is_set() {
        let test_position: u8 = 3;
        let bit_position_3_set_integer: u32 = 8;
        assert!(check_bit_mask_position(
            bit_position_3_set_integer,
            test_position
        ));
    }

    #[test]
    fn return_false_when_a_specific_bit_position_is_not_set() {
        let test_position: u8 = 3;
        let no_bit_positions_set_integer: u32 = 0;
        assert!(!check_bit_mask_position(
            no_bit_positions_set_integer,
            test_position
        ));
    }
}
