use crate::byteio::take_first_number_of_bytes_as_string;
use crate::errors::LocalError;
use crate::template::Template;
use upon::Value;

const TEMPLATE_NAME: &str = "ixml";
const TEMPLATE_CONTENT: &str = include_str!("../templates/wave/ixml.tmpl");

#[derive(Debug, Clone, Default)]
pub struct IXMLFields {
    template_name: &'static str,
    template_content: &'static str,
    ixml_xml: String,
}

impl IXMLFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, LocalError> {
        let chunk_size = chunk_data.len();
        let ixml_xml = take_first_number_of_bytes_as_string(&mut chunk_data, chunk_size)?;
        Ok(Self {
            template_name: TEMPLATE_NAME,
            template_content: TEMPLATE_CONTENT,
            ixml_xml,
        })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        let wave_output_values: Value = upon::value! {
            ixml_xml: &self.ixml_xml,
        };

        let formated_output =
            template.get_wave_chunk_output(self.template_name, self.template_content, wave_output_values)?;
        Ok(formated_output)
    }
}
