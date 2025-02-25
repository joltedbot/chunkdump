use crate::bytes::take_first_number_of_bytes;
use crate::errors::LocalError;
use crate::formating::format_bytes_as_string;
use crate::template::Template;
use upon::Value;

const TEMPLATE_NAME: &str = "umid"; // A short name to identify the template
const TEMPLATE_CONTENT: &str = include_str!("../templates/wave/umid.tmpl"); // The file path where you placed the template

// Rename the struct to reflect your new chunk nmae
#[derive(Debug, Clone, Default)]
pub struct UMIDFields {
    umid_bytes: Vec<u8>,
}

// Rename the struct to reflect your struct name
impl UMIDFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, LocalError> {
        let chunk_size = chunk_data.len();

        let umid = take_first_number_of_bytes(&mut chunk_data, chunk_size)?;

        Ok(Self { umid_bytes: umid })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        let wave_output_values: Value = upon::value! {
            umid: format_bytes_as_string(&self.umid_bytes),
        };

        let formated_output = template.get_wave_chunk_output(TEMPLATE_NAME, TEMPLATE_CONTENT, wave_output_values)?;
        Ok(formated_output)
    }
}
