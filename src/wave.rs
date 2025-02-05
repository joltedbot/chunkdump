use crate::chunk::Chunk;
use crate::errors::LocalError;
use crate::fileio::{
    canonicalize_file_path, get_file_name_from_file_path, read_bytes_from_file, read_bytes_from_file_as_string,
    read_chunk_size_from_file, skip_over_bytes_in_file,
};
use crate::template::Template;

use byte_unit::{Byte, UnitType};
use std::error::Error;
use std::fs::File;
use upon::Value;

const TEMPLATE_NAME: &str = "riff";
const TEMPLATE_PATH: &str = include_str!("templates/wave/riff.tmpl");
const WAVEID_FIELD_LENGTH_IN_BYTES: usize = 4;
const CHUNKID_FIELD_LENGTH_IN_BYTES: usize = 4;
const RIFF_CKSIZE_FIELD_LENGTH_IN_BYTES: usize = 4;
const WAVEID_IN_DECIMAL_LITTLE_ENDIAN_BYTES: [u8; 4] = [87, 65, 86, 69];

#[derive(Default)]
pub struct Wave {
    pub template_name: &'static str,
    pub template_path: &'static str,
    pub original_file_path: String,
    pub name: String,
    pub canonical_path: String,
    pub size_in_bytes: u64,
    chunks: Chunk,
}

impl Wave {
    pub fn new(file_path: String, mut wave_file: File) -> Result<Self, Box<dyn Error>> {
        skip_over_bytes_in_file(&mut wave_file, RIFF_CKSIZE_FIELD_LENGTH_IN_BYTES)?;

        let wave_id_bytes = read_bytes_from_file(&mut wave_file, WAVEID_FIELD_LENGTH_IN_BYTES)?;
        if wave_id_bytes != WAVEID_IN_DECIMAL_LITTLE_ENDIAN_BYTES {
            return Err(Box::new(LocalError::InvalidWaveID));
        }

        let mut new_wave: Self = extract_file_metadata(file_path, wave_file)?;

        new_wave.template_name = TEMPLATE_NAME;
        new_wave.template_path = TEMPLATE_PATH;

        Ok(new_wave)
    }

    pub fn display_wave_file_metadata(&self, mut template: Template) -> Result<(), Box<dyn Error>> {
        println!("{}", self.format_data_for_output(&mut template)?);

        let output_lines = self.chunks.format_data_for_output(&mut template)?;

        for line in output_lines {
            println!("{}", line);
        }

        Ok(())
    }

    fn format_data_for_output(&self, template: &mut Template) -> Result<String, Box<dyn Error>> {
        template.add_chunk_template(self.template_name, self.template_path)?;

        let wave_output_values: Value = upon::value! {
            file_name: self.name.clone(),
            file_path: self.original_file_path.clone(),

            file_size: format_file_size_as_string(self.size_in_bytes),
            chunk_ids_found: self.chunks.found_chunk_ids.join(", "),
        };

        let wave_metadata_output = template.get_wave_chunk_output(self.template_name, wave_output_values)?;
        Ok(wave_metadata_output)
    }

    fn parse_wave_chunks(&mut self, wave_file: &mut File) -> Result<(), Box<dyn Error>> {
        loop {
            let chunk_id: String = match read_bytes_from_file_as_string(wave_file, CHUNKID_FIELD_LENGTH_IN_BYTES) {
                Ok(chunkid) => chunkid.to_lowercase(),
                Err(_) => break,
            };

            let chunk_size = read_chunk_size_from_file(wave_file)?;
            let mut chunk_data: Vec<u8> = vec![];

            if self.chunks.ignore_data_for_chunks.contains(&chunk_id.as_str()) {
                skip_over_bytes_in_file(wave_file, chunk_size)?;
            } else {
                chunk_data = read_bytes_from_file(wave_file, chunk_size)?;
            }

            self.chunks.add_chunk(chunk_id, chunk_data)?;
        }

        Ok(())
    }
}

fn extract_file_metadata(file_path: String, mut wave_file: File) -> Result<Wave, Box<dyn Error>> {
    let mut new_wave: Wave = Default::default();
    new_wave.original_file_path = file_path.clone();
    new_wave.name = get_file_name_from_file_path(&file_path)?;
    new_wave.canonical_path = canonicalize_file_path(&file_path)?;
    new_wave.size_in_bytes = wave_file.metadata()?.len();
    new_wave.chunks = Chunk::new(new_wave.canonical_path.clone());

    new_wave.parse_wave_chunks(&mut wave_file)?;

    Ok(new_wave)
}

pub fn add_one_if_byte_size_is_odd(mut byte_size: u32) -> u32 {
    if byte_size % 2 > 0 {
        byte_size += 1;
    }

    byte_size
}

fn format_file_size_as_string(file_size_in_bytes: u64) -> String {
    format!(
        "{:#.2}",
        Byte::from_u64(file_size_in_bytes).get_appropriate_unit(UnitType::Binary)
    )
}
