use crate::byteio::take_first_four_bytes_as_unsigned_integer;
use crate::template::Template;
use std::error::Error;
use upon::Value;

const TEMPLATE_NAME: &str = "fact";
const TEMPLATE_PATH: &str = include_str!("../templates/wave/fact.tmpl");

#[derive(Debug, Clone, Default)]
pub struct FactFields {
    pub template_name: &'static str,
    pub template_path: &'static str,
    pub samples_per_channel: u32,
}

impl FactFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let samples_per_channel = take_first_four_bytes_as_unsigned_integer(&mut chunk_data)?;

        Ok(Self {
            template_name: TEMPLATE_NAME,
            template_path: TEMPLATE_PATH,
            samples_per_channel,
        })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, Box<dyn Error>> {
        template.add_chunk_template(self.template_name, self.template_path)?;

        let wave_output_values: Value = upon::value! {
            samples_per_channel: self.samples_per_channel,
        };

        Ok(template.get_wave_chunk_output(self.template_name, wave_output_values)?)
    }
}
