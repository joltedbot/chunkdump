use crate::bytes::take_first_number_of_bytes_as_string;
use crate::errors::LocalError;
use crate::template::Template;
use serde::Serialize;
use upon::Value;

const TEMPLATE_NAME: &str = "extra";
const TEMPLATE_CONTENT: &str = include_str!("../templates/shared/extra.tmpl");
const EMPTY_DATA_MESSAGE: &str = "[The chunk exists but is empty]";

#[derive(Debug, Clone, Default, Serialize)]
pub struct ExtraChunks {
    chunks: Vec<Chunk>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Chunk {
    id: String,
    data: String,
}

impl ExtraChunks {
    pub fn new() -> Self {
        Self {
            chunks: Default::default(),
        }
    }

    pub fn add_chunk(&mut self, chunk_id: &str, mut chunk_data: Vec<u8>) -> Result<(), LocalError> {
        let chunk_size = chunk_data.len();
        let mut chunk_data = take_first_number_of_bytes_as_string(&mut chunk_data, chunk_size)?;

        if chunk_data.is_empty() {
            chunk_data = EMPTY_DATA_MESSAGE.to_string();
        }

        self.chunks.push(Chunk {
            id: chunk_id.to_string(),
            data: chunk_data,
        });

        Ok(())
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        if self.chunks.is_empty() {
            return Ok("".to_string());
        }

        let wave_output_values: Value = upon::value! {
            extra_chunks: &self.chunks
        };

        let formated_output = template.get_wave_chunk_output(TEMPLATE_NAME, TEMPLATE_CONTENT, wave_output_values)?;
        Ok(formated_output)
    }
}
