use crate::errors::LocalError;
use crate::template::Template;
use id3::Tag;
use serde::Serialize;
use upon::Value;

const TEMPLATE_NAME: &str = "id3";
const TEMPLATE_CONTENT: &str = include_str!("../templates/aiff/id3.tmpl");

const TIME_FIELD_TITLE: &str = "Time";
const TIME_HOUR_MINUTE_DIVIDER_POSITION: usize = 2;

#[derive(Debug, Clone, Default, Serialize)]
struct ID3Tag {
    id: String,
    spacer: String,
    content: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ID3Fields {
    tags: Vec<ID3Tag>,
}

impl ID3Fields {
    pub fn new(wave_file_path: String) -> Result<Self, LocalError> {
        let mut id3_entries = Self { tags: Vec::new() };

        let tag = Tag::read_from_path(wave_file_path).map_err(|e| LocalError::InvalidID3TagDataFound(e.to_string()))?;

        let longest_tag_id = get_longest_tag_id(&tag)?;

        for frame in tag.frames() {
            let mut content: String = frame.content().to_string();
            let id = frame.name().to_string();
            if id == TIME_FIELD_TITLE {
                content.insert(TIME_HOUR_MINUTE_DIVIDER_POSITION, ':');
            }
            let spacer = " ".repeat(longest_tag_id - id.len());

            id3_entries.tags.push(ID3Tag { id, spacer, content });
        }

        Ok(id3_entries)
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        let wave_output_values: Value = upon::value! {
            id3_tags: self.tags.clone(),
        };

        let formated_output = template.get_wave_chunk_output(TEMPLATE_NAME, TEMPLATE_CONTENT, wave_output_values)?;
        Ok(formated_output)
    }
}

fn get_longest_tag_id(tags: &Tag) -> Result<usize, LocalError> {
    match tags.frames().max_by_key(|tag| tag.name().len()) {
        Some(tag) => Ok(tag.name().len()),
        None => return Err(LocalError::ErrorParsingID3TagIDs),
    }
}
