use crate::byte_arrays::{
    take_first_four_bytes_as_unsigned_integer, take_first_two_bytes_as_signed_integer, Endian,
};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use serde::Serialize;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/caf_chunks/ovvw.tmpl");
const OVERVIEW_SAMPLE_TOTAL_BYTES: usize = 32;
const DEFAULT_SPACER_LENGTH: usize = 5;

#[derive(Default, Debug, Serialize)]
struct OverviewSample {
    sample_index: String,
    spacer1: String,
    min_value: String,
    spacer2: String,
    max_value: String,
    spacer3: String,
}

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let edit_count = take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
    let frames_per_ovw_sample =
        take_first_four_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;

    let mut samples: Vec<OverviewSample> = Vec::new();
    let mut sample_index: u32 = 0;
    chunk_data
        .chunks_exact(OVERVIEW_SAMPLE_TOTAL_BYTES)
        .for_each(|sample| {
            sample_index += 1;
            let min =
                take_first_two_bytes_as_signed_integer(&mut sample[0..2].to_vec(), Endian::Big)
                    .unwrap_or_default();
            let max =
                take_first_two_bytes_as_signed_integer(&mut sample[2..4].to_vec(), Endian::Big)
                    .unwrap_or_default();

            samples.push(OverviewSample {
                sample_index: format!("{}:", sample_index),
                spacer1: "   ".to_string(),
                min_value: format!("{}", min),
                spacer2: "   ".to_string(),
                max_value: format!("{}", max),
                spacer3: "   ".to_string(),
            })
        });

    set_overview_samples_spacers(&mut samples);

    let output_values: Value = upon::value! {
        edit_count: edit_count,
        frames_per_ovw_sample: frames_per_ovw_sample,
        samples: samples,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}

fn set_overview_samples_spacers(overview_samples: &mut Vec<OverviewSample>) {
    let longest_index = match overview_samples
        .iter()
        .max_by_key(|tag| tag.sample_index.len())
    {
        Some(tag) => tag.sample_index.len(),
        None => DEFAULT_SPACER_LENGTH,
    };

    let longest_min_value = match overview_samples
        .iter()
        .max_by_key(|tag| tag.min_value.len())
    {
        Some(tag) => tag.min_value.len(),
        None => DEFAULT_SPACER_LENGTH,
    };

    let longest_max_value = match overview_samples
        .iter()
        .max_by_key(|tag| tag.max_value.len())
    {
        Some(tag) => tag.max_value.len(),
        None => DEFAULT_SPACER_LENGTH,
    };

    for kv in overview_samples {
        if longest_index > kv.sample_index.len() {
            kv.spacer1 = " ".repeat(longest_index - kv.sample_index.len());
        } else {
            kv.spacer1 = String::new();
        }

        if longest_min_value > kv.min_value.len() {
            kv.spacer2 = " ".repeat(longest_min_value - kv.min_value.len());
        } else {
            kv.spacer2 = String::new();
        }

        if longest_max_value > kv.max_value.len() {
            kv.spacer3 = " ".repeat(longest_max_value - kv.max_value.len());
        } else {
            kv.spacer3 = String::new();
        }
    }
}
