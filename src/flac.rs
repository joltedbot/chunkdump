use crate::chunks::{Chunk, Section};
use crate::errors::LocalError;
use crate::fileio::{canonicalize_file_path, get_file_name_from_file_path};
use crate::formating::format_file_size_as_string;
use crate::template::Template;
use byte_unit::rust_decimal::prelude::Zero;
use claxon::metadata::Tags;
use claxon::{FlacReader, FlacReaderOptions};
use serde::Serialize;
use std::error::Error;
use std::fs::{metadata, File};
use upon::Value;

const TEMPLATE_NAME: &str = "flac";
const TEMPLATE_CONTENT: &str = include_str!("templates/files/flac.tmpl");
const SECONDS_PER_MINUTE: u64 = 60;

#[derive(Debug, Serialize)]
struct VorbisTag {
    id: String,
    spacer: String,
    value: String,
}

pub fn get_metadata_from_file(flac_file_path: &str) -> Result<Vec<Chunk>, Box<dyn Error>> {
    let mut template = Template::new();
    let file_stream = FlacReader::open_ext(
        flac_file_path,
        FlacReaderOptions {
            metadata_only: true,
            read_vorbis_comment: true,
        },
    )?;

    let formated_output = format_data_for_output(&mut template, flac_file_path, file_stream)?;
    Ok(vec![formated_output])
}

fn format_data_for_output(
    template: &mut Template,
    file_path: &str,
    file_stream: FlacReader<File>,
) -> Result<Chunk, Box<dyn Error>> {
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
        text: template.get_wave_chunk_output(TEMPLATE_NAME, TEMPLATE_CONTENT, wave_output_values)?,
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
    if !total_sample.is_zero() {
        let duration_seconds: u64 = total_sample / samples_per_second as u64;
        let whole_minutes = duration_seconds / SECONDS_PER_MINUTE;
        let remaining_seconds = duration_seconds % SECONDS_PER_MINUTE;
        duration = format!("{}:{}", whole_minutes, remaining_seconds);
    }
    duration
}
