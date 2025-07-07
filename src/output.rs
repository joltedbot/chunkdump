use crate::errors::LocalError;
use std::error::Error;
use std::fs::File;
use std::io::{stdout, Write};
use std::path::Path;

#[derive(PartialEq, Eq, Debug)]
pub enum Section {
    Header,
    Mandatory,
    Optional,
    Unsupported,
    Skipped,
    Empty,
}

#[derive(PartialEq, Eq, Debug)]
pub struct OutputEntry {
    pub section: Section,
    pub text: String,
}

pub fn output_metadata(
    file_data: Vec<OutputEntry>,
    output_file_path: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let mut header: Vec<String> = vec![];
    let mut mandatory: Vec<String> = vec![];
    let mut optional: Vec<String> = vec![];
    let mut unsupported: Vec<String> = vec![];
    let mut skipped: Vec<String> = vec![];
    let mut empty: Vec<String> = vec![];

    file_data.iter().for_each(|chunk| match chunk.section {
        Section::Header => header.push(chunk.text.clone()),
        Section::Mandatory => mandatory.push(chunk.text.clone()),
        Section::Optional => optional.push(chunk.text.clone()),
        Section::Unsupported => unsupported.push(chunk.text.clone()),
        Section::Skipped => skipped.push(chunk.text.clone()),
        Section::Empty => empty.push(chunk.text.clone()),
    });

    let mut output_metadata: Vec<String> = header;

    if !mandatory.is_empty() {
        output_metadata.push(include_str!("templates/output/mandatory.tmpl").to_string());
        output_metadata.append(&mut mandatory);
    }

    if !optional.is_empty() {
        output_metadata.push(include_str!("templates/output/optional.tmpl").to_string());
        output_metadata.append(&mut optional);
    }

    if !unsupported.is_empty() {
        output_metadata.push(include_str!("templates/output/unsupported.tmpl").to_string());
        output_metadata.append(&mut unsupported);
    }

    if !skipped.is_empty() {
        output_metadata.push(include_str!("templates/output/skipped.tmpl").to_string());
        output_metadata.append(&mut skipped);
    }

    if !empty.is_empty() {
        output_metadata.push(include_str!("templates/output/empty.tmpl").to_string());
        output_metadata.append(&mut empty);
    }

    write_to_output(output_file_path, output_metadata)?;

    Ok(())
}

fn write_to_output(
    output_file_path: Option<String>,
    output_metadata: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    match output_file_path {
        None => write_to_stdout(output_metadata),
        Some(output_path) => write_to_file(output_metadata, output_path),
    }
}

fn write_to_stdout(file_data: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut lock = stdout().lock();
    file_data
        .iter()
        .for_each(|line| writeln!(lock, "{}", line).unwrap());
    Ok(())
}

fn write_to_file(file_data: Vec<String>, output_file_path: String) -> Result<(), Box<dyn Error>> {
    if Path::new(&output_file_path).exists() {
        return Err(Box::new(LocalError::OutputFileAlreadyExists(
            output_file_path,
        )));
    }

    let mut output_file = File::create(output_file_path)?;
    for data in file_data {
        let line = data + "\n";
        output_file.write_all(line.as_bytes())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn fails_to_write_to_a_file_that_already_exists() {
        let mut test_file_path: PathBuf = std::env::temp_dir();
        test_file_path.push("test_writing_to_exisitng_file.txt");
        File::create(test_file_path.clone()).unwrap();
        let result = write_to_file(
            vec!["test".to_string()],
            test_file_path.to_str().unwrap().to_string(),
        );
        std::fs::remove_file(test_file_path).unwrap();
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("test_writing_to_exisitng_file.txt Already Exists"));
    }

    #[test]
    fn correctly_writes_to_file_when_the_file_path_is_valid() {
        let mut test_file_path: PathBuf = std::env::temp_dir();
        test_file_path.push("test_writing_valid_file.txt");
        let result = write_to_file(
            vec!["test1\n".to_string(), "test2\n".to_string()],
            test_file_path.to_str().unwrap().to_string(),
        );
        std::fs::remove_file(test_file_path).unwrap();
        assert!(result.is_ok());
    }
}
