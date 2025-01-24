use crate::fileio::{read_four_byte_integer_from_file, skip_over_bytes_in_file};
use id3::Tag;
use std::error::Error;
use std::fs::File;

pub fn read_id3_chunk(
    wave_file: &mut File,
    wave_file_path: String,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let chunk_size = read_four_byte_integer_from_file(wave_file)?;
    skip_over_bytes_in_file(wave_file, chunk_size as i64)?;

    let mut id3_entries: Vec<(String, String)> = Default::default();
    let tag = Tag::read_from_path(wave_file_path)?;

    for frame in tag.frames() {
        id3_entries.push((frame.name().to_string(), frame.content().to_string()));
    }

    Ok(id3_entries)
}
