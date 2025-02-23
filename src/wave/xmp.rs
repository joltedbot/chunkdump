use crate::byteio::take_first_number_of_bytes_as_string;
use crate::errors::LocalError;
use crate::template::Template;
use upon::Value;

const TEMPLATE_NAME: &str = "xmp";
const TEMPLATE_CONTENT: &str = include_str!("../templates/wave/xmp.tmpl");

#[derive(Debug, Clone, Default)]
pub struct XMPFields {
    xmp_xml: String,
}

impl XMPFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, LocalError> {
        let chunk_size = chunk_data.len();
        let xmp_xml = take_first_number_of_bytes_as_string(&mut chunk_data, chunk_size)?;
        Ok(Self { xmp_xml })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        let wave_output_values: Value = upon::value! {
            xmp_xml: self.xmp_xml.clone(),
        };

        let formated_output = template.get_wave_chunk_output(TEMPLATE_NAME, TEMPLATE_CONTENT, wave_output_values)?;
        Ok(formated_output)
    }
}
