use crate::errors::LocalError;
use std::error::Error;
use std::fs::File;
use std::io::{stdout, Write};
use std::path::Path;

pub fn write_out_file_data(file_data: Vec<String>, output_file_path: &str) -> Result<(), Box<dyn Error>> {
    if !output_file_path.is_empty() {
        write_to_file(file_data, output_file_path)?;
    } else {
        write_to_stdout(file_data)?;
    }

    Ok(())
}

fn write_to_stdout(file_data: Vec<String>) -> Result<(), Box<dyn Error>> {
    for line in file_data {
        let mut lock = stdout().lock();
        writeln!(lock, "{}", line).unwrap()
    }

    Ok(())
}

fn write_to_file(file_data: Vec<String>, output_file_path: &str) -> Result<(), Box<dyn Error>> {
    check_if_file_already_exists(output_file_path)?;

    let mut output_file = File::create(output_file_path)?;
    for data in file_data {
        let line = data + "\n";
        output_file.write_all(line.as_bytes())?;
    }

    Ok(())
}

fn check_if_file_already_exists(output_file: &str) -> Result<(), Box<dyn Error>> {
    if Path::new(output_file).exists() {
        return Err(Box::new(LocalError::OutputFileAlreadyExists(output_file.to_string())));
    }

    Ok(())
}
