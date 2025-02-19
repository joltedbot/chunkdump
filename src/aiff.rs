use crate::fileio::{read_aiff_chunk_size_from_file, read_bytes_from_file_as_string};
use crate::output::write_out_file_data;
use crate::template::Template;
use std::error::Error;
use std::fs::File;
//const TEMPLATE_NAME: &str = "aiff";
//const TEMPLATE_CONTENT: &str = include_str!("templates/aiff/aiff.tmpl");

pub fn output_aiff_metadata(aiff_file_path: &str, output_file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut template = Template::new();
    let output_lines: Vec<String> = vec![format_data_for_output(&mut template, aiff_file_path)?];
    write_out_file_data(output_lines, output_file_path)?;

    Ok(())
}

fn format_data_for_output(template: &mut Template, aiff_file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut aiff_file = File::open(aiff_file_path)?;

    let chunk_id = read_bytes_from_file_as_string(&mut aiff_file, 4)?;
    let chunk_size = read_aiff_chunk_size_from_file(&mut aiff_file)?;
    let form_type = read_bytes_from_file_as_string(&mut aiff_file, 4)?;

    Ok(format!("{} - {} - {}", chunk_id, chunk_size, form_type))
}
