mod blocks;
mod byte_arrays;
mod chunks;
mod cli;
mod errors;
mod fileio;
mod files;
mod formating;
mod output;
mod template;

use crate::cli::process_cli_arguments;
use crate::cli::{print_usage_message, EXIT_CODE_ERROR};
use crate::errors::handle_local_error;
use crate::errors::LocalError;
use crate::fileio::{get_file_id_from_file, FileType};
use files::{aiff, flac, rmid, smf, wave};
use output::write_out_metadata;
use output::OutputEntry;
use std::process::exit;

fn main() {
    let cli_args = process_cli_arguments(argh::from_env());

    let file_type = get_file_id_from_file(&cli_args.input_file_path).unwrap_or_else(|err| {
        handle_local_error(
            LocalError::CouldNotReadData(cli_args.input_file_path.clone()),
            err.to_string(),
        );
        exit(EXIT_CODE_ERROR);
    });

    let metadata: Vec<OutputEntry> = match file_type {
        FileType::Wave => wave::get_metadata_from_file(&cli_args.input_file_path).unwrap_or_else(|err| {
            handle_local_error(LocalError::CouldNotReadFile(cli_args.input_file_path), err.to_string());
            exit(EXIT_CODE_ERROR);
        }),

        FileType::Flac => flac::get_metadata_from_file(&cli_args.input_file_path).unwrap_or_else(|err| {
            handle_local_error(LocalError::CouldNotReadData(cli_args.input_file_path), err.to_string());
            exit(EXIT_CODE_ERROR);
        }),

        FileType::Aiff => aiff::get_metadata_from_file(&cli_args.input_file_path).unwrap_or_else(|err| {
            handle_local_error(LocalError::CouldNotReadData(cli_args.input_file_path), err.to_string());
            exit(EXIT_CODE_ERROR);
        }),
        FileType::Smf => smf::get_metadata_from_file(&cli_args.input_file_path).unwrap_or_else(|err| {
            handle_local_error(LocalError::CouldNotReadData(cli_args.input_file_path), err.to_string());
            exit(EXIT_CODE_ERROR);
        }),
        FileType::Rmid => rmid::get_metadata_from_file(&cli_args.input_file_path).unwrap_or_else(|err| {
            handle_local_error(LocalError::CouldNotReadData(cli_args.input_file_path), err.to_string());
            exit(EXIT_CODE_ERROR);
        }),
        _ => {
            handle_local_error(LocalError::UnsupportedFileType, "".to_string());
            print_usage_message();
            exit(EXIT_CODE_ERROR);
        }
    };

    write_out_metadata(metadata, cli_args.output_file_path).unwrap_or_else(|error| {
        handle_local_error(LocalError::CouldNotWrteOutData, error.to_string());
        exit(EXIT_CODE_ERROR);
    });
}
