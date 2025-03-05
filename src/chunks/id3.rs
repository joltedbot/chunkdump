use crate::chunks::{Chunk, Section};
use crate::errors::LocalError;
use crate::template::get_file_chunk_output;
use id3::Tag;
use serde::Serialize;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/id3.tmpl");

const TIME_FIELD_TITLE: &str = "Time";
const TIME_HOUR_MINUTE_DIVIDER_POSITION: usize = 2;

#[derive(Debug, Clone, Default, Serialize)]
struct ID3Tag {
    id: String,
    spacer: String,
    content: String,
}

fn get_longest_tag_id(tags: &Tag) -> Result<usize, LocalError> {
    match tags.frames().max_by_key(|tag| tag.name().len()) {
        Some(tag) => Ok(tag.name().len()),
        None => Err(LocalError::ErrorParsingID3TagIDs),
    }
}

pub fn get_metadata(file_path: &str) -> Result<Chunk, Box<dyn Error>> {
    let mut id3_entries: Vec<ID3Tag> = Vec::new();
    let tag = Tag::read_from_path(file_path).map_err(|e| LocalError::InvalidID3TagDataFound(e.to_string()))?;
    let longest_tag_id = get_longest_tag_id(&tag)?;

    tag.frames().for_each(|frame| {
        let mut content: String = frame.content().to_string();
        let id = frame.name().to_string();
        if id == TIME_FIELD_TITLE {
            content.insert(TIME_HOUR_MINUTE_DIVIDER_POSITION, ':');
        }

        let spacer = " ".repeat(longest_tag_id - id.len());

        id3_entries.push(ID3Tag { id, spacer, content });
    });

    let wave_output_values: Value = upon::value! {
        id3_tags: id3_entries,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, wave_output_values)?;

    Ok(Chunk {
        section: Section::Optional,
        text: formated_output,
    })
}
