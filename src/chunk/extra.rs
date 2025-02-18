use crate::byteio::take_first_number_of_bytes_as_string;
use crate::errors::LocalError;
use crate::template::Template;
use serde::Serialize;
use upon::Value;

const TEMPLATE_NAME: &str = "extra";
const TEMPLATE_CONTENT: &str = include_str!("../templates/wave/extra.tmpl");
const EMPTY_DATA_MESSAGE: &str = "[The chunk exists but is empty]";

#[derive(Debug, Clone, Default, Serialize)]
pub struct ExtraChunk {
    template_name: &'static str,
    template_content: &'static str,
    chunks: Vec<Chunk>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Chunk {
    id: String,
    data: String,
}

impl ExtraChunk {
    pub fn new() -> Self {
        Self {
            template_name: TEMPLATE_NAME,
            template_content: TEMPLATE_CONTENT,
            chunks: Default::default(),
        }
    }

    pub fn add_chunk(&mut self, chunk_id: String, mut chunk_data: Vec<u8>) -> Result<(), LocalError> {
        let chunk_size = chunk_data.len();
        let mut chunk_data = take_first_number_of_bytes_as_string(&mut chunk_data, chunk_size)?;

        if chunk_data.is_empty() {
            chunk_data = EMPTY_DATA_MESSAGE.to_string();
        }

        self.chunks.push(Chunk {
            id: chunk_id,
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

        let formated_output =
            template.get_wave_chunk_output(self.template_name, self.template_content, wave_output_values)?;
        Ok(formated_output)
    }
}
