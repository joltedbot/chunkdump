use std::fs::File;
use std::io::Write;

pub fn write_out_file_data(file_data: Vec<String>, output_file: String) -> Result<(), std::io::Error> {
    if output_file.is_empty() {
        for line in file_data {
            println!("{}", line);
        }
    } else {
        let mut file = File::create(output_file)?;
        for line in file_data {
            file.write_all(line.as_bytes())?;
        }
    }

    Ok(())
}
