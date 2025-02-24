use crate::errors::LocalError;
use crate::fileio::{canonicalize_file_path, get_file_name_from_file_path, write_out_file_data};
use crate::formating::format_file_size_as_string;
use crate::template::Template;
use byte_unit::rust_decimal::prelude::Zero;
use claxon::{FlacReader, FlacReaderOptions};
use serde::Serialize;
use std::error::Error;
use std::fs::{metadata, File};
use upon::Value;

const TEMPLATE_NAME: &str = "flac";
const TEMPLATE_CONTENT: &str = include_str!("templates/flac/flac.tmpl");
const SECONDS_PER_MINUTE: u64 = 60;
const VORBIS_TAG_ID_PLUS_SPACER_TOTAL_CHARACTERS: &str = "                    ";

#[derive(Debug, Serialize)]
struct VorbisTag {
    id: String,
    spacer: String,
    value: String,
}

pub fn extract_and_output_flac_metadata(flac_file_path: &str, output_file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut template = Template::new();
    let output_lines: Vec<String> = vec![format_data_for_output(&mut template, flac_file_path)?];
    write_out_file_data(output_lines, output_file_path)?;

    Ok(())
}

fn format_data_for_output(template: &mut Template, flac_file_path: &str) -> Result<String, Box<dyn Error>> {
    let file_size = format_file_size_as_string(metadata(flac_file_path)?.len());

    let file_stream = match open_flac_file(flac_file_path) {
        Ok(value) => value,
        Err(e) => return e,
    };

    let stream_info = file_stream.streaminfo();
    let vorbis_vendor = file_stream.vendor();
    let vorbis_tags = get_vorbis_comment_tags(&file_stream);
    let total_samples = stream_info.samples.unwrap_or_default();

    let wave_output_values: Value = upon::value! {
        file_name: get_file_name_from_file_path(flac_file_path)?,
        file_path: canonicalize_file_path(flac_file_path)?,
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

    let formated_output = template.get_wave_chunk_output(TEMPLATE_NAME, TEMPLATE_CONTENT, wave_output_values)?;
    Ok(formated_output)
}

fn open_flac_file(flac_file_path: &str) -> Result<FlacReader<File>, Result<String, Box<dyn Error>>> {
    let file_stream = match FlacReader::open_ext(
        flac_file_path,
        FlacReaderOptions {
            metadata_only: true,
            read_vorbis_comment: true,
        },
    ) {
        Ok(stream_reader) => stream_reader,
        Err(_) => return Err(Err(Box::new(LocalError::InvalidFlacFile(flac_file_path.to_string())))),
    };
    Ok(file_stream)
}

fn get_vorbis_comment_tags(file_stream: &FlacReader<File>) -> Vec<VorbisTag> {
    let vorbis_comments = file_stream.tags();
    let mut vorbis_tags: Vec<VorbisTag> = vec![];

    vorbis_comments.into_iter().for_each(|(k, v)| {
        let mut spacer = VORBIS_TAG_ID_PLUS_SPACER_TOTAL_CHARACTERS.to_string();
        spacer.truncate(VORBIS_TAG_ID_PLUS_SPACER_TOTAL_CHARACTERS.len() - k.len());

        vorbis_tags.push(VorbisTag {
            id: k.to_string(),
            spacer,
            value: v.to_string(),
        });
    });

    vorbis_tags
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
