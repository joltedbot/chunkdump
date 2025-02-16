/*
use crate::template::Template;
use std::error::Error;
use upon::Value;

// Replace the word placeholder below with your own short chunk name

const TEMPLATE_NAME: &str = "placeholder"; // A short name to identify the template
const TEMPLATE_CONTENT: &str = include_str!("../templates/wave/placeholder.tmpl"); // The file path where you placed the template

// Rename the struct to reflect your new chunk nmae
#[derive(Debug, Clone, Default)]
pub struct PlaceholderFields {
    template_name: &'static str,
    template_content: &'static str,
    // Insert a struct field for each of the data fields in your chunk. They do not need to be static as with the templates
}

// Rename the struct to reflect your struct name
impl PlaceholderFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Self {
        let chunk_size = chunk_data.len();

        // Insert Code to process the data fields of your chunk from the provided chunk data. Chunk data is a vec of u8 bytes

        // Rename the struct to reflect your struct name
        PlaceholderFieldsFields {
            template_name: TEMPLATE_NAME,
            template_content: TEMPLATE_CONTENT,
            // Assign the data fields you processed above to the struct fields in your struct
        }
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        template.add_chunk_template(self.template_name, self.template_content)?;

        let wave_output_values: Value = upon::value! {
            // Assign a value from your struct fields above for each variable name in your template
            // formated like this but with what ever additional code is needed to parse it correctly
            //  variable: self.struct_field
        };

        let formated_output = template.get_wave_chunk_output(self.template_name, wave_output_values)?;
        Ok(formated_output)
    }
}
*/
