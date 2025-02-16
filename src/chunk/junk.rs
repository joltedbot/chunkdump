use crate::byteio::take_first_number_of_bytes_as_string;
use crate::errors::LocalError;
use crate::template::Template;
use upon::Value;

const TEMPLATE_NAME: &str = "junk";
const TEMPLATE_CONTENT: &str = include_str!("../templates/wave/junk.tmpl");

#[derive(Debug, Clone, Default)]
pub struct JunkFields {
    template_name: &'static str,
    template_content: &'static str,
    junk_as_string: String,
}

impl JunkFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, LocalError> {
        let chunk_size = chunk_data.len();
        let junk_as_string = take_first_number_of_bytes_as_string(&mut chunk_data, chunk_size)?;
        Ok(JunkFields {
            template_name: TEMPLATE_NAME,
            template_content: TEMPLATE_CONTENT,
            junk_as_string,
        })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        if self.junk_as_string.is_empty() {
            return Ok("".to_string());
        }

        let wave_output_values: Value = upon::value! {
            junk: self.junk_as_string.clone(),
        };

        let formated_output =
            template.get_wave_chunk_output(self.template_name, self.template_content, wave_output_values)?;
        Ok(formated_output)
    }
}
