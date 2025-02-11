use crate::errors::LocalError;
use crate::fileio::{canonicalize_file_path, get_file_name_from_file_path};
use crate::formating::format_file_size_as_string;
use crate::output::write_out_file_data;
use crate::template::Template;
use flac::metadata::{get_vorbis_comment, VorbisComment};
use flac::StreamReader;
use std::error::Error;
use std::fs::File;
use upon::Value;

const TEMPLATE_NAME: &str = "flac";
const TEMPLATE_PATH: &str = include_str!("templates/flac/flac.tmpl");

pub fn output_flac_metadata(
    mut template: Template,
    flac_file_path: String,
    output_file_path: String,
) -> Result<(), Box<dyn Error>> {
    let output_lines: Vec<String> = vec![format_data_for_output(&mut template, flac_file_path)?];
    write_out_file_data(output_lines, output_file_path.clone())?;

    Ok(())
}

fn format_data_for_output(template: &mut Template, flac_file_path: String) -> Result<String, Box<dyn Error>> {
    let file_size = format_file_size_as_string(std::fs::metadata(flac_file_path.clone())?.len());

    let file_steam = match StreamReader::<File>::from_file(flac_file_path.as_str()) {
        Ok(stream_reader) => stream_reader,
        Err(_) => return Err(Box::new(LocalError::InvalidFlacFile(flac_file_path))),
    };
    let stream_info = file_steam.info();

    let vorbis_comment: VorbisComment = (get_vorbis_comment(flac_file_path.as_str())).unwrap_or_else(|_| VorbisComment {
        vendor_string: Default::default(),
        comments: Default::default(),
    });

    let wave_output_values: Value = upon::value! {
        file_name: get_file_name_from_file_path(&flac_file_path)?,
        file_path: canonicalize_file_path(&flac_file_path)?,
        file_size: file_size,
        min_block_size: stream_info.min_block_size,
        max_block_size: stream_info.max_block_size,
        min_frame_size: stream_info.min_frame_size,
        max_frame_size: stream_info.max_frame_size,
        sample_rate: stream_info.sample_rate as f64 / 1000.0,
        channels: stream_info.channels,
        bits_per_sample: stream_info.bits_per_sample,
        total_samples: stream_info.total_samples,
        md5_sum: format_md5_sum_from_bytes(stream_info.md5_sum),
        vorbis_vendor: vorbis_comment.vendor_string,
        vorbis_comments: vorbis_comment.comments,
    };

    let formated_output = template.get_wave_chunk_output(TEMPLATE_NAME, TEMPLATE_PATH, wave_output_values)?;
    Ok(formated_output)
}

fn format_md5_sum_from_bytes(bytes: [u8; 16]) -> String {
    bytes
        .iter()
        .fold(String::new(), |acc, byte| acc + format!("{:02x}", byte).as_str())
}
