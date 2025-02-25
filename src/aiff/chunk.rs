use crate::aiff::comm::CommonFields;
use crate::aiff::comt::CommentFields;
use crate::aiff::fver::FormatVersionFields;
use crate::aiff::mark::MarkerFields;
use crate::shared::extra::ExtraChunks;
use crate::shared::id3::ID3Fields;
use crate::shared::text::TextFields;
use crate::template::Template;
use std::error::Error;

const ANNOTATION_CHUNK_ID: &str = "anno";
const ANNOTATION_TEMPLATE_TITLE: &str = "Annotation (ANNO) Chunk Details";
const APPLICATION_CHUNK_ID: &str = "appl";
const APPLICATION_TEMPLATE_TITLE: &str = "Application (APPL) Chunk Details";
const AUTHOR_CHUNK_ID: &str = "auth";
const AUTHOR_TEMPLATE_TITLE: &str = "Author (auth) Chunk Details";
const COPYRIGHT_CHUNK_ID: &str = "(c) ";
const COPYRIGHT_TEMPLATE_TITLE: &str = "Copyright ((c) ) Chunk Details";
const CHAN_CHUNK_ID: &str = "chan";
const COMMON_CHUNK_ID: &str = "comm";
const COMMENT_CHUNK_ID: &str = "comt";
const FORMAT_VERSION_CHUNK_ID: &str = "fver";
pub const ID3_CHUNK_ID: &str = "id3 ";
pub const JUNK_CHUNK_ID: &str = "junk";
const JUNK_TEMPLATE_TITLE: &str = "Junk Chunk Details";
const LOGIC_PRO_CHUNK_ID: &str = "lgwv";
const MARKER_CHUNK_ID: &str = "mark";

pub const AUDIO_SAMPLES_CHUNK_ID: &str = "ssnd";
pub const NAME_CHUNK_ID: &str = "name";
const NAME_TEMPLATE_TITLE: &str = "Name Chunk Details";

const NUMBER_OF_CHUNKS_TO_SKIP: usize = 3;
const CHUNKS_TO_SKIP: [&str; NUMBER_OF_CHUNKS_TO_SKIP] = [AUDIO_SAMPLES_CHUNK_ID, LOGIC_PRO_CHUNK_ID, CHAN_CHUNK_ID];

#[derive(Default)]
pub struct Chunk {
    pub found_chunk_ids: Vec<String>,
    pub skipped_chunk_ids: Vec<String>,
    pub ignore_data_for_chunks: [&'static str; NUMBER_OF_CHUNKS_TO_SKIP],
    file_path: String,
    extra_chunks: ExtraChunks,
    format_version: FormatVersionFields,
    common: CommonFields,
    comments: CommentFields,
    markers: MarkerFields,
    id3: ID3Fields,
    application: TextFields,
    junk: TextFields,
    annotation: TextFields,
    author: TextFields,
    name: TextFields,
}

impl Chunk {
    pub fn new(aiff_file_path: String) -> Self {
        Self {
            file_path: aiff_file_path,
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
            ID3_CHUNK_ID => self.id3 = ID3Fields::new(self.file_path.clone())?,
            APPLICATION_CHUNK_ID => self.application = TextFields::new(chunk_data)?,
            ANNOTATION_CHUNK_ID => self.annotation = TextFields::new(chunk_data)?,
            AUTHOR_CHUNK_ID => self.author = TextFields::new(chunk_data)?,
            NAME_CHUNK_ID => self.name = TextFields::new(chunk_data)?,
            COPYRIGHT_CHUNK_ID => self.name = TextFields::new(chunk_data)?,
            JUNK_CHUNK_ID => self.junk = TextFields::new(chunk_data)?,

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
                ID3_CHUNK_ID => self.id3.format_data_for_output(template)?,
                APPLICATION_CHUNK_ID => self
                    .application
                    .format_data_for_output(template, APPLICATION_TEMPLATE_TITLE)?,
                ANNOTATION_CHUNK_ID => self
                    .annotation
                    .format_data_for_output(template, ANNOTATION_TEMPLATE_TITLE)?,

                AUTHOR_CHUNK_ID => self.author.format_data_for_output(template, AUTHOR_TEMPLATE_TITLE)?,

                NAME_CHUNK_ID => self.name.format_data_for_output(template, NAME_TEMPLATE_TITLE)?,
                COPYRIGHT_CHUNK_ID => self.name.format_data_for_output(template, COPYRIGHT_TEMPLATE_TITLE)?,

                JUNK_CHUNK_ID => self.junk.format_data_for_output(template, JUNK_TEMPLATE_TITLE)?,

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
