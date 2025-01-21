mod cue;
mod data;
mod extra;
mod fact;
mod fmt;
mod junk;
mod list;
mod resu;

use crate::errors::LocalError;
use crate::fileio::{
    canonicalize_file_path, get_file_name_from_file_path, read_bytes_from_file,
    read_bytes_from_file_as_string, skip_over_bytes_in_file,
};
use crate::wave::cue::{read_cue_chunk, CueFields};
use crate::wave::data::skip_data_chunk;
use crate::wave::extra::read_extra_chunk_fields;
use crate::wave::fact::read_fact_chunk;
use crate::wave::fmt::{read_fmt_chunk, FmtFields};
use crate::wave::junk::read_junk_chunk;
use crate::wave::list::read_list_chunk_fields;
use crate::wave::resu::read_resu_chunk;

use std::error::Error;
use std::fs::File;

const WAVEID_FIELD_LENGTH_IN_BYTES: usize = 4;
const CHUNKID_FIELD_LENGTH_IN_BYTES: usize = 4;
const RIFF_CKSIZE_FIELD_LENGTH_IN_BYTES: i64 = 4;
const WAVEID_IN_DECIMAL_BYTES: [u8; 4] = [87, 65, 86, 69];
const FMT_CHUNKID: &str = "fmt ";
const FACT_CHUNKID: &str = "fact";
const DATA_CHUNKID: &str = "data";
const CUE_CHUNKID: &str = "cue ";
const RESU_CHUNKID: &str = "ResU";
const JUNK_CHUNKID: &str = "JUNK";
const LIST_CHUNKID: &str = "LIST";

#[derive(Debug, Clone, Default)]
pub struct Wave {
    pub name: String,
    pub canonical_path: String,
    pub size_in_bytes: u64,
    pub samples_per_channel: u32,
    pub format_data: FmtFields,
    pub resu_data: String,
    pub cue_data: CueFields,
    pub junk_data: String,
    pub list_data: Vec<(String, String)>,
    pub extra_data: Vec<String>,
}

impl Wave {
    pub fn new(file_path: String, mut wave_file: File) -> Result<Self, Box<dyn Error>> {
        skip_riff_cksize_field(&mut wave_file)?;

        if !is_valid_wave_id(&mut wave_file)? {
            return Err(Box::new(LocalError::InvalidWaveID));
        }

        let mut new_wave: Self = Default::default();

        loop {
            let next_chunkid =
                match read_bytes_from_file_as_string(&mut wave_file, CHUNKID_FIELD_LENGTH_IN_BYTES)
                {
                    Ok(chunkid) => chunkid,
                    Err(_) => break,
                };

            match next_chunkid.as_str() {
                JUNK_CHUNKID => new_wave.junk_data = read_junk_chunk(&mut wave_file)?,
                FMT_CHUNKID => new_wave.format_data = read_fmt_chunk(&mut wave_file)?,
                FACT_CHUNKID => new_wave.samples_per_channel = read_fact_chunk(&mut wave_file)?,
                DATA_CHUNKID => skip_data_chunk(&mut wave_file)?,
                CUE_CHUNKID => new_wave.cue_data = read_cue_chunk(&mut wave_file)?,
                RESU_CHUNKID => new_wave.resu_data = read_resu_chunk(&mut wave_file)?,
                LIST_CHUNKID => new_wave.list_data = read_list_chunk_fields(&mut wave_file)?,
                _ => new_wave
                    .extra_data
                    .push(read_extra_chunk_fields(&mut wave_file)?),
            }
        }

        new_wave.name = get_file_name_from_file_path(&file_path)?;
        new_wave.canonical_path = canonicalize_file_path(&file_path)?;
        new_wave.size_in_bytes = wave_file.metadata()?.len();

        Ok(new_wave)
    }
}

pub fn take_first_four_bytes_as_integer(byte_data: &mut Vec<u8>) -> Result<u32, Box<dyn Error>> {
    let bytes: Vec<u8> = byte_data.drain(..4).collect();

    let mut chunk_size_array: [u8; 4] = Default::default();
    chunk_size_array.copy_from_slice(bytes.as_slice());

    Ok(u32::from_le_bytes(chunk_size_array))
}

pub fn take_first_two_bytes_as_integer(byte_data: &mut Vec<u8>) -> Result<u16, Box<dyn Error>> {
    let bytes: Vec<u8> = byte_data.drain(..2).collect();

    let mut chunk_size_array: [u8; 2] = Default::default();
    chunk_size_array.copy_from_slice(bytes.as_slice());

    Ok(u16::from_le_bytes(chunk_size_array))
}

fn is_valid_wave_id(wave_file: &mut File) -> Result<bool, Box<dyn Error>> {
    let wave_id_bytes = read_bytes_from_file(wave_file, WAVEID_FIELD_LENGTH_IN_BYTES)?;

    if wave_id_bytes != WAVEID_IN_DECIMAL_BYTES {
        return Err(Box::new(LocalError::InvalidWaveID));
    }

    Ok(true)
}

fn skip_riff_cksize_field(wave_file: &mut File) -> Result<(), Box<dyn Error>> {
    skip_over_bytes_in_file(wave_file, RIFF_CKSIZE_FIELD_LENGTH_IN_BYTES)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_first_four_bytes_as_integer_correct_result() {
        let mut little_endian_test_bytes: Vec<u8> = vec![0x10, 0x01, 0x01, 0x01, 0x01];
        let result_integer: u32 =
            take_first_four_bytes_as_integer(&mut little_endian_test_bytes).unwrap();
        let correct_result: u32 = 16843024;
        assert_eq!(result_integer, correct_result);
    }

    #[test]
    fn take_first_two_bytes_as_integer_correct_result() {
        let mut little_endian_test_bytes: Vec<u8> = vec![0x10, 0x01, 0x01, 0x01, 0x01];
        let result_integer: u16 =
            take_first_two_bytes_as_integer(&mut little_endian_test_bytes).unwrap();
        let correct_result: u16 = 272;
        assert_eq!(result_integer, correct_result);
    }
}
