use crate::bytes::{read_four_byte_integer_from_file, skip_over_bytes_in_file};
use std::error::Error;
use std::fs::File;

const CUE_CKSIZE_FIELD_LENGTH_IN_BYTES: i64 = 4;

#[derive(Debug, Clone, Default)]
pub struct CueFields {
    pub number_of_cue_points: u32,
    pub cue_points: Vec<CuePoint>,
}

#[derive(Debug, Clone, Default)]
pub struct CuePoint {
    pub id: u32,
    pub position: u32,
    pub data_chunk_id: u32,
    pub chunk_start: u32,
    pub block_start: u32,
    pub sample_start: u32,
}

pub fn read_cue_chunk_fields(wave_file: &mut File) -> Result<CueFields, Box<dyn Error>> {
    skip_over_bytes_in_file(wave_file, CUE_CKSIZE_FIELD_LENGTH_IN_BYTES)?;
    let number_of_cue_points = read_four_byte_integer_from_file(wave_file)?;

    let mut cue_points: Vec<CuePoint> = vec![];

    for cue_point in 0..number_of_cue_points {
        cue_points.push(CuePoint {
            id: read_four_byte_integer_from_file(wave_file)?,
            position: read_four_byte_integer_from_file(wave_file)?,
            data_chunk_id: read_four_byte_integer_from_file(wave_file)?,
            chunk_start: read_four_byte_integer_from_file(wave_file)?,
            block_start: read_four_byte_integer_from_file(wave_file)?,
            sample_start: read_four_byte_integer_from_file(wave_file)?,
        })
    }

    Ok(CueFields {
        number_of_cue_points,
        cue_points,
    })
}
