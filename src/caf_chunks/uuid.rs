use crate::byte_arrays::{take_first_number_of_bytes, take_first_number_of_bytes_as_string};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use std::error::Error;
use upon::Value;
use uuid::{Uuid, Version};

const TEMPLATE_CONTENT: &str = include_str!("../templates/caf_chunks/uuid.tmpl");
const REMAINING_DATA_MESSAGE: &str =
    "** Additional data exists but in a non-standard format. Here is an attempt to render it as a string";
const UUID_LENGTH_AS_BYTES: usize = 16;

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let uuid_bytes = take_first_number_of_bytes(&mut chunk_data, 16)?;
    let uuid_string: String = get_string_of_hexbytes_from_bytes(&uuid_bytes);
    let uuid = Uuid::parse_str(uuid_string.as_str())?;

    let variant = format!("{:?}", uuid.get_variant());
    let version = format!("{:?}", uuid.get_version().unwrap_or(Version::Nil));
    let version_number = uuid.get_version_num();

    let timestamp = match uuid.get_timestamp() {
        Some(time) => format!("{:?}", time),
        None => "None".to_string(),
    };

    let node_id = match uuid.get_node_id() {
        Some(node_id) => format!("{:?}", node_id),
        None => "None".to_string(),
    };

    let mut remaining_data: String = "".to_string();
    let chunk_data_length = chunk_data.len();
    if chunk_data_length > UUID_LENGTH_AS_BYTES {
        let remaining_data_as_string =
            take_first_number_of_bytes_as_string(&mut chunk_data, chunk_data_length)?;
        remaining_data = format!(
            "{} \n\n {}",
            REMAINING_DATA_MESSAGE, remaining_data_as_string
        );
    }

    let output_values: Value = upon::value! {
        node_id: node_id,
        timestamp: timestamp,
        variant: variant,
        version: version,
        version_number: version_number,
        remaining_data: remaining_data,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}

pub fn get_string_of_hexbytes_from_bytes(bytes: &[u8]) -> String {
    let output_string = bytes.iter().fold("".to_string(), |umid: String, byte| {
        format!("{}{:02x?}", umid, byte)
    });

    output_string.trim().to_string()
}
