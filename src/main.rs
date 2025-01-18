mod bytes;
mod errors;
mod wave;

use crate::bytes::read_bytes_from_file;
use crate::wave::Wave;
use std::error::Error;
use std::fs::File;

const FILE_CHUNKID_LENGTH_IN_BYTES: usize = 4;
const WAVE_FILE_CHUNKID: &str = "RIFF";

fn main() {
    let path_of_file_to_read = "./test/a440-ext.wav".to_string();
    let mut file = File::open(&path_of_file_to_read).expect("Could not open file path");

    let file_chunk_id = get_file_chunk_id(&mut file).expect("Could not extract file chunk id");
    if file_chunk_id.as_str() == WAVE_FILE_CHUNKID {
        let wave_file =
            Wave::new(path_of_file_to_read, file).expect("Could not crate new wave file object");

        println!("{:#?}", wave_file);
    }
}

pub fn get_file_chunk_id(file: &mut File) -> Result<String, Box<dyn Error>> {
    let file_chunk_id = read_bytes_from_file(file, FILE_CHUNKID_LENGTH_IN_BYTES)?;
    Ok(String::from_utf8(file_chunk_id)?)
}
