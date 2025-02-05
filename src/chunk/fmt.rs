use crate::byteio::{take_first_four_bytes_as_unsigned_integer, take_first_two_bytes_as_unsigned_integer};
use crate::template::Template;
use std::error::Error;
use upon::Value;

const TEMPLATE_NAME: &str = "fmt";
const TEMPLATE_PATH: &str = include_str!("../templates/wave/fmt.tmpl");
const FORMAT_CHUNK_SIZE_IF_NO_EXTENSION: usize = 16;
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
    pub template_name: &'static str,
    pub template_path: &'static str,
    pub format_code: String,
    pub number_of_channels: u16,
    pub samples_per_second: u32,
    pub average_data_rate: u32,
    pub data_block_size: u16,
    pub bits_per_sample: u16,
    pub valid_bits_per_sample: u16,
    pub speaker_position_mask: u32,
    pub subformat_guid: [u8; GUID_LENGTH_IN_BYTES],
}

impl FmtFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let chunk_size = chunk_data.len();

        let format_code = get_format_name_from_format_id(take_first_two_bytes_as_unsigned_integer(&mut chunk_data)?);
        let number_of_channels = take_first_two_bytes_as_unsigned_integer(&mut chunk_data)?;
        let samples_per_second = take_first_four_bytes_as_unsigned_integer(&mut chunk_data)?;
        let average_data_rate = take_first_four_bytes_as_unsigned_integer(&mut chunk_data)?;
        let data_block_size = take_first_two_bytes_as_unsigned_integer(&mut chunk_data)?;
        let bits_per_sample = take_first_two_bytes_as_unsigned_integer(&mut chunk_data)?;

        let mut extension_size: u16 = Default::default();

        if chunk_size > FORMAT_CHUNK_SIZE_IF_NO_EXTENSION {
            extension_size = take_first_two_bytes_as_unsigned_integer(&mut chunk_data)?;
        }

        let mut valid_bits_per_sample: u16 = Default::default();
        let mut speaker_position_mask: u32 = Default::default();
        let mut subformat_guid: [u8; GUID_LENGTH_IN_BYTES] = Default::default();

        if extension_size > 0 {
            valid_bits_per_sample = take_first_two_bytes_as_unsigned_integer(&mut chunk_data)?;
            speaker_position_mask = take_first_four_bytes_as_unsigned_integer(&mut chunk_data)?;
            subformat_guid.copy_from_slice(chunk_data.as_slice());
        }

        Ok(Self {
            template_name: TEMPLATE_NAME,
            template_path: TEMPLATE_PATH,
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

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, Box<dyn Error>> {
        template.add_chunk_template(self.template_name, self.template_path)?;

        let wave_output_values: Value = upon::value! {
            format_code: &self.format_code,
            number_of_channels: &self.number_of_channels,
            samples_per_second: &(self.samples_per_second as f64 / 1000.0),
            bits_per_sample: &self.bits_per_sample,
            average_data_rate: &(self.average_data_rate as f64 / 1000.0),
            data_block_size: self.data_block_size,
            valid_bits_per_sample: &self.valid_bits_per_sample,
            speaker_position_mask: format_speaker_position(self.speaker_position_mask),
            subformat_guid: format_guid(self.subformat_guid.clone())
        };

        Ok(template.get_wave_chunk_output(self.template_name, wave_output_values)?)
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

fn format_guid(guid_bytes: [u8; GUID_LENGTH_IN_BYTES]) -> String {
    if guid_bytes.len() != GUID_LENGTH_IN_BYTES {
        return NO_GUID_FOUND_MESSAGE.to_string();
    }

    let formated_guid: Vec<String> = guid_bytes.iter().map(|byte| format!("{:X}", byte)).collect();

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
