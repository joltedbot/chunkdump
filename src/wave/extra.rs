use crate::fileio::{read_bytes_from_file_as_lossy_string, read_chunk_size_from_file};
use crate::template::Template;
use serde::Serialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Clone, Default, Serialize)]
pub struct ExtraChunks {
    pub chunks: Vec<Chunk>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Chunk {
    pub id: String,
    pub data: String,
}

impl ExtraChunks {
    pub fn new() -> Self {
        Self {
            chunks: Default::default(),
        }
    }

    pub fn add_chunk(&mut self, wave_file: &mut File, chunk_id: String) -> Result<(), Box<dyn Error>> {
        let chunk_size = read_chunk_size_from_file(wave_file)?;
        let chunk_data = read_bytes_from_file_as_lossy_string(wave_file, chunk_size as usize)?;
        self.chunks.push(Chunk {
            id: chunk_id,
            data: chunk_data,
        });

        Ok(())
    }

    pub fn get_metadata_outputs(&self, template: &Template, template_name: &str) -> Result<String, Box<dyn Error>> {
        if self.chunks.is_empty() {
            return Ok("".to_string());
        }

        let extra_chunk_output = &template.get_wave_chunk_output(
            template_name,
            upon::value! {
                extra_chunks: &self.chunks
            },
        )?;

        Ok(extra_chunk_output.to_string())
    }
}
