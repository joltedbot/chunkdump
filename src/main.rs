mod byteio;
mod errors;
mod fileio;
mod wave;

use crate::fileio::read_bytes_from_file_as_string;
use crate::wave::Wave;
use std::env::Args;
use std::fs::File;
use std::process::exit;

const FILE_CHUNKID_LENGTH_IN_BYTES: usize = 4;
const WAVE_FILE_CHUNKID: &str = "RIFF";

fn main() {
    let mut argv: Args = std::env::args();

    let path_of_file_to_read: String = {
        if argv.len() > 1 {
            argv.nth(1).expect("Failed to read the 1st parameter")
        } else {
            println!("Please provide a path to a file to read.");
            exit(1);
        }
    };

    let mut file = File::open(&path_of_file_to_read).expect("Could not open file path");

    let file_chunk_id = read_bytes_from_file_as_string(&mut file, FILE_CHUNKID_LENGTH_IN_BYTES)
        .expect("Could not extract file chunk id");
    if file_chunk_id.as_str() == WAVE_FILE_CHUNKID {
        let wave_file =
            Wave::new(path_of_file_to_read, file).expect("Could not create new wave file object");

        println!("{:#?}", wave_file);
    }
}
