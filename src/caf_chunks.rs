mod chan;
mod desc;
mod info;
mod mark;
mod midi;
mod ovvw;
mod regn;
mod strg;
mod uuid;

use crate::byte_arrays::Endian;
use crate::chunks::{extra, id3, skipped, text};
use crate::fileio::{
    read_bytes_from_file, read_chunk_id_from_file, read_chunk_size_from_file,
    skip_over_bytes_in_file,
};
use crate::output::OutputEntry;
use std::error::Error;
use std::fs::File;

pub const CHUNK_SIZE_FIELD_LENGTH_IN_BYTES: usize = 8;
const AUDIO_DESCRIPTION_CHUNK_ID: &str = "desc";
const AUDIO_DATA_CHUNK_ID: &str = "data";
const PACKET_DESCRIPTION_CHUNK_ID: &str = "pakt";
const CHANNEL_LAYOUT_CHUNK_ID: &str = "chan";
const MAGIC_COOKIE_CHUNK_ID: &str = "kuki";
const MARKER_CHUNK_ID: &str = "mark";
const REGION_CHUNK_ID: &str = "regn";
const MIDI_CHUNK_ID: &str = "midi";
const OVERVIEW_CHUNK_ID: &str = "ovvw";
const INFORMATION_CHUNK_ID: &str = "info";
const USER_DEFINED_CHUNK_ID: &str = "uuid";
const ID3_CHUNK_ID: &str = "id3 ";
const FREE_CHUNK_ID: &str = "free";
const FREE_CHUNK_TITLE: &str = "Free (File Padding)";
const STRINGS_CHUNK_ID: &str = "strg";
// const INSTRUMENT_CHUNK_ID: &str = "instr";
// const EDIT_COMMENTS_CHUNK_ID: &str = "edct";
// const PEAK_CHUNK_ID: &str = "peak";
// const IDENTIFIER_CHUNK_ID: &str = "umid";

pub const MANDATORY_CHUNKS: [&str; 3] = [
    AUDIO_DESCRIPTION_CHUNK_ID,
    PACKET_DESCRIPTION_CHUNK_ID,
    MAGIC_COOKIE_CHUNK_ID,
];
pub const CHUNKS_NOT_TO_EXTRACT_DATA_FROM: [&str; 3] = [
    AUDIO_DATA_CHUNK_ID,
    PACKET_DESCRIPTION_CHUNK_ID,
    ID3_CHUNK_ID,
];
pub const ERROR_TO_MATCH_IF_NOT_ENOUGH_BYTES_LEFT_IN_FILE: &str = "failed to fill whole buffer";

pub fn get_metadata_from_caf_chunks(
    input_file: &mut File,
    file_path: &str,
    mandatory_sections_only: bool,
) -> Result<Vec<OutputEntry>, Box<dyn Error>> {
    let mut output: Vec<OutputEntry> = vec![];

    loop {
        let chunk_id: String = read_chunk_id_from_file(input_file)?;

        if chunk_id.is_empty() {
            break;
        }

        let chunk_size = if chunk_id == ID3_CHUNK_ID {
            read_chunk_size_from_file(input_file, Endian::Little)?
        } else {
            read_caf_chunk_size_from_file(input_file)?
        };

        if mandatory_sections_only && !MANDATORY_CHUNKS.contains(&chunk_id.as_str()) {
            skip_over_bytes_in_file(input_file, chunk_size)?;
            continue;
        }

        let chunk_data =
            get_caf_chunk_data_bytes_from_file(input_file, chunk_id.clone(), chunk_size)?;
        output.push(get_caf_chunk_metadata(chunk_id, file_path, chunk_data)?);
    }

    Ok(output)
}

pub fn read_caf_chunk_size_from_file(file: &mut File) -> Result<usize, Box<dyn Error>> {
    let chunk_size_bytes = read_bytes_from_file(file, CHUNK_SIZE_FIELD_LENGTH_IN_BYTES)?;
    let mut byte_array: [u8; CHUNK_SIZE_FIELD_LENGTH_IN_BYTES] = Default::default();
    byte_array.copy_from_slice(chunk_size_bytes.as_slice());

    let chunk_size = i64::from_be_bytes(byte_array);

    Ok(chunk_size as usize)
}

fn get_caf_chunk_data_bytes_from_file(
    input_file: &mut File,
    chunk_id: String,
    chunk_size: usize,
) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(
        if CHUNKS_NOT_TO_EXTRACT_DATA_FROM.contains(&chunk_id.as_str()) {
            skip_over_bytes_in_file(input_file, chunk_size)?;
            Vec::new()
        } else {
            read_bytes_from_file(input_file, chunk_size).unwrap_or_default()
        },
    )
}

fn get_caf_chunk_metadata(
    chunk_id: String,
    file_path: &str,
    chunk_data: Vec<u8>,
) -> Result<OutputEntry, Box<dyn Error>> {
    let result = match chunk_id.as_str() {
        AUDIO_DESCRIPTION_CHUNK_ID => desc::get_metadata(chunk_data)?,
        AUDIO_DATA_CHUNK_ID => skipped::get_metadata(chunk_id)?,
        CHANNEL_LAYOUT_CHUNK_ID => chan::get_metadata(chunk_data)?,
        USER_DEFINED_CHUNK_ID => uuid::get_metadata(chunk_data)?,
        INFORMATION_CHUNK_ID => info::get_metadata(chunk_data)?,
        OVERVIEW_CHUNK_ID => ovvw::get_metadata(chunk_data)?,
        STRINGS_CHUNK_ID => strg::get_metadata(chunk_data)?,
        FREE_CHUNK_ID => text::get_metadata(FREE_CHUNK_TITLE, chunk_data)?,
        PACKET_DESCRIPTION_CHUNK_ID => skipped::get_metadata(chunk_id)?,
        MIDI_CHUNK_ID => midi::get_metadata(chunk_data)?,
        MAGIC_COOKIE_CHUNK_ID => skipped::get_metadata(chunk_id)?,
        ID3_CHUNK_ID => id3::get_metadata(file_path)?,
        REGION_CHUNK_ID => regn::get_metadata(chunk_data)?,
        MARKER_CHUNK_ID => mark::get_metadata(chunk_data)?,
        _ => extra::get_metadata(chunk_id, chunk_data)?,
    };

    Ok(result)
}
