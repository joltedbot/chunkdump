use argh::FromArgs;
use std::process::exit;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const EXIT_CODE_ERROR: i32 = 1;
pub const EXIT_CODE_SUCCESS: i32 = 0;
pub const USAGE_MESSAGE: &str = " usage: chunkdump [-hv] file";

pub struct Args {
    pub input_file_path: String,
    pub output_file_path: String,
}

#[derive(FromArgs)]
/// Chunkdump - Extract Metadata From Wave, AIFF, and Flac Files
#[argh(help_triggers("-h", "--help", "help"))]
pub struct CliArguments {
    /// print the version
    #[argh(switch, short = 'v')]
    version: bool,

    /// a file path to output the data to rather than stdout
    #[argh(option, short = 'o')]
    output_file: Option<String>,

    #[argh(positional)]
    file: Option<String>,
}

pub fn process_cli_arguments() -> Args {
    let args: CliArguments = argh::from_env();

    if args.version {
        println!("{}", VERSION);
        exit(EXIT_CODE_SUCCESS);
    }

    if args.file.is_none() {
        print_usage_message();
        exit(EXIT_CODE_ERROR);
    }

    Args {
        input_file_path: args.file.unwrap(),
        output_file_path: args.output_file.unwrap_or_default(),
    }
}

pub fn print_usage_message() {
    println!("\n{}\n", USAGE_MESSAGE);
}
