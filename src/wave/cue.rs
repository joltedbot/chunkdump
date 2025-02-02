use crate::byteio::{take_first_four_bytes_as_unsigned_integer, take_first_number_of_bytes_as_string};
use crate::fileio::{read_bytes_from_file, read_chunk_size_from_file};
use crate::template::Template;
use serde::Serialize;
use std::error::Error;
use std::fs::File;

const DATA_CHUNK_ID_LENGTH_IN_BYTES: usize = 4;

#[derive(Debug, Clone, Default)]
pub struct CueFields {
    pub number_of_cue_points: u32,
    pub cue_points: Vec<CuePoint>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CuePoint {
    pub id: u32,
    pub position: u32,
    pub data_chunk_id: String,
    pub chunk_start: u32,
    pub block_start: u32,
    pub sample_start: u32,
}

impl CueFields {
    pub fn new(wave_file: &mut File) -> Result<Self, Box<dyn Error>> {
        let chunk_size = read_chunk_size_from_file(wave_file)?;
        let mut cue_data = read_bytes_from_file(wave_file, chunk_size as usize)?;
        let mut cue_points: Vec<CuePoint> = vec![];
        let number_of_cue_points: u32 = take_first_four_bytes_as_unsigned_integer(&mut cue_data)?;

        for _ in 0..number_of_cue_points {
            cue_points.push(CuePoint {
                id: take_first_four_bytes_as_unsigned_integer(&mut cue_data)?,
                position: take_first_four_bytes_as_unsigned_integer(&mut cue_data)?,
                data_chunk_id: take_first_number_of_bytes_as_string(&mut cue_data, DATA_CHUNK_ID_LENGTH_IN_BYTES)?,
                chunk_start: take_first_four_bytes_as_unsigned_integer(&mut cue_data)?,
                block_start: take_first_four_bytes_as_unsigned_integer(&mut cue_data)?,
                sample_start: take_first_four_bytes_as_unsigned_integer(&mut cue_data)?,
            })
        }

        Ok(Self {
            number_of_cue_points,
            cue_points,
        })
    }

    pub fn get_metadata_output(&self, template: &Template, template_name: &str) -> Result<String, Box<dyn Error>> {
        let cue_output: String = template.get_wave_chunk_output(
            template_name,
            upon::value! {
                number_of_cue_points: &self.number_of_cue_points,
                cue_points: &self.cue_points
            },
        )?;
        Ok(cue_output)
    }
}
