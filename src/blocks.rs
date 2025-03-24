mod application;
mod cuesheet;
mod extra;
mod padding;
mod picture;
mod seek_table;
mod stream_info;
mod vorbis_comment;

use crate::byte_arrays::{take_first_four_bytes_as_unsigned_integer, Endian};
use crate::fileio::{read_byte_from_file, read_bytes_from_file, skip_over_bytes_in_file};
use crate::output::OutputEntry;
use std::error::Error;
use std::fs::File;

const DECIMAL_REPRESENTATION_OF_IS_LAST_BLOCK_BIT_POSITION: u32 = 128;
const FLAC_FILE_SIGNATURE_LENGTH_IN_BYTES: usize = 4;
const BLOCK_LENGTH_FIELD_IN_BYTES: usize = 3;
const STREAM_INFO_BLOCK_ID: u32 = 0;
const PADDING_BLOCK_ID: u32 = 1;
const APPLICATION_BLOCK_ID: u32 = 2;
const SEEK_TABLE_BLOCK_ID: u32 = 3;
const VORBIS_COMMENT_BLOCK_ID: u32 = 4;
const CUE_SHEET_BLOCK_ID: u32 = 5;
const PICTURE_BLOCK_ID: u32 = 6;

struct MetadataBlock {
    header_type: u32,
    is_last_block: bool,
    data: Vec<u8>,
}

pub fn get_metadata_from_blocks(
    flac_file: &mut File,
    mandatory_sections_only: bool,
) -> Result<Vec<OutputEntry>, Box<dyn Error>> {
    let mut output: Vec<OutputEntry> = vec![];

    skip_over_bytes_in_file(flac_file, FLAC_FILE_SIGNATURE_LENGTH_IN_BYTES)?;

    loop {
        let metadata_block = read_metadata_block_from_file(flac_file)?;
        let metadata_output = get_block_metadata(metadata_block.header_type, metadata_block.data)?;
        output.push(metadata_output);

        if metadata_block.is_last_block || mandatory_sections_only {
            break;
        }
    }

    Ok(output)
}

pub fn get_block_metadata(
    block_type: u32,
    block_data: Vec<u8>,
) -> Result<OutputEntry, Box<dyn Error>> {
    let result = match block_type {
        STREAM_INFO_BLOCK_ID => stream_info::get_metadata(block_data)?,
        PADDING_BLOCK_ID => padding::get_metadata(block_data)?,
        APPLICATION_BLOCK_ID => application::get_metadata(block_data)?,
        SEEK_TABLE_BLOCK_ID => seek_table::get_metadata(block_data)?,
        VORBIS_COMMENT_BLOCK_ID => vorbis_comment::get_metadata(block_data)?,
        CUE_SHEET_BLOCK_ID => cuesheet::get_metadata(block_data)?,
        PICTURE_BLOCK_ID => picture::get_metadata(block_data)?,
        _ => extra::get_metadata(block_type, block_data)?,
    };

    Ok(result)
}

fn read_metadata_block_from_file(flac_file: &mut File) -> Result<MetadataBlock, Box<dyn Error>> {
    let header_byte = read_byte_from_file(flac_file)?;
    let mut block_data_length_bytes = read_bytes_from_file(flac_file, BLOCK_LENGTH_FIELD_IN_BYTES)?;

    let header_type = get_header_type_from_header_byte(header_byte);
    let is_last_block: bool = (header_byte >> 7) == 1;
    let block_data_length = get_block_data_length_from_bytes(&mut block_data_length_bytes)?;

    let data = read_bytes_from_file(flac_file, block_data_length as usize)?;

    let metadata = MetadataBlock {
        header_type,
        is_last_block,
        data,
    };

    Ok(metadata)
}

fn get_header_type_from_header_byte(header_byte: u8) -> u32 {
    let mut header_type = header_byte as u32;

    if header_type >= DECIMAL_REPRESENTATION_OF_IS_LAST_BLOCK_BIT_POSITION {
        header_type -= DECIMAL_REPRESENTATION_OF_IS_LAST_BLOCK_BIT_POSITION;
    }

    header_type
}

fn get_block_data_length_from_bytes(block_data: &mut Vec<u8>) -> Result<u32, Box<dyn Error>> {
    block_data.insert(0, 0x00);
    let block_data_length = take_first_four_bytes_as_unsigned_integer(block_data, Endian::Big)?;
    Ok(block_data_length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_correct_header_type_from_byte_when_not_the_last_block() {
        let test_byte: u8 = 130;
        let correct_response: u32 = 2;
        let header_type = get_header_type_from_header_byte(test_byte);
        assert_eq!(header_type, correct_response);
    }

    #[test]
    fn return_correct_header_type_from_byte_when_it_is_the_last_block() {
        let test_byte: u8 = 2;
        let correct_result: u32 = 2;
        let header_type = get_header_type_from_header_byte(test_byte);
        assert_eq!(header_type, correct_result);
    }

    #[test]
    fn return_correct_u32_block_data_length_from_3_bytes_vector() {
        let mut test_data: Vec<u8> = vec![0x01, 0x01, 0x01];
        let correct_result = 65793;
        let result: u32 = get_block_data_length_from_bytes(&mut test_data).unwrap();
        assert_eq!(result, correct_result);
    }
}
