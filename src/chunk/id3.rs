use crate::errors::LocalError;
use crate::template::Template;
use id3::Tag;
use serde::Serialize;
use upon::Value;

const TEMPLATE_NAME: &str = "id3";
const TEMPLATE_CONTENT: &str = include_str!("../templates/wave/id3.tmpl");

const TIME_FIELD_TITLE: &str = "Time";
const TIME_HOUR_MINUTE_DIVIDER_POSITION: usize = 2;

#[derive(Debug, Clone, Default, Serialize)]
struct ID3Tag {
    id: String,
    content: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ID3Fields {
    template_name: &'static str,
    template_content: &'static str,
    tags: Vec<ID3Tag>,
}

impl ID3Fields {
    pub fn new(wave_file_path: String) -> Result<Self, LocalError> {
        let mut id3_entries = Self {
            template_name: TEMPLATE_NAME,
            template_content: TEMPLATE_CONTENT,
            tags: Vec::new(),
        };

        let tag = Tag::read_from_path(wave_file_path).map_err(|e| LocalError::InvalidID3TagDataFound(e.to_string()))?;

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

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        let wave_output_values: Value = upon::value! {
            id3_tags: self.tags.clone(),
        };

        let formated_output =
            template.get_wave_chunk_output(self.template_name, self.template_content, wave_output_values)?;
        Ok(formated_output)
    }
}
