use crate::byte_arrays::take_first_number_of_bytes_as_string;
use crate::formating::format_bytes_as_string;
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::collections::HashMap;
use std::error::Error;
use upon::Value;

const APPLICATION_ID_NAMES: [(&str, &str); 24] = [
    ("ATCH", "FlacFile"),
    ("BSOL", "beSolo"),
    ("BUGS", "Bugs Player"),
    ("Cues", "GoldWave cue points"),
    ("Fica", "CUE Splitter"),
    ("Ftol", "flac-tools"),
    ("MOTB", "MOTB MetaCzar"),
    ("MPSE", "MP3 Stream Editor"),
    ("MuML", "MusicML: Music Metadata Language"),
    ("RIFF", "Sound Devices RIFF chunk storage"),
    ("SFFL", "Sound Font FLAC"),
    ("SONY", "Sony Creative Software"),
    ("SQEZ", "flacsqueeze"),
    ("TtWv", "TwistedWave"),
    ("UITS", "UITS Embedding tools"),
    ("aiff", "FLAC AIFF chunk storage"),
    ("imag", "flac-image"),
    ("peem", "Parseable Embedded Extensible Metadata"),
    ("qfst", "QFLAC Studio"),
    ("riff", "FLAC RIFF chunk storage"),
    ("tune", "TagTuner"),
    ("w64 ", "FLAC Wave64 chunk storage"),
    ("xbat", "XBAT"),
    ("xmcd", "xmcd"),
];

const APPLICATION_ID_LENGTH_IN_BYTES: usize = 4;
const TEMPLATE_CONTENT: &str = include_str!("../templates/blocks/application.tmpl");

pub fn get_metadata(mut block_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let application_id_names: HashMap<&str, &str> = HashMap::from(APPLICATION_ID_NAMES);
    let application_id = take_first_number_of_bytes_as_string(&mut block_data, APPLICATION_ID_LENGTH_IN_BYTES)?;
    let application = *application_id_names
        .get(application_id.as_str())
        .unwrap_or(&"Unknown Application Type");

    let data = format_bytes_as_string(block_data)?;

    let output_values: Value = upon::value! {
        application_id: application,
        data: data,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, output_values)?;

    let result = OutputEntry {
        section: Section::Optional,
        text: formated_output,
    };

    Ok(result)
}
