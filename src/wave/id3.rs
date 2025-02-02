use crate::fileio::{read_chunk_size_from_file, skip_over_bytes_in_file};
use crate::template::Template;
use crate::wave::add_one_if_byte_size_is_odd;
use id3::Tag;
use serde::Serialize;
use std::error::Error;
use std::fs::File;

const TIME_FIELD_TITLE: &str = "Time";
const TIME_HOUR_MINUTE_DIVIDER_POSITION: usize = 2;

#[derive(Debug, Clone, Default, Serialize)]
pub struct ID3Tag {
    pub id: String,
    pub content: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ID3Fields {
    pub tags: Vec<ID3Tag>,
}

impl ID3Fields {
    pub fn new(wave_file: &mut File, wave_file_path: String) -> Result<Self, Box<dyn Error>> {
        let mut chunk_size = read_chunk_size_from_file(wave_file)?;

        chunk_size = add_one_if_byte_size_is_odd(chunk_size);

        skip_over_bytes_in_file(wave_file, chunk_size as i64)?;

        let mut id3_entries: Self = Default::default();
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

    pub fn get_metadata_output(&self, template: &Template, template_name: &str) -> Result<String, Box<dyn Error>> {
        let id3_output = template.get_wave_chunk_output(
            template_name,
            upon::value! {
                id3_tags: self.tags.clone(),
            },
        )?;

        Ok(id3_output)
    }
}
