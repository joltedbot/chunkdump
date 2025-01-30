mod byteio;
mod errors;
mod fileio;
mod wave;

use crate::errors::LocalError;
use crate::fileio::read_bytes_from_file_as_string;
use crate::wave::Wave;
use std::env::Args;
use std::fs::File;
use std::process::exit;

const FILE_CHUNKID_LENGTH_IN_BYTES: usize = 4;
const WAVE_FILE_CHUNKID: &str = "RIFF";
const HELP_FLAG: &str = "-h";

fn main() {
    let path_of_file_to_read: String = process_cli_arguments();

    let mut file = match File::open(&path_of_file_to_read) {
        Ok(file) => file,
        Err(e) => {
            println!(
                "\n{}: {}\n   {}",
                LocalError::InvalidPath,
                path_of_file_to_read,
                e.to_string()
            );
            print_usage_message();
            exit(1);
        }
    };

    let file_chunk_id = read_bytes_from_file_as_string(&mut file, FILE_CHUNKID_LENGTH_IN_BYTES)
        .expect("Could not extract file chunk id");
    if file_chunk_id.as_str() == WAVE_FILE_CHUNKID {
        let wave_file =
            Wave::new(path_of_file_to_read, file).expect("Could not create new wave file object");
        let _ = wave_file.display_wave_file_metadata();
    }
}

fn process_cli_arguments() -> String {
    let mut argv: Args = std::env::args();

    if argv.len() < 2 {
        print_usage_message();
        exit(1);
    }

    let argument: String = match argv.nth(1) {
        None => {
            print_usage_message();
            exit(1);
        }
        Some(argument) => argument,
    };

    if argument == HELP_FLAG {
        print_usage_message();
        exit(0);
    }

    argument
}

fn print_usage_message() {
    println!("\nusage: chunkdump [-h] file");
}
