use crate::chunks::id3::get_metadata;
use crate::errors::LocalError;
use crate::file_types::Mp3SubType;
use crate::fileio::{get_file_metadata, read_bytes_from_file, skip_over_bytes_in_file};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use std::fs::File;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/file_types/mp3.tmpl");
const HEADER_TEMPLATE_CONTENT: &str = include_str!("../templates/file_types/mp3_header.tmpl");
const ID3_HEADER_BYTES_BEFORE_ID3_SIZE_BYTES: usize = 6;
const ID3_HEADER_SIZE_LENGTH_IN_BYTES: usize = 4;
const MP3_HEADER_LENGTH_IN_BYTES: usize = 4;

const MAX_SYNC_SAFE_INTEGER_BYTE_VALUE: u8 = 0x7F;

const MPEG_AUDIO_VERSION_IDS: [&str; 4] = [
    "MPEG Version 2.5",
    "Reserved",
    "MPEG Version 2 (ISO/IEC 13818-3)",
    "MPEG Version 1 (ISO/IEC 11172-3)",
];

const VERSION_RESERVED_VALUE: u8 = 1;

const MPEG_LAYER_DESCRIPTION: [&str; 4] = ["Reserved", "Layer III", "Layer II", "Layer I"];

const LAYER_RESERVED_VALUE: u8 = 0;

const MPEG_BITRATE_INDEX: [[[u16; 3]; 2]; 15] = [
    [[0, 0, 0], [0, 0, 0]],
    [[32, 32, 32], [32, 8, 8]],
    [[64, 48, 40], [48, 16, 16]],
    [[96, 56, 48], [56, 24, 24]],
    [[128, 64, 56], [64, 32, 32]],
    [[60, 80, 64], [80, 40, 40]],
    [[192, 96, 80], [96, 48, 48]],
    [[224, 112, 96], [112, 56, 56]],
    [[256, 128, 112], [128, 64, 64]],
    [[288, 160, 128], [144, 80, 80]],
    [[320, 192, 160], [160, 96, 96]],
    [[352, 224, 192], [176, 112, 112]],
    [[384, 256, 224], [192, 128, 128]],
    [[416, 320, 256], [224, 144, 144]],
    [[448, 384, 320], [256, 160, 160]],
];

const MPEG_SAMPLE_RATE_INDEX: [[u16; 4]; 4] = [
    [11025, 0, 22050, 44100],
    [12000, 0, 24000, 48000],
    [8000, 0, 16000, 32000],
    [0, 0, 0, 0],
];

const MPEG_CHANNEL_MODE: [&str; 4] = [
    "Stereo",
    "Joint stereo (Stereo)",
    "Dual channel (Stereo)",
    "Single channel (Mono)",
];

const MPEG_MODE_EXTENSION_LAYERS_I_AND_II: [&str; 4] = [
    "Bands 4 to 31",
    "Bands 8 to 31",
    "Bands 12 to 31",
    "Bands 16 to 31",
];
const MPEG_MODE_EXTENSION_LAYERS_III: [(&str, &str); 4] =
    [("Off", "Off"), ("On", "Off"), ("Off", "On"), ("On", "On")];

const MPEG_EMPHASIS: [&str; 4] = ["None", "50/15 ms", "Reserved", "CCIT J.17"];

pub fn get_metadata_from_file(
    file_path: &str,
    subtype: Mp3SubType,
    mandatory_sections_only: bool,
) -> Result<Vec<OutputEntry>, Box<dyn Error>> {
    let mut mp3_file = File::open(file_path)?;
    let file_metadata = get_file_metadata(file_path, &mp3_file, TEMPLATE_CONTENT)?;
    let mut output = vec![file_metadata];

    if subtype == Mp3SubType::ID3 {
        if !mandatory_sections_only {
            let id3_metadata = get_metadata(file_path)?;
            output.push(id3_metadata);
        }
        skip_over_id3_data_in_file(&mut mp3_file)?;
    }

    let mp3_header_bytes = read_bytes_from_file(&mut mp3_file, MP3_HEADER_LENGTH_IN_BYTES)?;
    let header_metadata = get_header_metadata(mp3_header_bytes)?;
    output.push(header_metadata);

    Ok(output)
}

fn get_header_metadata(header_bytes: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let version = (header_bytes[1] >> 3) & 3;
    let layer = (header_bytes[1] >> 1) & 3;
    let protected = header_bytes[1] & 1;
    let bitrate = header_bytes[2] >> 4;
    let sample_rate = (header_bytes[2] >> 2) & 3;
    let private_bit = header_bytes[2] & 1;
    let channel_mode = header_bytes[3] >> 6;
    let mode_extension = (header_bytes[3] >> 4) & 3;
    let copyright = (header_bytes[3] >> 3) & 1;
    let original = (header_bytes[3] >> 2) & 1;
    let emphasis = header_bytes[3] & 3;

    let mode_extension_values = get_mode_extension_from_index(mode_extension, layer);

    let output_values: Value = upon::value! {
        version: MPEG_AUDIO_VERSION_IDS[version as usize],
        layer: MPEG_LAYER_DESCRIPTION[layer as usize],
        protected: format_bit_as_bool_string(protected),
        bitrate: get_bitrate_from_index(bitrate, version, layer)?,
        sample_rate: get_sample_rate_from_index(sample_rate, version)?,
        private_bit: format_bit_as_bool_string(private_bit),
        channel_mode: MPEG_CHANNEL_MODE[channel_mode as usize],
        joint_stereo: mode_extension_values.0,
        intensity_stereo: mode_extension_values.1,
        mid_side_stereo: mode_extension_values.2,
        copyright: format_bit_as_bool_string(copyright),
        original: format_bit_as_bool_string(original),
        emphasis: MPEG_EMPHASIS[emphasis as usize],
    };

    let formated_output = get_file_chunk_output(HEADER_TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Mandatory,
        text: formated_output,
    })
}

fn skip_over_id3_data_in_file(mp3_file: &mut File) -> Result<(), Box<dyn Error>> {
    skip_over_bytes_in_file(mp3_file, ID3_HEADER_BYTES_BEFORE_ID3_SIZE_BYTES)?;

    let id3_chunk_size_bytes = read_bytes_from_file(mp3_file, ID3_HEADER_SIZE_LENGTH_IN_BYTES)?;
    let id3_chunk_size = u32_integer_from_sync_safe_integer(id3_chunk_size_bytes)?;
    let remaining_ide3_bytes = id3_chunk_size as usize;

    skip_over_bytes_in_file(mp3_file, remaining_ide3_bytes)?;

    Ok(())
}

fn u32_integer_from_sync_safe_integer(sync_safe_integer_bytes: Vec<u8>) -> Result<u32, LocalError> {
    if sync_safe_integer_bytes.iter().max().unwrap_or(&0xFF) > &MAX_SYNC_SAFE_INTEGER_BYTE_VALUE {
        return Err(LocalError::MP3SyncSafeIntegerOverflow(
            MAX_SYNC_SAFE_INTEGER_BYTE_VALUE,
        ));
    }

    let byte0 = sync_safe_integer_bytes[3] as u32; // MSB
    let byte1 = sync_safe_integer_bytes[2] as u32;
    let byte2 = sync_safe_integer_bytes[1] as u32;
    let byte3 = sync_safe_integer_bytes[0] as u32; // LSB

    let mut final_integer: u32 = 0;
    final_integer |= byte0;
    final_integer |= byte1 << 7;
    final_integer |= byte2 << 14;
    final_integer |= byte3 << 21;

    Ok(final_integer)
}

fn get_sample_rate_from_index(sample_rate: u8, version: u8) -> Result<String, LocalError> {
    if sample_rate as usize >= MPEG_SAMPLE_RATE_INDEX.len() {
        return Err(LocalError::MP3SampleRateIndexOverflow(sample_rate));
    }

    if version as usize >= MPEG_SAMPLE_RATE_INDEX[0].len() {
        return Err(LocalError::MP3VersionIndexOverflow(version));
    }

    let raw_rate = MPEG_SAMPLE_RATE_INDEX[sample_rate as usize][version as usize];
    let sample_rate_in_khz = f32::from(raw_rate) / 1000.0;

    if sample_rate_in_khz == sample_rate_in_khz.floor() {
        Ok(format!("{:#.0}", sample_rate_in_khz))
    } else {
        Ok(format!("{:#.1}", sample_rate_in_khz))
    }
}

fn format_bit_as_bool_string(bit: u8) -> String {
    if bit == 1 {
        "True".to_string()
    } else {
        "False".to_string()
    }
}

fn get_mode_extension_from_index(mode_extension: u8, layer: u8) -> (String, String, String) {
    let mut joint_stereo = String::new();
    let mut intensity_stereo = String::new();
    let mut mid_side_stereo = String::new();

    match layer {
        3 => {
            joint_stereo = MPEG_MODE_EXTENSION_LAYERS_I_AND_II[mode_extension as usize].to_string()
        }
        2 => {
            joint_stereo = MPEG_MODE_EXTENSION_LAYERS_I_AND_II[mode_extension as usize].to_string()
        }
        1 => {
            let extenstion_values = MPEG_MODE_EXTENSION_LAYERS_III[mode_extension as usize];
            intensity_stereo = extenstion_values.0.to_string();
            mid_side_stereo = extenstion_values.1.to_string();
        }
        _ => (),
    }

    (joint_stereo, intensity_stereo, mid_side_stereo)
}

fn get_bitrate_from_index(bitrate: u8, version: u8, layer: u8) -> Result<u16, LocalError> {
    if bitrate as usize >= MPEG_BITRATE_INDEX.len() {
        return Err(LocalError::MP3BitrateIndexOverflow(bitrate));
    }

    if version == VERSION_RESERVED_VALUE || layer == LAYER_RESERVED_VALUE {
        return Ok(0);
    }

    let version_index: [usize; 4] = [1, 4, 1, 0];
    if version as usize >= version_index.len() {
        return Err(LocalError::MP3VersionIndexOverflow(version));
    }

    let layer_index: [usize; 4] = [4, 2, 1, 0];
    if layer as usize >= layer_index.len() {
        return Err(LocalError::MP3LayerIndexOverflow(layer));
    }

    Ok(
        MPEG_BITRATE_INDEX[bitrate as usize][version_index[version as usize]]
            [layer_index[layer as usize]],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_true_string_when_boolean_flag_bit_is_set() {
        let result = format_bit_as_bool_string(1 as u8);
        assert_eq!(result, "True");
    }

    #[test]
    fn return_true_string_when_boolean_flag_bit_is_unset() {
        let result = format_bit_as_bool_string(0 as u8);
        assert_eq!(result, "False");
    }

    #[test]
    fn return_correct_integer_from_sync_safe_integer() {
        let test_sync_safe_integer_bytes: Vec<u8> = vec![0x25, 0x37, 0x27, 0x2C];
        let correct_result = 78500780;
        let result = u32_integer_from_sync_safe_integer(test_sync_safe_integer_bytes).unwrap();
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_expected_error_from_sync_safe_integer_when_some_bytes_exceeds_the_max_value() {
        let test_sync_safe_integer_bytes: Vec<u8> = vec![0xFF, 0x00, 0x00, 0x00];
        let expected_error =
            LocalError::MP3SyncSafeIntegerOverflow(MAX_SYNC_SAFE_INTEGER_BYTE_VALUE);
        let result = u32_integer_from_sync_safe_integer(test_sync_safe_integer_bytes)
            .expect_err("u32_integer_from_sync_safe_integer should have returned an error");
        assert_eq!(result, expected_error);
    }

    #[test]
    fn return_correct_sample_rate_string_from_valid_raw_sample_rate_with_whole_khz() {
        let test_sample_rate = 2;
        let test_version = 2;
        let correct_sample_rate = String::from("16");
        let sample_rate_result =
            get_sample_rate_from_index(test_sample_rate, test_version).unwrap();
        assert_eq!(sample_rate_result, correct_sample_rate);
    }

    #[test]
    fn return_correct_sample_rate_string_from_valid_raw_sample_rate_with_decimal_khz() {
        let test_sample_rate = 0;
        let test_version = 3;
        let correct_sample_rate = String::from("44.1");
        let sample_rate_result =
            get_sample_rate_from_index(test_sample_rate, test_version).unwrap();
        assert_eq!(sample_rate_result, correct_sample_rate);
    }

    #[test]
    fn return_expected_error_from_out_of_range_sample_rate_when_getting_sample_rate_from_index() {
        let test_sample_rate = 10;
        let test_version = 2;
        let expected_error = LocalError::MP3SampleRateIndexOverflow(test_sample_rate);
        let sample_rate_result = get_sample_rate_from_index(test_sample_rate, test_version)
            .expect_err("get_sample_rate_from_index should have returned an error");
        assert_eq!(sample_rate_result, expected_error);
    }

    #[test]
    fn return_expected_error_from_out_of_range_version_when_getting_sample_rate_from_index() {
        let test_sample_rate = 2;
        let test_version = 11;
        let expected_error = LocalError::MP3VersionIndexOverflow(test_version);
        let sample_rate_result = get_sample_rate_from_index(test_sample_rate, test_version)
            .expect_err("get_sample_rate_from_index should have returned an error");
        assert_eq!(sample_rate_result, expected_error);
    }

    #[test]
    fn return_correct_bitrate_string_from_valid_raw_bitrate() {
        let test_bitrate = 12;
        let test_version = 3;
        let test_layer = 3;
        let correct_bitrate = 384;
        let bitrate_result =
            get_bitrate_from_index(test_bitrate, test_version, test_layer).unwrap();
        assert_eq!(bitrate_result, correct_bitrate);
    }

    #[test]
    fn return_expected_error_from_out_of_range_raw_bitrate() {
        let test_bitrate = MPEG_BITRATE_INDEX.len() as u8;
        let test_version = 3;
        let test_layer = 3;
        let expected_error = LocalError::MP3BitrateIndexOverflow(test_bitrate);
        let bitrate_result = get_bitrate_from_index(test_bitrate, test_version, test_layer)
            .expect_err("get_bitrate_from_index should have returned an error");
        assert_eq!(bitrate_result, expected_error);
    }

    #[test]
    fn return_expected_error_from_out_of_range_version_when_getting_bitrate_from_index() {
        let test_bitrate = 12;
        let test_version = 12;
        let test_layer = 3;
        let expected_error = LocalError::MP3VersionIndexOverflow(test_version);
        let bitrate_result = get_bitrate_from_index(test_bitrate, test_version, test_layer)
            .expect_err("get_bitrate_from_index should have returned an error");
        assert_eq!(bitrate_result, expected_error);
    }

    #[test]
    fn return_expected_error_from_out_of_range_layer_when_getting_bitrate_from_index() {
        let test_bitrate = 12;
        let test_version = 3;
        let test_layer = 12;
        let expected_error = LocalError::MP3LayerIndexOverflow(test_layer);
        let bitrate_result = get_bitrate_from_index(test_bitrate, test_version, test_layer)
            .expect_err("get_bitrate_from_index should have returned an error");
        assert_eq!(bitrate_result, expected_error);
    }

    #[test]
    fn return_expected_error_from_out_of_range_layer_when_getting_bitrate_from_index_where_version_is_reserved_value(
    ) {
        let test_bitrate = 12;
        let test_version = VERSION_RESERVED_VALUE;
        let test_layer = 3;
        let zero_bitrate = 0;
        let bitrate_result =
            get_bitrate_from_index(test_bitrate, test_version, test_layer).unwrap();
        assert_eq!(bitrate_result, zero_bitrate);
    }

    #[test]
    fn return_expected_error_from_out_of_range_layer_when_getting_bitrate_from_index_where_layre_is_reserved_value(
    ) {
        let test_bitrate = 12;
        let test_version = 3;
        let test_layer = LAYER_RESERVED_VALUE;
        let zero_bitrate = 0;
        let bitrate_result =
            get_bitrate_from_index(test_bitrate, test_version, test_layer).unwrap();
        assert_eq!(bitrate_result, zero_bitrate);
    }

    #[test]
    fn get_correct_mode_extension_joint_stereo_value_from_mode_extension_when_layer_is_layer_1() {
        let test_mode_extension: u8 = 2;
        let test_layer: u8 = 3; // Layer 1 is representing with binary value 3 (and vice versa) for reasons I would really like somebody to explain to me
        let correct_joint_stereo_values = (
            MPEG_MODE_EXTENSION_LAYERS_I_AND_II[test_mode_extension as usize].to_string(),
            String::new(),
            String::new(),
        );
        let joint_stereo_mode_extension =
            get_mode_extension_from_index(test_mode_extension, test_layer);
        assert_eq!(joint_stereo_mode_extension, correct_joint_stereo_values);
    }

    #[test]
    fn get_correct_mode_extension_joint_stereo_value_from_mode_extension_when_layer_is_layer_2() {
        let test_mode_extension: u8 = 2;
        let test_layer: u8 = 2;
        let correct_joint_stereo_values = (
            MPEG_MODE_EXTENSION_LAYERS_I_AND_II[test_mode_extension as usize].to_string(),
            String::new(),
            String::new(),
        );
        let joint_stereo_mode_extension =
            get_mode_extension_from_index(test_mode_extension, test_layer);
        assert_eq!(joint_stereo_mode_extension, correct_joint_stereo_values);
    }

    #[test]
    fn get_correct_mode_extension_joint_stereo_value_from_mode_extension_when_layer_is_layer_3() {
        let test_mode_extension: u8 = 2;
        let test_layer: u8 = 1; // Layer 3is representing with binary value 1 (and vice versa) for reasons I would really like somebody to explain to me
        let correct_joint_stereo_values = (
            String::new(),
            MPEG_MODE_EXTENSION_LAYERS_III[test_mode_extension as usize]
                .0
                .to_string(),
            MPEG_MODE_EXTENSION_LAYERS_III[test_mode_extension as usize]
                .1
                .to_string(),
        );
        let joint_stereo_mode_extension =
            get_mode_extension_from_index(test_mode_extension, test_layer);
        assert_eq!(joint_stereo_mode_extension, correct_joint_stereo_values);
    }

    #[test]
    fn get_correct_mode_extension_joint_stereo_value_from_mode_extension_when_layer_is_out_of_range(
    ) {
        let test_mode_extension: u8 = 2;
        let test_layer: u8 = MPEG_MODE_EXTENSION_LAYERS_I_AND_II.len() as u8;
        let correct_joint_stereo_values = (String::new(), String::new(), String::new());
        let joint_stereo_mode_extension =
            get_mode_extension_from_index(test_mode_extension, test_layer);
        assert_eq!(joint_stereo_mode_extension, correct_joint_stereo_values);
    }
}
