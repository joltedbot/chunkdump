use crate::byteio::{take_first_four_bytes_as_integer, take_first_two_bytes_as_integer};
use crate::fileio::{read_bytes_from_file, read_chunk_size_from_file};
use std::error::Error;
use std::fs::File;

const FORMAT_CHUNK_SIZE_IF_NO_EXTENSION: u32 = 16;
const PCM_FORMAT_ID: u16 = 1;
const PCM_FORMAT_NAME: &str = "PCM";
const IEEE_FORMAT_FLOAT_ID: u16 = 3;
const IEEE_FORMAT_FLOAT_NAME: &str = "IEEE float";
const ALAW_FORMAT_ID: u16 = 6;
const ALAW_FORMAT_NAME: &str = "8-bit ITU-T G.711 A-law";
const MULAW_FORMAT_ID: u16 = 7;
const MULAW_FORMAT_NAME: &str = "8-bit ITU-T G.711 µ-law";
const EXTENSIBLE_FORMAT_ID: u16 = 65279;
const EXTENSIBLE_FORMAT_NAME: &str = "Determined by SubFormat";
const UNKOWN_FORMAT: &str = "Unknown Format ID: ";
const GUID_LENGTH_IN_BYTES: usize = 16;
const NO_GUID_FOUND_MESSAGE: &str = "N/A";
const OUTPUT_HEADER: &str = "\n-------------\nFormat (fmt) Chunk Details:\n-------------";
const OUTPUT_FORMAT_CODE_MESSAGE: &str = "Format Code: ";
const OUTPUT_NUMBER_OF_CHANNELS_MESSAGE: &str = "Number of Channels: ";
const OUTPUT_SAMPLES_PER_SECOND_MESSAGE: &str = "Sample Rate: ";
const OUTPUT_SAMPLES_PER_SECOND_UNIT: &str = "kHz";
const OUTPUT_BITS_PER_SAMPLE_MESSAGE: &str = "Bit Depth: ";
const OUTPUT_BITS_PER_SAMPLE_UNIT: &str = " bits";
const OUTPUT_AVERAGE_DATA_RATE_MESSAGE: &str = "Average Data Rate: ";
const OUTPUT_AVERAGE_DATA_RATE_UNIT: &str = " kB/Second";
const OUTPUT_DATA_BLOCK_SIZE_MESSAGE: &str = "Data Block Size: ";
const OUTPUT_DATA_BLOCK_SIZE_UNIT: &str = " bytes";
const OUTPUT_VALID_BITS_PER_SAMPLE_MESSAGE: &str = "Valid Bits per Sample: ";
const OUTPUT_VALID_BITS_PER_SAMPLE_UNIT: &str = " bits";
const OUTPUT_SPEAKER_POSITION_MESSAGE: &str = "Speaker Position Mask: ";
const OUTPUT_GUID_MESSAGE: &str = "GUID: ";
const SPEAKER_POSITION_MASK_BIT_MEANING: [&str; 18] = [
    "Front Left",
    "Front Tight",
    "Front Center",
    "Low Frequency",
    "Back Left",
    "Back Tight",
    "Front Left Of Center",
    "Front Tight Of Center",
    "Back Center",
    "Side Left",
    "Side Tight",
    "Top Center",
    "Top Front Left",
    "Top Front Center",
    "Top Front Tight",
    "Top Back Left",
    "Top Back Center",
    "Top Back Tight",
];

#[derive(Debug, Clone, Default)]
pub struct FmtFields {
    pub format_code: String,
    pub number_of_channels: u16,
    pub samples_per_second: u32,
    pub average_data_rate: u32,
    pub data_block_size: u16,
    pub bits_per_sample: u16,
    pub valid_bits_per_sample: u16,
    pub speaker_position_mask: u32,
    pub subformat_guid: Vec<u8>,
}

impl FmtFields {
    pub fn new(wave_file: &mut File) -> Result<Self, Box<dyn Error>> {
        let chunk_size = read_chunk_size_from_file(wave_file)?;
        let mut fmt_data = read_bytes_from_file(wave_file, chunk_size as usize)?;

        let format_code =
            get_format_name_from_format_id(take_first_two_bytes_as_integer(&mut fmt_data)?);
        let number_of_channels = take_first_two_bytes_as_integer(&mut fmt_data)?;
        let samples_per_second = take_first_four_bytes_as_integer(&mut fmt_data)?;
        let average_data_rate = take_first_four_bytes_as_integer(&mut fmt_data)?;
        let data_block_size = take_first_two_bytes_as_integer(&mut fmt_data)?;
        let bits_per_sample = take_first_two_bytes_as_integer(&mut fmt_data)?;

        let mut extension_size: u16 = Default::default();

        if chunk_size > FORMAT_CHUNK_SIZE_IF_NO_EXTENSION {
            extension_size = take_first_two_bytes_as_integer(&mut fmt_data)?;
        }

        let mut valid_bits_per_sample: u16 = Default::default();
        let mut speaker_position_mask: u32 = Default::default();
        let mut subformat_guid: Vec<u8> = vec![];

        if extension_size > 0 {
            valid_bits_per_sample = take_first_two_bytes_as_integer(&mut fmt_data)?;
            speaker_position_mask = take_first_four_bytes_as_integer(&mut fmt_data)?;
            subformat_guid = fmt_data;
        }

        Ok(Self {
            format_code,
            number_of_channels,
            samples_per_second,
            average_data_rate,
            data_block_size,
            bits_per_sample,
            valid_bits_per_sample,
            speaker_position_mask,
            subformat_guid,
        })
    }

    pub fn get_metadata_output(&self) -> Vec<String> {
        let mut fmt_data: Vec<String> = vec![];

        fmt_data.push(OUTPUT_HEADER.to_string());
        fmt_data.push(format!(
            "{}{}",
            OUTPUT_FORMAT_CODE_MESSAGE, self.format_code
        ));
        fmt_data.push(format!(
            "{}{}",
            OUTPUT_NUMBER_OF_CHANNELS_MESSAGE, self.number_of_channels
        ));
        fmt_data.push(format!(
            "{} {} {}",
            OUTPUT_SAMPLES_PER_SECOND_MESSAGE,
            self.samples_per_second as f64 / 1000.0,
            OUTPUT_SAMPLES_PER_SECOND_UNIT
        ));
        fmt_data.push(format!(
            "{}{}{}",
            OUTPUT_BITS_PER_SAMPLE_MESSAGE, self.bits_per_sample, OUTPUT_BITS_PER_SAMPLE_UNIT
        ));
        fmt_data.push(format!(
            "{}{}{}",
            OUTPUT_AVERAGE_DATA_RATE_MESSAGE,
            self.average_data_rate as f64 / 1000.0,
            OUTPUT_AVERAGE_DATA_RATE_UNIT
        ));
        fmt_data.push(format!(
            "{}{}{}",
            OUTPUT_DATA_BLOCK_SIZE_MESSAGE, self.data_block_size, OUTPUT_DATA_BLOCK_SIZE_UNIT
        ));

        fmt_data.push(format!(
            "{}{}{}",
            OUTPUT_VALID_BITS_PER_SAMPLE_MESSAGE,
            self.valid_bits_per_sample,
            OUTPUT_VALID_BITS_PER_SAMPLE_UNIT
        ));
        fmt_data.push(format!(
            "{}{}",
            OUTPUT_SPEAKER_POSITION_MESSAGE,
            format_speaker_position(self.speaker_position_mask)
        ));
        fmt_data.push(format!(
            "{}{:#?}",
            OUTPUT_GUID_MESSAGE,
            format_guid(self.subformat_guid.clone())
        ));

        fmt_data
    }
}

fn get_format_name_from_format_id(format_id: u16) -> String {
    match format_id {
        PCM_FORMAT_ID => PCM_FORMAT_NAME.to_string(),
        IEEE_FORMAT_FLOAT_ID => IEEE_FORMAT_FLOAT_NAME.to_string(),
        ALAW_FORMAT_ID => ALAW_FORMAT_NAME.to_string(),
        MULAW_FORMAT_ID => MULAW_FORMAT_NAME.to_string(),
        EXTENSIBLE_FORMAT_ID => EXTENSIBLE_FORMAT_NAME.to_string(),
        _ => format!("{} {}", UNKOWN_FORMAT, format_id),
    }
}

fn format_guid(guid_bytes: Vec<u8>) -> String {
    if guid_bytes.len() != GUID_LENGTH_IN_BYTES {
        return NO_GUID_FOUND_MESSAGE.to_string();
    }

    let formated_guid: Vec<String> = guid_bytes
        .iter()
        .map(|byte| format!("{:X}", byte))
        .collect();

    formated_guid.join("")
}

fn format_speaker_position(speaker_position_mask: u32) -> String {
    let mut positions: Vec<String> = Default::default();

    for position in 0..SPEAKER_POSITION_MASK_BIT_MEANING.len() {
        if (speaker_position_mask & (1 << position)) > 0 {
            positions.push(format!(
                " - {}",
                SPEAKER_POSITION_MASK_BIT_MEANING[position].to_string()
            ));
        }
    }

    if !positions.is_empty() {
        positions.insert(0, "".to_string());
    }

    positions.join("\n")
}
