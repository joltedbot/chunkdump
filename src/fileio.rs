use crate::errors::LocalError;
use byte_unit::rust_decimal::prelude::Zero;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;

pub fn canonicalize_file_path(file_path: &String) -> Result<String, Box<dyn Error>> {
    let path = Path::new(file_path).canonicalize()?;

    let canonical_path = match path.to_str() {
        Some(path) => path.to_string(),
        None => return Err(Box::new(LocalError::InvalidPath)),
    };

    Ok(canonical_path)
}

pub fn get_file_name_from_file_path(file_path: &String) -> Result<String, Box<dyn Error>> {
    let path = Path::new(file_path);

    let file_name = match path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => return Err(Box::new(LocalError::InvalidFileName)),
    };

    Ok(file_name)
}

pub fn read_bytes_from_file(
    file: &mut File,
    number_of_bytes: usize,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut read_bytes: Vec<u8> = vec![0; number_of_bytes];
    file.read_exact(&mut read_bytes)?;

    Ok(read_bytes)
}

pub fn skip_over_bytes_in_file(
    file: &mut File,
    number_of_bytes: i64,
) -> Result<(), Box<dyn Error>> {
    file.seek_relative(number_of_bytes)?;

    Ok(())
}

pub fn read_four_byte_integer_from_file(file: &mut File) -> Result<u32, Box<dyn Error>> {
    let read_bytes = read_bytes_from_file(file, 4)?;
    let mut byte_array: [u8; 4] = Default::default();
    byte_array.copy_from_slice(read_bytes.as_slice());

    Ok(u32::from_le_bytes(byte_array))
}

pub fn read_chunk_size_from_file(file: &mut File) -> Result<u32, Box<dyn Error>> {
    let chunk_size_bytes = read_bytes_from_file(file, 4)?;
    let mut byte_array: [u8; 4] = Default::default();
    byte_array.copy_from_slice(chunk_size_bytes.as_slice());

    let chunk_size = u32::from_le_bytes(byte_array);

    Ok(chunk_size)
}

pub fn read_bytes_from_file_as_string(
    file: &mut File,
    number_of_bytes: usize,
) -> Result<String, Box<dyn Error>> {
    let read_bytes = read_bytes_from_file(file, number_of_bytes)?;

    Ok(String::from_utf8(read_bytes)?)
}

pub fn read_bytes_from_file_as_lossy_string(
    file: &mut File,
    number_of_bytes: usize,
) -> Result<String, Box<dyn Error>> {
    let extracted_bytes = read_bytes_from_file(file, number_of_bytes)?;
    let cleaned_bytes: Vec<u8> = extracted_bytes
        .into_iter()
        .filter(|byte| byte.is_ascii() && !byte.is_zero() && !byte.is_ascii_control())
        .collect();
    Ok(String::from_utf8_lossy(&cleaned_bytes).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn return_correct_canonicalize_path_when_given_path_is_valid() {
        let correct_result =
            env::current_dir().unwrap().to_str().unwrap().to_string() + "/src/main.rs";
        let result = canonicalize_file_path(&"./src/main.rs".to_string()).unwrap();

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
        let result = get_file_name_from_file_path(&"/test/path/filename".to_string()).unwrap();
        assert_eq!(result, "filename")
    }
}
