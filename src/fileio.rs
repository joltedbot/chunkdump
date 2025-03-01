use crate::cli::{Args, EXIT_CODE_ERROR};
use crate::errors::{handle_local_error, LocalError};
use crate::wave::CHUNK_SIZE_FIELD_LENGTH_IN_BYTES;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;
use std::process::exit;

pub const FILE_CHUNKID_LENGTH_IN_BYTES: usize = 4;

const AIFF_FILE_CHUNKID: &str = "FORM";
const FLAC_FILE_CHUNKID: &str = "fLaC";
const WAVE_FILE_CHUNKID: &str = "RIFF";

#[derive(Debug, PartialEq)]
pub enum FileType {
    AIFF,
    FLAC,
    WAVE,
    UNSUPPORTED,
}

#[derive(Debug, PartialEq)]
pub enum Endian {
    Little,
    Big,
}

pub fn get_file_id_from_file_or_exit(cli_args: &Args) -> FileType {
    let mut input_file = File::open(&cli_args.input_file_path).unwrap_or_else(|e| {
        handle_local_error(LocalError::InvalidPath(cli_args.input_file_path.clone()), e.to_string());
        exit(EXIT_CODE_ERROR);
    });

    let file_chunk_id = match read_bytes_from_file_as_string(&mut input_file, FILE_CHUNKID_LENGTH_IN_BYTES) {
        Ok(chunk_id) => chunk_id,
        Err(e) => {
            handle_local_error(
                LocalError::CouldNotReadFile(cli_args.input_file_path.clone()),
                e.to_string(),
            );
            exit(EXIT_CODE_ERROR);
        }
    };

    match file_chunk_id.as_str() {
        AIFF_FILE_CHUNKID => FileType::AIFF,
        FLAC_FILE_CHUNKID => FileType::FLAC,
        WAVE_FILE_CHUNKID => FileType::WAVE,
        _ => FileType::UNSUPPORTED,
    }
}

pub fn canonicalize_file_path(file_path: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new(file_path).canonicalize()?;

    let canonical_path = match path.to_str() {
        Some(path) => path.to_string(),
        None => return Err(Box::new(LocalError::InvalidPath(file_path.to_string()))),
    };

    Ok(canonical_path)
}

pub fn get_file_name_from_file_path(file_path: &str) -> Result<String, LocalError> {
    let path = Path::new(file_path);

    let file_name = match path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => return Err(LocalError::InvalidFileName),
    };

    Ok(file_name)
}

pub fn add_one_if_byte_size_is_odd(mut byte_size: u32) -> u32 {
    if byte_size % 2 > 0 {
        byte_size += 1;
    }

    byte_size
}

pub fn read_chunk_size_from_file(file: &mut File, endianness: Endian) -> Result<usize, Box<dyn Error>> {
    let chunk_size_bytes = read_bytes_from_file(file, 4)?;
    let mut byte_array: [u8; CHUNK_SIZE_FIELD_LENGTH_IN_BYTES] = Default::default();
    byte_array.copy_from_slice(chunk_size_bytes.as_slice());

    let mut chunk_size = match endianness {
        Endian::Little => u32::from_le_bytes(byte_array),
        Endian::Big => u32::from_be_bytes(byte_array),
    };

    chunk_size = add_one_if_byte_size_is_odd(chunk_size);

    Ok(chunk_size as usize)
}

pub fn skip_over_bytes_in_file(file: &mut File, number_of_bytes: usize) -> Result<(), Box<dyn Error>> {
    file.seek_relative(number_of_bytes as i64)?;

    Ok(())
}

pub fn read_bytes_from_file(file: &mut File, number_of_bytes: usize) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut read_bytes: Vec<u8> = vec![0; number_of_bytes];
    file.read_exact(&mut read_bytes)?;

    Ok(read_bytes)
}

pub fn read_bytes_from_file_as_string(file: &mut File, number_of_bytes: usize) -> Result<String, Box<dyn Error>> {
    let read_bytes = read_bytes_from_file(file, number_of_bytes)?;

    Ok(String::from_utf8(read_bytes)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn return_correct_canonicalize_path_when_given_path_is_valid() {
        let correct_result = env::current_dir().unwrap().to_str().unwrap().to_string() + "/src/main.rs";
        let result = canonicalize_file_path("./src/main.rs").unwrap();

        assert_eq!(result, correct_result);
    }

    #[test]
    fn canonicalize_file_path_throws_error_when_given_path_is_invalid() {
        let invalid_test_path = "/not/a/real/path".to_string();
        let result = canonicalize_file_path(&invalid_test_path);

        assert!(result.is_err());
    }

    #[test]
    fn get_file_name_from_file_path_returns_correct_result() {
        let result = get_file_name_from_file_path("/test/path/filename.wav").unwrap();
        assert_eq!(result, "filename.wav")
    }

    #[test]
    fn errors_when_geting_filename_from_filepath_if_path_is_invalid() {
        let result = get_file_name_from_file_path("/");
        assert_eq!(result.unwrap_err(), LocalError::InvalidFileName);
    }

    #[test]
    fn correctly_adds_one_if_byte_size_is_odd() {
        let test_size = 3;
        let correct_size = test_size + 1;

        assert_eq!(add_one_if_byte_size_is_odd(test_size), correct_size);
    }

    #[test]
    fn does_not_add_one_if_byte_size_is_even() {
        let test_size = 4;
        assert_eq!(add_one_if_byte_size_is_odd(test_size), test_size);
    }
}
