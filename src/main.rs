mod byteio;
mod chunk;
mod errors;
mod fileio;
mod midi;
mod output;
mod template;
mod wave;

const USAGE_MESSAGE: &str = " usage: chunkdump [-hv] file";

use crate::errors::LocalError;
use crate::fileio::{get_file_chunk_id, open_file};
use crate::template::Template;
use crate::wave::Wave;
use argh::FromArgs;
use std::process::exit;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const FILE_CHUNKID_LENGTH_IN_BYTES: usize = 4;
const WAVE_FILE_CHUNKID: &str = "RIFF";
const EXIT_CODE_ERROR: i32 = 1;
const EXIT_CODE_SUCCESS: i32 = 0;

#[derive(FromArgs)]
/// Chunkdump - Extract Metadata From Wave Files
#[argh(help_triggers("-h", "--help", "help"))]
struct CLIArguments {
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
    let args: CLIArguments = argh::from_env();

    if args.version {
        println!("{}", VERSION);
        exit(EXIT_CODE_SUCCESS);
    }

    if args.wave_file.is_none() {
        print_usage_message();
        exit(EXIT_CODE_ERROR);
    }

    let path_to_ouptut_file: String = args.output_file.unwrap_or_else(|| String::new());

    let path_of_file_to_read = args.wave_file.unwrap();

    let mut file = open_file(&path_of_file_to_read);
    let file_chunk_id = get_file_chunk_id(&path_of_file_to_read, &mut file);

    validate_file_chunk_id(file_chunk_id, path_of_file_to_read.clone());

    let wave_file = match Wave::new(path_of_file_to_read.clone(), file, path_to_ouptut_file) {
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

    let template = Template::new();

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

fn print_usage_message() {
    println!("\n{}\n", USAGE_MESSAGE);
}
