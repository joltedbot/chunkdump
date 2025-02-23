use crate::byteio::take_first_number_of_bytes_as_string;
use crate::errors::LocalError;
use crate::formating::add_one_if_byte_size_is_odd;
use crate::template::Template;
use upon::Value;

const TEMPLATE_NAME: &str = "text";
const TEMPLATE_CONTENT: &str = include_str!("../templates/shared/text.tmpl");

#[derive(Debug, Clone, Default)]
pub struct TextFields {
    body: String,
}

impl TextFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, LocalError> {
        let chunk_size = add_one_if_byte_size_is_odd(chunk_data.len() as u32);
        let body = take_first_number_of_bytes_as_string(&mut chunk_data, chunk_size as usize)?;
        Ok(Self { body })
    }

    pub fn format_data_for_output(&self, template: &mut Template, title: &str) -> Result<String, upon::Error> {
        let wave_output_values: Value = upon::value! {
            title: title,
            body: &self.body,
        };

        let formated_output = template.get_wave_chunk_output(TEMPLATE_NAME, TEMPLATE_CONTENT, wave_output_values)?;
        Ok(formated_output)
    }
}
