/*
    Types of chunks found in AIFF files:
    FORM  - the whole file chunk (like RIFF in a wave file)
    - Format Version Chunk - FVER
    - Common Chunk (required) - COMM - general file format data (like fmt in wave)
    - Sound Data Chunk (required) - SSND - the pcm samples (like data in a wave file)
    - Comment Chunk - COMT
    - Marker Chunk - MARK - Like cues
    Instrument Chunk - INST
    Name Chunk - NAME
    Author Chunk - AUTH
    Copyright Chunk - '(c) '
    Annotation Chunk - ANNO
    Audio Recording Chunk - AESD
    MIDI Data Chunk - MIDI
    Application Chunk - APPL
    ID3 Chunk - 'ID3 '
*/
use crate::aiff::comm::CommonFields;
use crate::aiff::comt::CommentFields;
use crate::aiff::extra::ExtraChunks;
use crate::aiff::fver::FormatVersionFields;
use crate::aiff::mark::MarkerFields;
use crate::template::Template;
use std::error::Error;

const CHAN_CHUNK_ID: &str = "chan";
const COMMON_CHUNK_ID: &str = "comm";
const COMMENT_CHUNK_ID: &str = "comt";
const FORMAT_VERSION_CHUNK_ID: &str = "fver";
const LOGIC_PRO_CHUNK_ID: &str = "lgwv";
const MARKER_CHUNK_ID: &str = "mark";
pub const AUDIO_SAMPLES_CHUNK_ID: &str = "ssnd";

const NUMBER_OF_CHUNKS_TO_SKIP: usize = 3;
const CHUNKS_TO_SKIP: [&str; NUMBER_OF_CHUNKS_TO_SKIP] = [AUDIO_SAMPLES_CHUNK_ID, LOGIC_PRO_CHUNK_ID, CHAN_CHUNK_ID];

#[derive(Default)]
pub struct Chunk {
    pub found_chunk_ids: Vec<String>,
    pub skipped_chunk_ids: Vec<String>,
    pub ignore_data_for_chunks: [&'static str; NUMBER_OF_CHUNKS_TO_SKIP],
    extra_chunks: ExtraChunks,
    format_version: FormatVersionFields,
    common: CommonFields,
    comments: CommentFields,
    markers: MarkerFields,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            ignore_data_for_chunks: CHUNKS_TO_SKIP,
            extra_chunks: ExtraChunks::new(),
            ..Default::default()
        }
    }

    pub fn add_chunk(&mut self, chunk_id: &str, chunk_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let found_chunk_id = chunk_id.to_string();

        match chunk_id {
            FORMAT_VERSION_CHUNK_ID => self.format_version = FormatVersionFields::new(chunk_data)?,
            COMMON_CHUNK_ID => self.common = CommonFields::new(chunk_data)?,
            COMMENT_CHUNK_ID => self.comments = CommentFields::new(chunk_data)?,
            MARKER_CHUNK_ID => self.markers = MarkerFields::new(chunk_data)?,
            LOGIC_PRO_CHUNK_ID => {}
            AUDIO_SAMPLES_CHUNK_ID => {}
            CHAN_CHUNK_ID => {}
            _ => self.extra_chunks.add_chunk(chunk_id, chunk_data)?,
        }

        if CHUNKS_TO_SKIP.contains(&found_chunk_id.as_str()) {
            self.skipped_chunk_ids.push(found_chunk_id);
        } else if !self.found_chunk_ids.contains(&found_chunk_id) {
            self.found_chunk_ids.push(found_chunk_id);
        }
        Ok(())
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<Vec<String>, Box<dyn Error>> {
        let mut data_output_lines: Vec<String> = vec![];

        for chunk in self.found_chunk_ids.iter() {
            let chunk_fields = match chunk.as_str() {
                FORMAT_VERSION_CHUNK_ID => self.format_version.format_data_for_output(template)?,
                COMMON_CHUNK_ID => self.common.format_data_for_output(template)?,
                COMMENT_CHUNK_ID => self.comments.format_data_for_output(template)?,
                MARKER_CHUNK_ID => self.markers.format_data_for_output(template)?,
                _ => continue,
            };

            if !chunk_fields.is_empty() {
                data_output_lines.push(chunk_fields);
            }
        }

        let extra_chunks_output = self.extra_chunks.format_data_for_output(template)?;

        if !extra_chunks_output.is_empty() {
            data_output_lines.push(extra_chunks_output);
        }

        Ok(data_output_lines)
    }
}
