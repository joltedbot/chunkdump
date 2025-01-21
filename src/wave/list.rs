use crate::errors::LocalError;
use crate::fileio::{read_bytes_from_file_as_string, read_four_byte_integer_from_file};
use std::error::Error;
use std::fs::File;

const LIST_TYPE_LENGTH_IN_BYTES: usize = 4;
const INFO_TYPE_ID: &str = "INFO";
const ADTL_TYPE_ID: &str = "adtl";

const INFO_ITEM_ID: usize = 4;

#[derive(Debug, Clone, Default)]
pub struct ListAdtl {}

pub fn read_list_chunk_fields(
    wave_file: &mut File,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let chunk_size = read_four_byte_integer_from_file(wave_file)?;
    let list_type = read_bytes_from_file_as_string(wave_file, LIST_TYPE_LENGTH_IN_BYTES)?;
    let mut info_data: Vec<(String, String)> = Default::default();

    match list_type.as_str() {
        INFO_TYPE_ID => {
            if list_type == INFO_TYPE_ID {
                info_data =
                    read_info_list(wave_file, chunk_size - LIST_TYPE_LENGTH_IN_BYTES as u32)?;
            }
        }
        ADTL_TYPE_ID => {}
        other => return Err(Box::new(LocalError::InvalidInfoTypeID(other.to_string()))),
    }

    Ok(info_data)
}

fn read_info_list(
    wave_file: &mut File,
    mut data_size: u32,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut list_fields: Vec<(String, String)> = Default::default();

    loop {
        let id = match read_bytes_from_file_as_string(wave_file, INFO_ITEM_ID) {
            Ok(id) => id,
            Err(_) => break,
        };
        let size = read_four_byte_integer_from_file(wave_file)?;
        let data = read_bytes_from_file_as_string(wave_file, size as usize)?;

        list_fields.push((id, data));

        let info_item_size = INFO_ITEM_ID as u32 + 4 + size;

        if data_size > info_item_size {
            data_size -= info_item_size;
        } else {
            break;
        }
    }

    Ok(list_fields)
}
