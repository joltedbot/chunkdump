mod aiff;
mod bytes;
mod cli;
mod errors;
mod fileio;
mod flac;
mod formating;
mod shared;
mod template;
mod wave;

use crate::cli::process_cli_arguments;
use crate::cli::{print_usage_message, EXIT_CODE_ERROR};
use crate::errors::handle_local_error;
use crate::errors::LocalError;
use crate::fileio::{get_file_id_from_file_or_exit, write_out_file_data, FileType};
use std::process::exit;

fn main() {
    let cli_args = process_cli_arguments();

    let file_type = get_file_id_from_file_or_exit(&cli_args);

    let metadata = match file_type {
        FileType::WAVE => wave::get_metadata_from_file(&cli_args.input_file_path).unwrap_or_else(|error| {
            handle_local_error(
                LocalError::CouldNotReadData(cli_args.input_file_path),
                error.to_string(),
            );
            exit(EXIT_CODE_ERROR);
        }),
        FileType::FLAC => flac::get_metadata_from_file(&cli_args.input_file_path).unwrap_or_else(|error| {
            handle_local_error(
                LocalError::CouldNotReadData(cli_args.input_file_path),
                error.to_string(),
            );
            exit(EXIT_CODE_ERROR);
        }),
        FileType::AIFF => aiff::get_metadata_from_file(&cli_args.input_file_path).unwrap_or_else(|error| {
            handle_local_error(
                LocalError::CouldNotReadData(cli_args.input_file_path),
                error.to_string(),
            );
            exit(EXIT_CODE_ERROR);
        }),
        _ => {
            handle_local_error(LocalError::UnsupportedFileType, "".to_string());
            print_usage_message();
            exit(EXIT_CODE_ERROR);
        }
    };

    write_out_file_data(metadata, cli_args.output_file_path).unwrap_or_else(|error| {
        handle_local_error(LocalError::CouldNotWrteOutData, error.to_string());
        exit(EXIT_CODE_ERROR);
    });
}
