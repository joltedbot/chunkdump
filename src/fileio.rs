use crate::errors::LocalError;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;

pub fn canonicalize_file_path(file_path: &String) -> Result<String, Box<dyn Error>> {
    let path = Path::new(file_path).canonicalize()?;

    let canonical_path = match path.to_str() {
        Some(full_path) => full_path.to_string(),
        None => return Err(Box::new(LocalError::InvalidPath)),
    };

    Ok(canonical_path)
}

pub fn get_file_name_from_file_path(file_path: &String) -> Result<String, Box<dyn Error>> {
    let path = Path::new(file_path);

    let file_name = match path.file_name() {
        Some(file_name) => file_name.to_string_lossy().to_string(),
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
    let chunk_size_bytes = read_bytes_from_file(file, 4)?;
    let mut chunk_size_array: [u8; 4] = Default::default();
    chunk_size_array.copy_from_slice(chunk_size_bytes.as_slice());
    Ok(u32::from_le_bytes(chunk_size_array))
}

pub fn read_bytes_from_file_as_string(
    file: &mut File,
    number_of_bytes: usize,
) -> Result<String, Box<dyn Error>> {
    let extracted_bytes = read_bytes_from_file(file, number_of_bytes)?;
    Ok(String::from_utf8(extracted_bytes)?)
}

pub fn read_bytes_from_file_as_lossy_string(
    file: &mut File,
    number_of_bytes: usize,
) -> Result<String, Box<dyn Error>> {
    let extracted_bytes = read_bytes_from_file(file, number_of_bytes)?;
    let cleaned_bytes: Vec<u8> = extracted_bytes.into_iter().filter(|b| *b != 0).collect();

    Ok(String::from_utf8_lossy(&cleaned_bytes).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn canonicalize_file_path_correct_result() {
        let correct_result =
            env::current_dir().unwrap().to_str().unwrap().to_string() + "/src/main.rs";
        let result = canonicalize_file_path(&"./src/main.rs".to_string()).unwrap();

        assert_eq!(result, correct_result);
    }

    #[test]
    fn get_file_name_from_file_path_returns_correct_result() {
        let result = get_file_name_from_file_path(&"/test/path/filename".to_string()).unwrap();
        assert_eq!(result, "filename")
    }
}
