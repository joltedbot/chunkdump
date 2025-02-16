mod byteio;
mod chunk;
mod cli;
mod errors;
mod fileio;
mod flac;
mod formating;
mod midi;
mod output;
mod template;
mod wave;

use crate::cli::process_cli_arguments;
use crate::cli::{print_usage_message, EXIT_CODE_ERROR};
use crate::errors::handle_local_error;
use crate::errors::LocalError;
use crate::fileio::{open_file, read_chunk_id_from_file};
use crate::flac::output_flac_metadata;
use crate::wave::output_wave_metadata;
use std::process::exit;

const WAVE_FILE_CHUNKID: &str = "RIFF";
const FLAC_FILE_CHUNKID: &str = "fLaC";

fn main() {
    let cli_args = process_cli_arguments();

    let mut input_file = open_file(&cli_args.input_file_path);
    let file_chunk_id = read_chunk_id_from_file(&cli_args.input_file_path, &mut input_file);

    match file_chunk_id.as_str() {
        WAVE_FILE_CHUNKID => {
            output_wave_metadata(&cli_args.input_file_path, &cli_args.output_file_path).unwrap_or_else(|error| {
                handle_local_error(
                    LocalError::CouldNotReadData(cli_args.input_file_path),
                    error.to_string(),
                );
                exit(EXIT_CODE_ERROR);
            });
        }
        FLAC_FILE_CHUNKID => {
            output_flac_metadata(&cli_args.input_file_path, &cli_args.output_file_path).unwrap_or_else(|error| {
                handle_local_error(
                    LocalError::CouldNotReadData(cli_args.input_file_path),
                    error.to_string(),
                );
                exit(EXIT_CODE_ERROR);
            });
        }
        _ => {
            handle_local_error(LocalError::UnsupportedFileType(), "".to_string());
            print_usage_message();
            exit(EXIT_CODE_ERROR);
        }
    }
}
