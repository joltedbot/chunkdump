mod blocks;
mod byte_arrays;
mod chunks;
mod cli;
mod errors;
mod file_types;
mod fileio;
mod formating;
mod output;
mod template;

use crate::cli::process_cli_arguments;
use crate::cli::EXIT_CODE_ERROR;
use crate::errors::handle_local_error;
use crate::errors::LocalError;
use crate::fileio::get_file_id_from_file;
use output::write_out_metadata;
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

    let metadata = file_types::get_file_metadata(&cli_args, file_type).unwrap_or_else(|err| {
        handle_local_error(LocalError::CouldNotReadData(cli_args.input_file_path), err.to_string());
        exit(EXIT_CODE_ERROR);
    });

    write_out_metadata(metadata, cli_args.output_file_path).unwrap_or_else(|error| {
        handle_local_error(LocalError::CouldNotWrteOutData, error.to_string());
        exit(EXIT_CODE_ERROR);
    });
}
