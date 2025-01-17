use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek};

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

pub fn read_two_byte_integer_from_file(file: &mut File) -> Result<u16, Box<dyn Error>> {
    let chunk_size_bytes = read_bytes_from_file(file, 2)?;
    let mut chunk_size_array: [u8; 2] = Default::default();
    chunk_size_array.copy_from_slice(chunk_size_bytes.as_slice());
    Ok(u16::from_le_bytes(chunk_size_array))
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
