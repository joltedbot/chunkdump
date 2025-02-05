use crate::byteio::take_first_number_of_bytes_as_string;
use crate::template::Template;
use std::error::Error;
use upon::Value;

const TEMPLATE_NAME: &str = "junk";
const TEMPLATE_PATH: &str = include_str!("../templates/wave/junk.tmpl");

#[derive(Debug, Clone, Default)]
pub struct JunkFields {
    pub template_name: &'static str,
    pub template_path: &'static str,
    junk_as_string: String,
}

impl JunkFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let chunk_size = chunk_data.len();
        let junk_as_string = take_first_number_of_bytes_as_string(&mut chunk_data, chunk_size as usize)?;
        Ok(JunkFields {
            template_name: TEMPLATE_NAME,
            template_path: TEMPLATE_PATH,
            junk_as_string,
        })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, Box<dyn Error>> {
        template.add_chunk_template(self.template_name, self.template_path)?;
        if self.junk_as_string.is_empty() {
            return Ok("".to_string());
        }

        let wave_output_values: Value = upon::value! {
            junk: self.junk_as_string.clone(),
        };

        Ok(template.get_wave_chunk_output(self.template_name, wave_output_values)?)
    }
}
