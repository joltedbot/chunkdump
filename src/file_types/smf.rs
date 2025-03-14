use crate::file_types::midi::get_metadata_from_midi_data;
use crate::fileio::get_file_metadata;
use crate::output::OutputEntry;
use std::error::Error;
use std::fs::File;
use std::io::Read;

const TEMPLATE_CONTENT: &str = include_str!("../templates/file_types/smf.tmpl");

pub fn get_metadata_from_file(smf_file_path: &str) -> Result<Vec<OutputEntry>, Box<dyn Error>> {
    let mut smf_file = File::open(smf_file_path)?;
    let file_metadata = get_file_metadata(smf_file_path, &mut smf_file, TEMPLATE_CONTENT)?;
    let smf_metadata = get_metadata_from_smf(&mut smf_file)?;

    let mut chunks = vec![file_metadata];
    chunks.extend(smf_metadata);

    Ok(chunks)
}

pub fn get_metadata_from_smf(smf_file: &mut File) -> Result<Vec<OutputEntry>, Box<dyn Error>> {
    let mut midi_data = get_midi_data_from_file(smf_file)?;
    let midi_metadata = get_metadata_from_midi_data(&mut midi_data)?;
    Ok(midi_metadata)
}

pub fn get_midi_data_from_file(midi_file: &mut File) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut midi_data: Vec<u8> = Vec::new();
    midi_file.read_to_end(&mut midi_data)?;
    Ok(midi_data)
}
