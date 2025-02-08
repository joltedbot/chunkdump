use crate::byteio::take_first_number_of_bytes_as_string;
use crate::template::Template;
use std::error::Error;
use upon::Value;

const TEMPLATE_NAME: &str = "xmp";
const TEMPLATE_PATH: &str = include_str!("../templates/wave/xmp.tmpl");

#[derive(Debug, Clone, Default)]
pub struct XMPFields {
    pub template_name: &'static str,
    pub template_path: &'static str,
    pub xmp_xml: String,
}

impl XMPFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let chunk_size = chunk_data.len();
        let xmp_xml = take_first_number_of_bytes_as_string(&mut chunk_data, chunk_size)?;
        Ok(Self {
            template_name: TEMPLATE_NAME,
            template_path: TEMPLATE_PATH,
            xmp_xml,
        })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, Box<dyn Error>> {
        let wave_output_values: Value = upon::value! {
            xmp_xml: self.xmp_xml.clone(),
        };

        let formated_output = template.get_wave_chunk_output(self.template_name, self.template_path, wave_output_values)?;
        Ok(formated_output)
    }
}
