use crate::blocks::get_metadata_from_blocks;
use crate::fileio::get_file_metadata;
use crate::output::OutputEntry;
use std::error::Error;
use std::fs::File;

const TEMPLATE_CONTENT: &str = include_str!("../templates/file_types/flac.tmpl");

pub fn get_metadata_from_file(file_path: &str) -> Result<Vec<OutputEntry>, Box<dyn Error>> {
    let mut flac_file = File::open(file_path)?;
    let file_metadata = get_file_metadata(file_path, &flac_file, TEMPLATE_CONTENT)?;
    let block_metadata = get_metadata_from_blocks(&mut flac_file)?;

    let mut output = vec![file_metadata];
    output.extend(block_metadata);

    Ok(output)
}
