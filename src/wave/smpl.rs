use crate::byteio::{
    take_first_byte_as_signed_integer, take_first_byte_as_unsigned_integer, take_first_four_bytes_as_unsigned_integer,
    take_first_number_of_bytes, Endian,
};
use crate::errors::LocalError;
use crate::formating::get_note_name_from_midi_note_number;
use crate::template::Template;
use serde::Serialize;
use upon::Value;

const TEMPLATE_NAME: &str = "smpl";
const TEMPLATE_CONTENT: &str = include_str!("../templates/wave/smpl.tmpl");
const MANUFACTURER_ID_LENGTH_IN_BYTES: usize = 4;

#[derive(Debug, Clone, Default, Serialize)]
struct SampleLoops {
    cue_point_id: u32,
    loop_type: u32,
    start_point: u32,
    end_point: u32,
    fraction: u32,
    number_of_time_to_play_the_loop: u32,
}

#[derive(Debug, Clone, Default)]
pub struct SmplFields {
    template_name: &'static str,
    template_content: &'static str,
    manufacturer: String,
    product: u32,
    sample_period: u32,
    midi_unity_note: String,
    midi_pitch_fraction: u32,
    smpte_format: u32,
    smpte_offset: String,
    number_of_sample_loops: u32,
    sample_data_size_in_bytes: u32,
    sample_loops: Vec<SampleLoops>,
}

impl SmplFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, LocalError> {
        let mut sample_loops: Vec<SampleLoops> = vec![];

        let manufacturer = format_manufacturer_id(take_first_number_of_bytes(
            &mut chunk_data,
            MANUFACTURER_ID_LENGTH_IN_BYTES,
        )?)?;
        let product = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
        let sample_period = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
        let midi_unity_note = get_note_name_from_midi_note_number(take_first_four_bytes_as_unsigned_integer(
            &mut chunk_data,
            Endian::Little,
        )?);
        let midi_pitch_fraction = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
        let smpte_format = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
        let smpte_offset = format_smpte_offset(&mut chunk_data)?;
        let number_of_sample_loops = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;
        let sample_data_size_in_bytes = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?;

        for _ in 0..number_of_sample_loops {
            sample_loops.push(SampleLoops {
                cue_point_id: take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?,
                loop_type: take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?,
                start_point: take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?,
                end_point: take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?,
                fraction: take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Little)?,
                number_of_time_to_play_the_loop: take_first_four_bytes_as_unsigned_integer(
                    &mut chunk_data,
                    Endian::Little,
                )?,
            })
        }

        Ok(Self {
            template_name: TEMPLATE_NAME,
            template_content: TEMPLATE_CONTENT,
            manufacturer,
            product,
            sample_period,
            midi_unity_note,
            midi_pitch_fraction,
            smpte_format,
            smpte_offset,
            number_of_sample_loops,
            sample_data_size_in_bytes,
            sample_loops,
        })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        let wave_output_values: Value = upon::value! {
            template_name: self.template_name,
            manufacturer:  &self.manufacturer,
            product:  self.product,
            sample_period:  self.sample_period,
            midi_unity_note:  &self.midi_unity_note,
            midi_pitch_fraction:  self.midi_pitch_fraction,
            smpte_format:  self.smpte_format,
            smpt_offset:  &self.smpte_offset,
            number_of_sample_loops:  self.number_of_sample_loops,
            sample_data_size_in_bytes:  self.sample_data_size_in_bytes,
            sample_loops: &self.sample_loops,
        };

        let formated_output =
            template.get_wave_chunk_output(self.template_name, self.template_content, wave_output_values)?;
        Ok(formated_output)
    }
}

fn format_manufacturer_id(mut bytes: Vec<u8>) -> Result<String, LocalError> {
    let manufacturer_id_bytes: Vec<u8> = match take_first_byte_as_unsigned_integer(&mut bytes, Endian::Little) {
        Ok(id_length) => bytes.drain(0..id_length as usize).collect(),
        Err(e) => return Err(e),
    };

    let mut manufacturer_id: Vec<String> = vec![];

    manufacturer_id_bytes
        .into_iter()
        .for_each(|byte| manufacturer_id.push(format!("{:0>2X?}H", byte)));

    Ok(manufacturer_id.join(" "))
}

fn format_smpte_offset(smpte_offset_bytes: &mut Vec<u8>) -> Result<String, LocalError> {
    let hours = take_first_byte_as_signed_integer(smpte_offset_bytes)?;
    let minutes = take_first_byte_as_unsigned_integer(smpte_offset_bytes, Endian::Little)?;
    let seconds = take_first_byte_as_unsigned_integer(smpte_offset_bytes, Endian::Little)?;
    let samples = take_first_byte_as_unsigned_integer(smpte_offset_bytes, Endian::Little)?;

    Ok(format!("{}h:{}m:{}s & {} samples", hours, minutes, seconds, samples))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_1_byte_manufacturer_id_when_first_byte_is_1() {
        let test_manufacturer_id_bytes = vec![0x01, 0x2A, 0x03, 0x04];
        let id = format_manufacturer_id(test_manufacturer_id_bytes).unwrap();
        assert_eq!(id, "2AH");
    }

    #[test]
    fn return_3_byte_manufacturer_id_when_first_byte_is_3() {
        let test_manufacturer_id_bytes = vec![0x03, 0x2A, 0x03, 0x04, 0x05];
        let id = format_manufacturer_id(test_manufacturer_id_bytes).unwrap();
        assert_eq!(id, "2AH 03H 04H");
    }

    #[test]
    fn returns_the_correct_format_smpte_offset() {
        let mut test_manufacturer_id_bytes = vec![0x01, 0x02, 0x03, 0x04];
        let id = format_smpte_offset(&mut test_manufacturer_id_bytes).unwrap();
        assert_eq!(id, "1h:2m:3s & 4 samples");
    }
}
