use crate::byte_arrays::{
    take_first_byte_as_unsigned_integer, take_first_four_bytes_as_unsigned_integer,
    take_first_number_of_bytes, Endian,
};
use crate::errors::LocalError;
use crate::formating::{format_midi_note_number_as_note_name, format_smpte_offset};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use serde::Serialize;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/smpl.tmpl");
const MANUFACTURER_ID_LENGTH_IN_BYTES: usize = 4;

#[derive(Serialize)]
struct SampleLoops {
    cue_point_id: u32,
    loop_type: u32,
    start_point: u32,
    end_point: u32,
    fraction: u32,
    number_of_time_to_play_the_loop: u32,
}

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let mut sample_loops: Vec<SampleLoops> = vec![];

    let manufacturer = format_manufacturer_id(take_first_number_of_bytes(
        &mut chunk_data,
        MANUFACTURER_ID_LENGTH_IN_BYTES,
    )?)?;
    let product = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
    let sample_period = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
    let midi_unity_note = format_midi_note_number_as_note_name(
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?,
    );
    let midi_pitch_fraction =
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
    let smpte_format = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
    let smpte_offset = format_smpte_offset(&mut chunk_data, Endian::Little)?;
    let number_of_sample_loops =
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
    let sample_data_size_in_bytes =
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;

    for _ in 0..number_of_sample_loops {
        sample_loops.push(SampleLoops {
            cue_point_id: take_first_four_bytes_as_unsigned_integer(
                &mut chunk_data,
                Endian::Little,
            )?,
            loop_type: take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?,
            start_point: take_first_four_bytes_as_unsigned_integer(
                &mut chunk_data,
                Endian::Little,
            )?,
            end_point: take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?,
            fraction: take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?,
            number_of_time_to_play_the_loop: take_first_four_bytes_as_unsigned_integer(
                &mut chunk_data,
                Endian::Little,
            )?,
        })
    }

    let wave_output_values: Value = upon::value! {
        manufacturer:  &manufacturer,
        product:  product,
        sample_period:  sample_period,
        midi_unity_note:  &midi_unity_note,
        midi_pitch_fraction:  midi_pitch_fraction,
        smpte_format:  smpte_format,
        smpt_offset:  &smpte_offset,
        number_of_sample_loops:  number_of_sample_loops,
        sample_data_size_in_bytes:  sample_data_size_in_bytes,
        sample_loops: &sample_loops,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, wave_output_values)?;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}

fn format_manufacturer_id(mut bytes: Vec<u8>) -> Result<String, LocalError> {
    let manufacturer_id_bytes: Vec<u8> =
        match take_first_byte_as_unsigned_integer(&mut bytes, Endian::Little) {
            Ok(id_length) => bytes.drain(0..id_length as usize).collect(),
            Err(e) => return Err(e),
        };

    let mut manufacturer_id: Vec<String> = vec![];

    manufacturer_id_bytes
        .into_iter()
        .for_each(|byte| manufacturer_id.push(format!("{:0>2X?}H", byte)));

    Ok(manufacturer_id.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_1_byte_manufacturer_id_when_first_byte_is_1() {
        let test_manufacturer_id_bytes = vec![0x01, 0x2A, 0x03, 0x04];
        let correct_id = "2AH";
        let id = format_manufacturer_id(test_manufacturer_id_bytes).unwrap();
        assert_eq!(id, correct_id);
    }

    #[test]
    fn return_3_byte_manufacturer_id_when_first_byte_is_3() {
        let test_manufacturer_id_bytes = vec![0x03, 0x2A, 0x03, 0x04, 0x05];
        let correct_id = "2AH 03H 04H";
        let id = format_manufacturer_id(test_manufacturer_id_bytes).unwrap();
        assert_eq!(id, correct_id);
    }
}
