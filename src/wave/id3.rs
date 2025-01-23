use crate::fileio::skip_over_bytes_in_file;
use crate::wave::CHUNK_SIZE_FIELD_LENGTH_IN_BYTES;
use id3::Tag;
use std::error::Error;
use std::fs::File;

pub fn read_id3_chunk(wave_file: &mut File) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    skip_over_bytes_in_file(wave_file, CHUNK_SIZE_FIELD_LENGTH_IN_BYTES as i64)?;
    let mut id3_entries: Vec<(String, String)> = Default::default();
    let tag = Tag::read_from2(wave_file)?;

    for frame in tag.frames() {
        id3_entries.push((frame.name().to_string(), frame.content().to_string()));
    }

    Ok(id3_entries)
}
