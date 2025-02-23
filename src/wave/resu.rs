use crate::errors::LocalError;
use crate::template::Template;
use flate2::read::ZlibDecoder;
use std::io::prelude::*;
use upon::Value;

const TEMPLATE_NAME: &str = "resu";
const TEMPLATE_CONTENT: &str = include_str!("../templates/wave/resu.tmpl");

#[derive(Debug, Clone, Default)]
pub struct ResuFields {
    resu_json: String,
}

impl ResuFields {
    pub fn new(chunk_data: Vec<u8>) -> Result<Self, LocalError> {
        let mut zlib = ZlibDecoder::new(chunk_data.as_slice());
        let mut resu_json = String::new();
        zlib.read_to_string(&mut resu_json)
            .map_err(|e| LocalError::InvalidZipDataFound(e.to_string()))?;

        Ok(Self { resu_json })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        let wave_output_values: Value = upon::value! {
            resu_json: self.resu_json.clone(),
        };

        let formated_output = template.get_wave_chunk_output(TEMPLATE_NAME, TEMPLATE_CONTENT, wave_output_values)?;
        Ok(formated_output)
    }
}
