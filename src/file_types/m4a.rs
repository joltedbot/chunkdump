use crate::fileio::get_file_metadata;
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use mp4ameta::Tag;
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/file_types/m4a.tmpl");
const HEADER_TEMPLATE_CONTENT: &str = include_str!("../templates/file_types/m4a_header.tmpl");
const USERDATA_TEMPLATE_CONTENT: &str = include_str!("../templates/file_types/m4a_userdata.tmpl");

#[derive(Debug, Serialize)]
struct UserdataTag {
    key: String,
    value: String,
}

pub fn get_metadata_from_file(
    m4a_file_path: &str,
    mandatory_sections_only: bool,
) -> Result<Vec<OutputEntry>, Box<dyn Error>> {
    let m4a_file = File::open(m4a_file_path)?;
    let file_metadata = get_file_metadata(m4a_file_path, &m4a_file, TEMPLATE_CONTENT)?;

    let mut m4a_tags = Tag::read_from_path(m4a_file_path).unwrap();
    let header_metadata = get_mandatory_m4a_chunk_metdata(&mut m4a_tags)?;

    let mut output = vec![file_metadata, header_metadata];

    if !mandatory_sections_only {
        let userdata_tags = get_m4a_userdata_tags(&mut m4a_tags)?;
        output.push(userdata_tags);
    }

    Ok(output)
}

fn get_mandatory_m4a_chunk_metdata(m4a_tags: &mut Tag) -> Result<OutputEntry, Box<dyn Error>> {
    let mut channel_count = 0;
    if let Some(channels) = m4a_tags.channel_config() {
        channel_count = channels.channel_count();
    }

    let avg_bitrate = m4a_tags.avg_bitrate().unwrap_or_default() / 1000;
    let max_bitrate = m4a_tags.max_bitrate().unwrap_or_default() / 1000;

    let raw_duration = m4a_tags.duration().as_secs();
    let duration = format!("{}:{:#02?}", raw_duration / 60, raw_duration % 60);

    let mut sample_rate = String::new();
    if let Some(rate) = m4a_tags.sample_rate() {
        sample_rate = format_sample_rate(rate.hz());
    }

    let output_values: Value = upon::value! {
        channel_count: channel_count,
        avg_bitrate: avg_bitrate,
        max_bitrate: max_bitrate,
        duration: duration,
        sample_rate: sample_rate,
    };

    let formated_output = get_file_chunk_output(HEADER_TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Mandatory,
        text: formated_output,
    })
}

fn get_m4a_userdata_tags(m4a_tags: &mut Tag) -> Result<OutputEntry, Box<dyn Error>> {
    let mut userdata_tags: Vec<UserdataTag> = vec![];
    let mut itunes_tags: Vec<UserdataTag> = vec![];

    m4a_tags.userdata.strings().for_each(|item| {
        let mut key = item.0.to_string();
        let value = item.1.trim().to_string();

        if item.0.to_string().starts_with("----") {
            let components: Vec<&str> = key.split(':').collect();
            key = components[2].to_string();
            itunes_tags.push(UserdataTag { key, value });
        } else {
            userdata_tags.push(UserdataTag { key, value });
        }
    });

    let output_values: Value = upon::value! {
        userdata_tags: userdata_tags,
        itunes_tags: itunes_tags,
    };

    let formated_output = get_file_chunk_output(USERDATA_TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}

fn format_sample_rate(sample_rate: u32) -> String {
    let sample_rate_in_khz = f64::from(sample_rate) / 1000.0;

    if sample_rate_in_khz == sample_rate_in_khz.floor() {
        format!("{:#.0}", sample_rate_in_khz)
    } else {
        format!("{:#.1}", sample_rate_in_khz)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_single_digit_decimal_khz_when_sample_rate_is_44100hz() {
        let test_sample_rate = 44100;
        let correct_result = "44.1";
        let result = format_sample_rate(test_sample_rate);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_whole_number_khz_when_sample_rate_is_48000hz() {
        let test_sample_rate = 48000;
        let correct_result = "48";
        let result = format_sample_rate(test_sample_rate);
        assert_eq!(result, correct_result);
    }
}
