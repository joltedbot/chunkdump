use crate::byteio::take_first_four_bytes_as_unsigned_integer;
use crate::errors::LocalError;
use crate::template::Template;
use upon::Value;

const TEMPLATE_NAME: &str = "fact";
const TEMPLATE_CONTENT: &str = include_str!("../templates/wave/fact.tmpl");

#[derive(Debug, Clone, Default)]
pub struct FactFields {
    template_name: &'static str,
    template_content: &'static str,
    samples_per_channel: u32,
}

impl FactFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, LocalError> {
        let samples_per_channel = take_first_four_bytes_as_unsigned_integer(&mut chunk_data)?;

        Ok(Self {
            template_name: TEMPLATE_NAME,
            template_content: TEMPLATE_CONTENT,
            samples_per_channel,
        })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        let wave_output_values: Value = upon::value! {
            samples_per_channel: self.samples_per_channel,
        };

        let formated_output =
            template.get_wave_chunk_output(self.template_name, self.template_content, wave_output_values)?;
        Ok(formated_output)
    }
}
