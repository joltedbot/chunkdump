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

#[derive(Default, Debug, Serialize)]
struct OverviewSample {
    sample_index: String,
    min_value: String,
    max_value: String,
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
                sample_index: format!("{}:\t", sample_index),
                min_value: format!("{}\t", min),
                max_value: format!("{}", max),
            })
        });

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
