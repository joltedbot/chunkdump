use crate::errors::LocalError;
use crate::formating::add_one_if_byte_size_is_odd;
use crate::wave::CHUNK_SIZE_FIELD_LENGTH_IN_BYTES;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;

pub const FILE_CHUNKID_LENGTH_IN_BYTES: usize = 4;

pub fn read_chunk_size_from_file(file: &mut File) -> Result<usize, Box<dyn Error>> {
    let chunk_size_bytes = read_bytes_from_file(file, 4)?;
    let mut byte_array: [u8; CHUNK_SIZE_FIELD_LENGTH_IN_BYTES] = Default::default();
    byte_array.copy_from_slice(chunk_size_bytes.as_slice());

    let mut chunk_size = u32::from_le_bytes(byte_array);
    chunk_size = add_one_if_byte_size_is_odd(chunk_size);

    Ok(chunk_size as usize)
}

pub fn canonicalize_file_path(file_path: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new(file_path).canonicalize()?;

    let canonical_path = match path.to_str() {
        Some(path) => path.to_string(),
        None => return Err(Box::new(LocalError::InvalidPath(file_path.to_string()))),
    };

    Ok(canonical_path)
}

pub fn get_file_name_from_file_path(file_path: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new(file_path);

    let file_name = match path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => return Err(Box::new(LocalError::InvalidFileName)),
    };

    Ok(file_name)
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
    fn throws_error_when_given_path_is_invalid() {
        let invalid_test_path = "/not/a/real/path".to_string();
        let result = canonicalize_file_path(&invalid_test_path);

        assert!(result.is_err());
    }

    #[test]
    fn get_file_name_from_file_path_returns_correct_result() {
        let result = get_file_name_from_file_path("/test/path/filename").unwrap();
        assert_eq!(result, "filename")
    }
}
