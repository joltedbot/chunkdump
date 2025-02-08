use crate::template::Template;
use id3::Tag;
use serde::Serialize;
use std::error::Error;
use upon::Value;

const TEMPLATE_NAME: &str = "id3";
const TEMPLATE_PATH: &str = include_str!("../templates/wave/id3.tmpl");

const TIME_FIELD_TITLE: &str = "Time";
const TIME_HOUR_MINUTE_DIVIDER_POSITION: usize = 2;

#[derive(Debug, Clone, Default, Serialize)]
pub struct ID3Tag {
    pub id: String,
    pub content: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ID3Fields {
    pub template_name: &'static str,
    pub template_path: &'static str,
    pub tags: Vec<ID3Tag>,
}

impl ID3Fields {
    pub fn new(wave_file_path: String) -> Result<Self, Box<dyn Error>> {
        let mut id3_entries: Self = Default::default();
        id3_entries.template_name = TEMPLATE_NAME;
        id3_entries.template_path = TEMPLATE_PATH;

        let tag = Tag::read_from_path(wave_file_path)?;

        for frame in tag.frames() {
            let mut content: String = frame.content().to_string();
            let id = frame.name().to_string();
            if id == TIME_FIELD_TITLE {
                content.insert(TIME_HOUR_MINUTE_DIVIDER_POSITION, ':');
            }
            id3_entries.tags.push(ID3Tag { id, content });
        }

        Ok(id3_entries)
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, Box<dyn Error>> {
        let wave_output_values: Value = upon::value! {
            id3_tags: self.tags.clone(),
        };

        let formated_output = template.get_wave_chunk_output(self.template_name, self.template_path, wave_output_values)?;
        Ok(formated_output)
    }
}
