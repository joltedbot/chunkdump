use crate::byteio::{
    take_first_four_bytes_as_unsigned_integer, take_first_number_of_bytes_as_string,
};
use crate::fileio::{read_bytes_from_file, read_chunk_size_from_file};
use byte_unit::rust_decimal::prelude::Zero;
use std::error::Error;
use std::fs::File;

const DATA_CHUNK_ID_LENGTH_IN_BYTES: usize = 4;

#[derive(Debug, Clone, Default)]
pub struct CueFields {
    pub number_of_cue_points: u32,
    pub cue_points: Vec<CuePoint>,
}

#[derive(Debug, Clone, Default)]
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
                data_chunk_id: take_first_number_of_bytes_as_string(
                    &mut cue_data,
                    DATA_CHUNK_ID_LENGTH_IN_BYTES,
                )?,
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

    pub fn get_metadata_output(&self) -> Vec<String> {
        let mut cue_data: Vec<String> = vec![];

        cue_data.push("\n-------------\nCue Chunk Details:\n-------------".to_string());
        cue_data.push(format!(
            "Number of Cue Points: {}",
            self.number_of_cue_points
        ));

        for cue_point in &self.cue_points {
            if !cue_point.id.is_zero() {
                cue_data.push("-------------".to_string());
                cue_data.push(format!("Cue Point ID: {}", cue_point.id));
                cue_data.push(format!("Position: Sample {}", cue_point.position));
                cue_data.push(format!("Data Chunk ID: {}", cue_point.data_chunk_id));
                cue_data.push(format!(
                    "Chunk Start: Byte Position {}",
                    cue_point.chunk_start
                ));
                cue_data.push(format!(
                    "Block Start: Byte Position {}",
                    cue_point.block_start
                ));
                cue_data.push(format!(
                    "Sample Start: Byte Position {}",
                    cue_point.sample_start
                ));
            }
            cue_data.push("-------------".to_string());
        }

        cue_data
    }
}
