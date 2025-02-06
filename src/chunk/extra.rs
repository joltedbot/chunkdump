use crate::byteio::take_first_number_of_bytes_as_string;
use crate::template::Template;
use serde::Serialize;
use std::error::Error;
use upon::Value;

const TEMPLATE_NAME: &str = "extra";
const TEMPLATE_PATH: &str = include_str!("../templates/wave/extra.tmpl");

#[derive(Debug, Clone, Default, Serialize)]
pub struct ExtraChunk {
    pub template_name: &'static str,
    pub template_path: &'static str,
    pub chunks: Vec<Chunk>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Chunk {
    pub id: String,
    pub data: String,
}

impl ExtraChunk {
    pub fn new() -> Self {
        Self {
            template_name: TEMPLATE_NAME,
            template_path: TEMPLATE_PATH,
            chunks: Default::default(),
        }
    }

    pub fn add_chunk(&mut self, chunk_id: String, mut chunk_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let chunk_size = chunk_data.len();
        let chunk_data = take_first_number_of_bytes_as_string(&mut chunk_data, chunk_size)?;
        self.chunks.push(Chunk {
            id: chunk_id,
            data: chunk_data,
        });

        Ok(())
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, Box<dyn Error>> {
        template.add_chunk_template(self.template_name, self.template_path)?;

        if self.chunks.is_empty() {
            return Ok("".to_string());
        }

        let wave_output_values: Value = upon::value! {
            extra_chunks: &self.chunks
        };

        Ok(template.get_wave_chunk_output(self.template_name, wave_output_values)?)
    }
}
