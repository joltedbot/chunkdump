use crate::fileio::{read_chunk_size_from_file, skip_over_bytes_in_file};
use id3::Tag;
use std::error::Error;
use std::fs::File;

const TIME_FIELD_TITLE: &str = "Time";
const TIME_HOUR_MINUTE_DIVIDER_POSITION: usize = 2;

#[derive(Debug, Clone, Default)]
pub struct ID3Fields {
    pub tags: Vec<(String, String)>,
}

impl ID3Fields {
    pub fn new(wave_file: &mut File, wave_file_path: String) -> Result<Self, Box<dyn Error>> {
        let mut chunk_size = read_chunk_size_from_file(wave_file)?;

        if chunk_size % 2 > 0 {
            chunk_size += 1;
        }

        skip_over_bytes_in_file(wave_file, chunk_size as i64)?;

        let mut id3_entries: Self = Default::default();
        let tag = Tag::read_from_path(wave_file_path)?;

        for frame in tag.frames() {
            let mut content: String = frame.content().to_string();
            let name = frame.name().to_string();
            if name == TIME_FIELD_TITLE {
                content.insert(TIME_HOUR_MINUTE_DIVIDER_POSITION, ':');
            }
            id3_entries.tags.push((name, content));
        }

        Ok(id3_entries)
    }

    pub fn get_metadata_output(&self) -> Vec<String> {
        let mut id3_data: Vec<String> = vec![];

        if !self.tags.is_empty() {
            id3_data.push("\n-------------\nID3 Chunk Details:\n-------------".to_string());

            self.tags.iter().for_each(|tag| {
                id3_data.push(format!("{}: {}", tag.0, tag.1));
            });
        }

        id3_data
    }
}
