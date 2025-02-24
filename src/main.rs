mod aiff;
mod byteio;
mod cli;
mod errors;
mod fileio;
mod flac;
mod formating;
mod shared;
mod template;
mod wave;

use crate::aiff::extract_and_output_aiff_metadata;
use crate::cli::process_cli_arguments;
use crate::cli::{print_usage_message, EXIT_CODE_ERROR};
use crate::errors::handle_local_error;
use crate::errors::LocalError;
use crate::fileio::{read_bytes_from_file_as_string, FILE_CHUNKID_LENGTH_IN_BYTES};
use crate::flac::extract_and_output_flac_metadata;
use crate::wave::extract_and_output_wave_metadata;
use std::fs::File;
use std::process::exit;

const WAVE_FILE_CHUNKID: &str = "RIFF";
const FLAC_FILE_CHUNKID: &str = "fLaC";
const AIFF_FILE_CHUNKID: &str = "FORM";

fn main() {
    let cli_args = process_cli_arguments();

    let mut input_file = File::open(&cli_args.input_file_path).unwrap_or_else(|e| {
        handle_local_error(LocalError::InvalidPath(cli_args.input_file_path.clone()), e.to_string());
        exit(EXIT_CODE_ERROR);
    });

    let file_chunk_id = match read_bytes_from_file_as_string(&mut input_file, FILE_CHUNKID_LENGTH_IN_BYTES) {
        Ok(chunk_id) => chunk_id,
        Err(e) => {
            handle_local_error(LocalError::CouldNotReadFile(cli_args.input_file_path), e.to_string());
            exit(EXIT_CODE_ERROR);
        }
    };

    match file_chunk_id.as_str() {
        WAVE_FILE_CHUNKID => {
            extract_and_output_wave_metadata(&cli_args.input_file_path, &cli_args.output_file_path).unwrap_or_else(
                |error| {
                    handle_local_error(
                        LocalError::CouldNotReadData(cli_args.input_file_path),
                        error.to_string(),
                    );
                    exit(EXIT_CODE_ERROR);
                },
            );
        }
        FLAC_FILE_CHUNKID => {
            extract_and_output_flac_metadata(&cli_args.input_file_path, &cli_args.output_file_path).unwrap_or_else(
                |error| {
                    handle_local_error(
                        LocalError::CouldNotReadData(cli_args.input_file_path),
                        error.to_string(),
                    );
                    exit(EXIT_CODE_ERROR);
                },
            );
        }
        AIFF_FILE_CHUNKID => {
            extract_and_output_aiff_metadata(&cli_args.input_file_path, &cli_args.output_file_path).unwrap_or_else(
                |error| {
                    handle_local_error(
                        LocalError::CouldNotReadData(cli_args.input_file_path),
                        error.to_string(),
                    );
                    exit(EXIT_CODE_ERROR);
                },
            );
        }
        _ => {
            handle_local_error(LocalError::UnsupportedFileType, "".to_string());
            print_usage_message();
            exit(EXIT_CODE_ERROR);
        }
    }
}
