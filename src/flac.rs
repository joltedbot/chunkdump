use crate::chunks::{Chunk, Section};
use crate::errors::LocalError;
use crate::formating::{canonicalize_file_path, format_file_size_as_string, get_file_name_from_file_path};
use crate::template::get_file_chunk_output;
use byte_unit::rust_decimal::prelude::Zero;
use claxon::metadata::Tags;
use claxon::{FlacReader, FlacReaderOptions};
use serde::Serialize;
use std::error::Error;
use std::fs::{metadata, File};
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("templates/files/flac.tmpl");
const SECONDS_PER_MINUTE: u64 = 60;

#[derive(Debug, Serialize)]
struct VorbisTag {
    id: String,
    spacer: String,
    value: String,
}

pub fn get_metadata_from_file(flac_file_path: &str) -> Result<Vec<Chunk>, Box<dyn Error>> {
    let file_stream = FlacReader::open_ext(
        flac_file_path,
        FlacReaderOptions {
            metadata_only: true,
            read_vorbis_comment: true,
        },
    )?;

    let formated_output = get_metadata_from_flac(flac_file_path, file_stream)?;
    Ok(vec![formated_output])
}

fn get_metadata_from_flac(file_path: &str, file_stream: FlacReader<File>) -> Result<Chunk, Box<dyn Error>> {
    let file_size = format_file_size_as_string(metadata(file_path)?.len());
    let stream_info = file_stream.streaminfo();
    let vorbis_vendor = file_stream.vendor();
    let vorbis_tags = get_vorbis_comment_tags(&file_stream);
    let total_samples = stream_info.samples.unwrap_or_default();

    let wave_output_values: Value = upon::value! {
        file_name: get_file_name_from_file_path(file_path)?,
        file_path: canonicalize_file_path(file_path)?,
        file_size: file_size,
        duration: format_estimated_duration(total_samples, stream_info.sample_rate),
        min_block_size: stream_info.min_block_size,
        max_block_size: stream_info.max_block_size,
        min_frame_size: stream_info.min_frame_size.unwrap_or_default(),
        max_frame_size: stream_info.max_frame_size.unwrap_or_default(),
        sample_rate: stream_info.sample_rate as f64 / 1000.0,
        channels: stream_info.channels,
        bits_per_sample: stream_info.bits_per_sample,
        total_samples: total_samples,
        md5_sum: format_md5_sum_from_bytes(stream_info.md5sum),
        vorbis_vendor: vorbis_vendor,
        vorbis_tags: vorbis_tags,
    };

    let output = Chunk {
        section: Section::Header,
        text: get_file_chunk_output(TEMPLATE_CONTENT, wave_output_values)?,
    };

    Ok(output)
}

fn get_vorbis_comment_tags(file_stream: &FlacReader<File>) -> Vec<VorbisTag> {
    let mut vorbis_tags: Vec<VorbisTag> = vec![];
    let longest_tag = get_longest_tag(file_stream.tags()).unwrap_or_default();

    file_stream.tags().for_each(|(k, v)| {
        let mut spacer = String::new();
        if longest_tag > k.len() {
            spacer = " ".repeat(longest_tag - k.len());
        }

        vorbis_tags.push(VorbisTag {
            id: k.to_string(),
            spacer,
            value: v.to_string(),
        });
    });

    vorbis_tags
}

fn get_longest_tag(tags: Tags) -> Result<usize, LocalError> {
    match tags.max_by_key(|tag| tag.0.len()) {
        Some(tag) => Ok(tag.0.len()),
        None => Err(LocalError::ErrorParsingVorbisTags),
    }
}

fn format_md5_sum_from_bytes(bytes: [u8; 16]) -> String {
    bytes
        .iter()
        .fold(String::new(), |acc, byte| acc + format!("{:02x}", byte).as_str())
}

fn format_estimated_duration(total_sample: u64, samples_per_second: u32) -> String {
    let mut duration: String = String::new();
    if !total_sample.is_zero() && !samples_per_second.is_zero() {
        let duration_seconds: u64 = total_sample / samples_per_second as u64;
        let whole_minutes = duration_seconds / SECONDS_PER_MINUTE;
        let remaining_seconds = duration_seconds % SECONDS_PER_MINUTE;
        duration = format!("{}:{}", whole_minutes, remaining_seconds);
    }
    duration
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_correct_md5_format_from_passed_bytes() {
        let input_bytes: [u8; 16] = [0x1A; 16];
        let correct_md5_format = String::from("1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a");
        let result = format_md5_sum_from_bytes(input_bytes);
        assert_eq!(correct_md5_format, result);
    }

    #[test]
    fn returns_the_correct_estimated_duration_format_from_correct_sample_data() {
        let total_sample = 15434143;
        let sample_rate = 44100;
        let correct_estimated_duration_format = String::from("5:49");
        let result = format_estimated_duration(total_sample, sample_rate);
        assert_eq!(correct_estimated_duration_format, result);
    }

    #[test]
    fn returns_empty_duration_format_from_zero_total_samples() {
        let total_sample = 0;
        let sample_rate = 48000;
        let correct_estimated_duration_format = String::from("");
        let result = format_estimated_duration(total_sample, sample_rate);
        assert_eq!(correct_estimated_duration_format, result);
    }

    #[test]
    fn returns_empty_duration_format_from_zero_sample_rate() {
        let total_sample = 15434143;
        let sample_rate = 0;
        let correct_estimated_duration_format = String::from("");
        let result = format_estimated_duration(total_sample, sample_rate);
        assert_eq!(correct_estimated_duration_format, result);
    }

    #[test]
    fn returns_correct_duration_format_from_max_sample_values() {
        let total_sample = u64::MAX;
        let sample_rate = u32::MAX;
        let correct_estimated_duration_format = String::from("71582788:17");
        let result = format_estimated_duration(total_sample, sample_rate);
        assert_eq!(correct_estimated_duration_format, result);
    }

    #[test]
    fn returns_correct_longest_flag_length_from_valid_tags() {
        let the_tags = [
            (String::from("abcsadal"), 2),
            (String::from("lajlmnop"), 3),
            (String::from("xyiausda"), 4),
        ];
        let tags: Tags = Tags::new(&the_tags);
        let longest_tag = get_longest_tag(tags).unwrap();
        assert_eq!(longest_tag, 4);
    }
}
