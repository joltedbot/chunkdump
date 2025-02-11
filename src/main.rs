mod byteio;
mod chunk;
mod errors;
mod fileio;
mod flac;
mod formating;
mod midi;
mod output;
mod template;
mod wave;

use crate::errors::LocalError;
use crate::fileio::{open_file, read_chunk_id_from_file};
use crate::flac::output_flac_metadata;
use crate::template::Template;
use crate::wave::Wave;
use argh::FromArgs;
use std::process::exit;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const USAGE_MESSAGE: &str = " usage: chunkdump [-hv] file";
const FILE_CHUNKID_LENGTH_IN_BYTES: usize = 4;
const WAVE_FILE_CHUNKID: &str = "RIFF";
const FLAC_FILE_CHUNKID: &str = "fLaC";
const EXIT_CODE_ERROR: i32 = 1;
const EXIT_CODE_SUCCESS: i32 = 0;

#[derive(FromArgs)]
/// Chunkdump - Extract Metadata From Wave Files
#[argh(help_triggers("-h", "--help", "help"))]
struct CliArguments {
    /// print the version
    #[argh(switch, short = 'v')]
    version: bool,

    /// a file path to output the data to rather than stdout
    #[argh(option, short = 'o')]
    output_file: Option<String>,

    #[argh(positional)]
    wave_file: Option<String>,
}

fn main() {
    let args: CliArguments = argh::from_env();

    process_cli_switches(&args);

    let input_file_path = args.wave_file.unwrap();
    let ouptut_file_path: String = args.output_file.unwrap_or_else(|| String::new());

    let mut input_file = open_file(&input_file_path);
    let file_chunk_id = read_chunk_id_from_file(&input_file_path, &mut input_file);

    match file_chunk_id.as_str() {
        WAVE_FILE_CHUNKID => {
            let wave_file = Wave::new(input_file_path.clone(), ouptut_file_path, &mut input_file).unwrap_or_else(|error| {
                println!("\n{}: {}", LocalError::CouldNotReadFile(input_file_path.clone()), error);
                exit(EXIT_CODE_ERROR);
            });

            wave_file.output_metadata(Template::new()).unwrap_or_else(|error| {
                println!("\n{}: {}", LocalError::CouldNotReadData(input_file_path.clone()), error);
                exit(EXIT_CODE_ERROR);
            });
        }
        FLAC_FILE_CHUNKID => {
            output_flac_metadata(Template::new(), input_file_path.clone(), ouptut_file_path).unwrap_or_else(|error| {
                println!("\n{}: {}", LocalError::CouldNotReadData(input_file_path.clone()), error);
                exit(EXIT_CODE_ERROR);
            });
        }
        _ => {
            println!("{}", LocalError::UnsupportedFileType(input_file_path));
            print_usage_message();
            exit(EXIT_CODE_ERROR);
        }
    }
}

fn process_cli_switches(args: &CliArguments) {
    if args.version {
        println!("{}", VERSION);
        exit(EXIT_CODE_SUCCESS);
    }

    if args.wave_file.is_none() {
        print_usage_message();
        exit(EXIT_CODE_ERROR);
    }
}

fn print_usage_message() {
    println!("\n{}\n", USAGE_MESSAGE);
}
