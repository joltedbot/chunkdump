use argh::FromArgs;
use std::process::exit;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const EXIT_CODE_ERROR: i32 = 1;
pub const EXIT_CODE_SUCCESS: i32 = 0;
pub const USAGE_MESSAGE: &str = " usage: chunkdump [-hv] file";

#[derive(PartialEq, Debug)]
pub struct Args {
    pub input_file_path: String,
    pub output_file_path: Option<String>,
}

#[derive(FromArgs)]
/// Chunkdump - Extract Metadata From RIFF Wave, AIFF, Flac, SMF MIDI, and RIFF RMID Files
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

pub fn process_cli_arguments(args: CliArguments) -> Args {
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
        output_file_path: args.output_file,
    }
}

pub fn print_usage_message() {
    println!("\n{}\n", USAGE_MESSAGE);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_correctly_formated_args_from_valid_cli_input_and_output_path() {
        let test_input_path = String::from("/tmp/test.wav");
        let test_output_path = String::from("/tmp/test.txt");
        let test_args = CliArguments {
            version: false,
            file: Some(test_input_path.clone()),
            output_file: Some(test_output_path.clone()),
        };
        let correct_result = Args {
            input_file_path: test_input_path,
            output_file_path: Some(test_output_path),
        };

        let result = process_cli_arguments(test_args);

        assert_eq!(result, correct_result);
    }

    #[test]
    fn returns_correctly_formated_args_from_valid_cli_input_but_no_output_path() {
        let test_input_path = String::from("/tmp/test.wav");
        let test_args = CliArguments {
            version: false,
            file: Some(test_input_path.clone()),
            output_file: None,
        };
        let correct_result = Args {
            input_file_path: test_input_path,
            output_file_path: None,
        };

        let result = process_cli_arguments(test_args);

        assert_eq!(result, correct_result);
    }
}
