use crate::byteio::take_first_four_bytes_as_integer;
use crate::fileio::{read_bytes_from_file, read_four_byte_integer_from_file};
use std::error::Error;
use std::fs::File;

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

pub fn read_cue_chunk(wave_file: &mut File) -> Result<CueFields, Box<dyn Error>> {
    let chunk_size = read_four_byte_integer_from_file(wave_file)?;
    let mut cue_data = read_bytes_from_file(wave_file, chunk_size as usize)?;
    let mut cue_points: Vec<CuePoint> = vec![];
    let number_of_cue_points: u32 = take_first_four_bytes_as_integer(&mut cue_data)?;

    for _ in 0..number_of_cue_points {
        cue_points.push(CuePoint {
            id: take_first_four_bytes_as_integer(&mut cue_data)?,
            position: take_first_four_bytes_as_integer(&mut cue_data)?,
            data_chunk_id: take_first_four_bytes_as_integer(&mut cue_data)?,
            chunk_start: take_first_four_bytes_as_integer(&mut cue_data)?,
            block_start: take_first_four_bytes_as_integer(&mut cue_data)?,
            sample_start: take_first_four_bytes_as_integer(&mut cue_data)?,
        })
    }

    Ok(CueFields {
        number_of_cue_points,
        cue_points,
    })
}
