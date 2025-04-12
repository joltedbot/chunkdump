use crate::byte_arrays::{
    take_first_eight_bytes_as_float, take_first_four_bytes_as_unsigned_integer,
    take_first_number_of_bytes_as_string, Endian,
};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::collections::HashMap;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/caf_chunks/desc.tmpl");
const FORMAT_ID_LENGTH_IN_BYTES: usize = 4;
const FORMAT_ID_LINEAR_PCM: &str = "lpcm";
const FORMAT_ID_MPEG_4_AAC: &str = "aac ";
const FORMAT_FLAG_FLOAT: &str = "Floating Point";
const FORMAT_FLAG_UNSIGNED_INTEGER: &str = "Unsigned Integer";
const LITTLE_ENDIAN: &str = "Little Endian";
const BIG_ENDIAN: &str = "Big Endian";

const FORMAT_ID_LONG_FORM: [(&str, &str); 12] = [
    ("lpcm", "Linear PCM"),
    ("ima4", "IMA 4:1 ADPCM"),
    ("aac ", "MPEG-4 AAC"),
    ("MAC3", "MACE 3:1"),
    ("MAC6", "MACE 6:1"),
    ("ulaw", "μLaw 2:1"),
    ("alaw", "aLaw 2:1"),
    (".mp1", "MPEG-1 or 2, Layer 1 audio"),
    (".mp2", "MPEG-1 or 2, Layer 2 audio"),
    (".mp3", "MPEG-1 or 2, Layer 3 audio"),
    ("alac", "Apple Lossless"),
    ("opus", "Opus Interactive Audio Codec"),
];
const UNKNOW_FORMAT_ID_MESSAGE: &str = "Unknown format: ";

const MPEG_4_AAC_OBJECT_TYPES: [&str; 46] = [
    "Null",
    "AAC Main",
    "AAC LC (Low Complexity)",
    "AAC SSR (Scalable Sample Rate)",
    "AAC LTP (Long Term Prediction)",
    "SBR (Spectral Band Replication)",
    "AAC Scalable",
    "TwinVQ",
    "CELP (Code Excited Linear Prediction)",
    "HXVC (Harmonic Vector eXcitation Coding)",
    "Reserved",
    "Reserved",
    "TTSI (Text-To-Speech Interface)",
    "Main Synthesis",
    "Wavetable Synthesis",
    "General MIDI",
    "Algorithmic Synthesis and Audio Effects",
    "ER (Error Resilient) AAC LC",
    "Reserved",
    "ER AAC LTP",
    "ER AAC Scalable",
    "ER TwinVQ",
    "ER BSAC (Bit-Sliced Arithmetic Coding)",
    "ER AAC LD (Low Delay)",
    "ER CELP",
    "ER HVXC",
    "ER HILN (Harmonic and Individual Lines Noise)",
    "ER Parametric",
    "SSC (SinuSoidal Coding)",
    "PS (Parametric Stereo)",
    "MPEG Surround",
    "(Escape value)",
    "Layer-1",
    "Layer-2",
    "Layer-3",
    "DST (Direct Stream Transfer)",
    "ALS (Audio Lossless)",
    "SLS (Scalable LosslesS)",
    "SLS non-core",
    "ER AAC ELD (Enhanced Low Delay)",
    "SMR (Symbolic Music Representation) Simple",
    "SMR Main",
    "USAC (Unified Speech and Audio Coding) SBR)",
    "SAOC (Spatial Audio Object Coding)",
    "LD MPEG Surround",
    "USAC",
];

const FORMAT_FLAGS_OTHER_MESSAGE_START: &str = "Meaning is format dependent. See ";
const FORMAT_FLAGS_OTHER_MESSAGE_MIDDLE: &str = "file specs. Raw flag mask is:";

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let sample_rate = take_first_eight_bytes_as_float(&mut chunk_data, Endian::Big)?;
    let format_string =
        take_first_number_of_bytes_as_string(&mut chunk_data, FORMAT_ID_LENGTH_IN_BYTES)?;
    let format_flag_mask = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
    let bytes_per_packet = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
    let frames_per_packet =
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
    let channels_per_frame =
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
    let bits_per_channel = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;

    let format_long_names: HashMap<&str, &str> = HashMap::from(FORMAT_ID_LONG_FORM);
    let format_id = format_string.as_str();
    let format = match format_long_names.get(format_id) {
        Some(format) => format.to_string(),
        None => [UNKNOW_FORMAT_ID_MESSAGE, &format_id].concat(),
    };

    let output_values: Value = upon::value! {
        sample_rate: format_sample_rate(sample_rate),
        format: format,
        format_flags: get_format_flags_from_mask(format_flag_mask, &format_id),
        bytes_per_packet: bytes_per_packet,
        frames_per_packet: frames_per_packet,
        channels_per_frame: channels_per_frame,
        bits_per_channel: bits_per_channel,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Mandatory,
        text: formated_output,
    })
}

fn format_sample_rate(sample_rate: f64) -> String {
    let sample_rate_in_khz = sample_rate / 1000.0;

    if sample_rate_in_khz == sample_rate_in_khz.floor() {
        format!("{:#.0}", sample_rate_in_khz)
    } else {
        format!("{:#.1}", sample_rate_in_khz)
    }
}

fn get_format_flags_from_mask(flag_mask: u32, format_id: &str) -> String {
    match format_id {
        FORMAT_ID_LINEAR_PCM => {
            let precision = if flag_mask & 1 == 0 {
                FORMAT_FLAG_UNSIGNED_INTEGER
            } else {
                FORMAT_FLAG_FLOAT
            };

            let endianness = if (flag_mask >> 1) & 1 == 0 {
                BIG_ENDIAN
            } else {
                LITTLE_ENDIAN
            };
            format!("{}, {}", endianness, precision)
        }
        FORMAT_ID_MPEG_4_AAC => MPEG_4_AAC_OBJECT_TYPES[flag_mask as usize].to_string(),
        _ => format!(
            "{} {} {} {}",
            FORMAT_FLAGS_OTHER_MESSAGE_START,
            format_id,
            FORMAT_FLAGS_OTHER_MESSAGE_MIDDLE,
            flag_mask
        ),
    }
}
