mod byteio;
mod errors;
mod fileio;
mod template;
mod wave;

use crate::errors::LocalError;
use crate::fileio::read_bytes_from_file_as_string;
use crate::template::Template;
use crate::wave::Wave;
use std::env::Args;
use std::fs::File;
use std::process::exit;

const FILE_CHUNKID_LENGTH_IN_BYTES: usize = 4;
const WAVE_FILE_CHUNKID: &str = "RIFF";
const HELP_FLAG: &str = "-h";
const EXIT_CODE_ERROR: i32 = 1;
const EXIT_CODE_SUCCESS: i32 = 0;

fn main() {
    let path_of_file_to_read: String = process_cli_arguments();
    let mut file = open_file(&path_of_file_to_read);
    let file_chunk_id = get_file_chunk_id(&path_of_file_to_read, &mut file);

    validate_file_chunk_id(file_chunk_id, path_of_file_to_read.clone());

    let wave_file = match Wave::new(path_of_file_to_read.clone(), file) {
        Ok(file) => file,
        Err(e) => {
            println!(
                "\n{}: {}",
                LocalError::CouldNotReadFile(path_of_file_to_read.clone()),
                e
            );
            exit(EXIT_CODE_ERROR);
        }
    };

    let template = match Template::new() {
        Ok(template) => template,
        Err(e) => {
            println!("\n{}: {}", LocalError::CouldNotCreateTemplateStore, e);
            exit(EXIT_CODE_ERROR);
        }
    };

    match wave_file.display_wave_file_metadata(template) {
        Ok(_) => {}
        Err(e) => {
            println!(
                "\n{}: {}",
                LocalError::CouldNotExtractMetaData(path_of_file_to_read.clone()),
                e
            );
            exit(EXIT_CODE_ERROR);
        }
    }
}

fn validate_file_chunk_id(file_chunk_id: String, path_of_file_to_read: String) {
    if file_chunk_id != WAVE_FILE_CHUNKID {
        println!("\n{}", LocalError::InvalidWaveFile(path_of_file_to_read.clone()),);
        exit(EXIT_CODE_ERROR);
    }
}

fn open_file(path_of_file_to_read: &String) -> File {
    match File::open(path_of_file_to_read) {
        Ok(file) => file,
        Err(e) => {
            println!("\n{}: {}\n  {}", LocalError::InvalidPath, path_of_file_to_read, e);
            print_usage_message();
            exit(EXIT_CODE_ERROR);
        }
    }
}

fn get_file_chunk_id(path_of_file_to_read: &str, file: &mut File) -> String {
    match read_bytes_from_file_as_string(file, FILE_CHUNKID_LENGTH_IN_BYTES) {
        Ok(chunk_id) => chunk_id,
        Err(e) => {
            println!(
                "\n{}: {}",
                LocalError::CouldNotReadFile(path_of_file_to_read.to_string()),
                e
            );
            print_usage_message();
            exit(EXIT_CODE_ERROR);
        }
    }
}

fn process_cli_arguments() -> String {
    let mut argv: Args = std::env::args();

    if argv.len() < 2 {
        print_usage_message();
        exit(EXIT_CODE_ERROR);
    }

    let argument: String = match argv.nth(1) {
        None => {
            print_usage_message();
            exit(EXIT_CODE_ERROR);
        }
        Some(argument) => argument,
    };

    if argument == HELP_FLAG {
        print_usage_message();
        exit(EXIT_CODE_SUCCESS);
    }

    argument
}

fn print_usage_message() {
    println!("\nusage: chunkdump [-h] file");
}
