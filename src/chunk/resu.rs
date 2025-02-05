use crate::template::Template;
use flate2::read::ZlibDecoder;
use std::error::Error;
use std::io::prelude::*;
use upon::Value;

const TEMPLATE_NAME: &str = "resu";
const TEMPLATE_PATH: &str = include_str!("../templates/wave/resu.tmpl");

#[derive(Debug, Clone, Default)]
pub struct ResuFields {
    pub template_name: &'static str,
    pub template_path: &'static str,
    pub resu_json: String,
}

impl ResuFields {
    pub fn new(chunk_data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let mut zlib = ZlibDecoder::new(chunk_data.as_slice());
        let mut resu_json = String::new();
        zlib.read_to_string(&mut resu_json)?;

        Ok(Self {
            template_name: TEMPLATE_NAME,
            template_path: TEMPLATE_PATH,
            resu_json,
        })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, Box<dyn Error>> {
        template.add_chunk_template(self.template_name, self.template_path)?;

        let wave_output_values: Value = upon::value! {
            resu_json: self.resu_json.clone(),
        };

        Ok(template.get_wave_chunk_output(self.template_name, wave_output_values)?)
    }
}
