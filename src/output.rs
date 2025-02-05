use crate::errors::LocalError;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn write_out_file_data(file_data: Vec<String>, output_file: String) -> Result<(), Box<dyn Error>> {
    if output_file.is_empty() {
        for line in file_data {
            println!("{}", line);
        }
    } else {
        if Path::new(&output_file).exists() {
            return Err(Box::new(LocalError::OutputFileAlreadyExists(output_file)));
            c
        }

        let mut file = File::create(output_file)?;
        for line in file_data {
            file.write_all(line.as_bytes())?;
        }
    }

    Ok(())
}
