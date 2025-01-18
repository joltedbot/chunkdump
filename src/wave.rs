mod data;
mod extra;
mod fact;
mod fmt;

use crate::bytes::{read_bytes_from_file, read_bytes_from_file_as_string, skip_over_bytes_in_file};
use crate::errors::LocalError;
use crate::wave::data::read_data_chunk_fields;
use crate::wave::extra::read_extra_chunk_fields;
use crate::wave::fact::read_fact_chunk_fields;
use crate::wave::fmt::{read_fmt_chunk_fields, FmtFields};

use std::error::Error;
use std::fs::File;
use std::path::Path;

const WAVEID_FIELD_LENGTH_IN_BYTES: usize = 4;
const CHUNKID_FIELD_LENGTH_IN_BYTES: usize = 4;
const RIFF_CKSIZE_FIELD_LENGTH_IN_BYTES: i64 = 4;
const WAVEID_IN_DECIMAL_BYTES: [u8; 4] = [87, 65, 86, 69];
const FMT_CHUNK_CHUNKID: &str = "fmt ";
const FACT_CHUNK_CHUNKID: &str = "fact";
const DATA_CHUNK_CHUNKID: &str = "data ";

#[derive(Debug, Clone)]
pub struct Wave {
    pub name: String,
    pub canonical_path: String,
    pub size_in_bytes: u64,
    pub number_of_samples_per_channel: u32,
    pub format_data: FmtFields,
    pub extra_data: Vec<Vec<u8>>,
}

impl Wave {
    pub fn new(file_path: String, mut wave_file: File) -> Result<Self, Box<dyn Error>> {
        skip_riff_cksize_field(&mut wave_file)?;

        if !is_valid_wave_id(&mut wave_file)? {
            return Err(Box::new(LocalError::InvalidWaveID));
        }

        let mut number_of_samples_per_channel = Default::default();
        let mut format_data: FmtFields = Default::default();
        let mut extra_data: Vec<Vec<u8>> = Default::default();

        loop {
            let next_chunkid =
                match read_bytes_from_file_as_string(&mut wave_file, CHUNKID_FIELD_LENGTH_IN_BYTES)
                {
                    Ok(chunkid) => chunkid,
                    Err(_) => break,
                };

            match next_chunkid.as_str() {
                FMT_CHUNK_CHUNKID => format_data = read_fmt_chunk_fields(&mut wave_file)?,
                FACT_CHUNK_CHUNKID => {
                    number_of_samples_per_channel = read_fact_chunk_fields(&mut wave_file)?
                }
                DATA_CHUNK_CHUNKID => read_data_chunk_fields(&mut wave_file)?,
                _ => extra_data.push(read_extra_chunk_fields(&mut wave_file)?),
            }
        }

        let name = get_file_name_from_file_path(&file_path)?;
        let canonical_path = canonicalize_file_path(&file_path)?;
        let size_in_bytes = wave_file.metadata()?.len();

        Ok(Self {
            name,
            canonical_path,
            size_in_bytes,
            number_of_samples_per_channel,
            format_data,
            extra_data,
        })
    }
}

fn canonicalize_file_path(file_path: &String) -> Result<String, Box<dyn Error>> {
    let path = Path::new(file_path).canonicalize()?;

    let canonical_path = match path.to_str() {
        Some(full_path) => full_path.to_string(),
        None => return Err(Box::new(LocalError::InvalidPath)),
    };

    Ok(canonical_path)
}

fn get_file_name_from_file_path(file_path: &String) -> Result<String, Box<dyn Error>> {
    let path = Path::new(file_path);

    let file_name = match path.file_name() {
        Some(file_name) => file_name.to_string_lossy().to_string(),
        None => return Err(Box::new(LocalError::InvalidFileName)),
    };

    Ok(file_name)
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
    use std::env;

    #[test]
    fn canonicalize_file_path_correct_result() {
        let correct_result =
            env::current_dir().unwrap().to_str().unwrap().to_string() + "/src/main.rs";
        let result = canonicalize_file_path(&"./src/main.rs".to_string()).unwrap();

        assert_eq!(result, correct_result);
    }

    #[test]
    fn get_file_name_from_file_path_returns_correct_result() {
        let result = get_file_name_from_file_path(&"/test/path/filename".to_string()).unwrap();
        assert_eq!(result, "filename")
    }
}
