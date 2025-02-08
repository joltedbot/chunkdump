use crate::byteio::{
    take_first_eight_bytes_as_unsigned_integer, take_first_number_of_bytes, take_first_number_of_bytes_as_string,
    take_first_two_bytes_as_signed_integer, take_first_two_bytes_as_unsigned_integer,
};

use crate::template::Template;
use serde::Serialize;
use std::error::Error;
use upon::Value;

const TEMPLATE_NAME: &str = "bext";
const TEMPLATE_PATH: &str = include_str!("../templates/wave/bext.tmpl");

const DESCRIPTION_LENGTH_IN_BYTES: usize = 256;
const ORIGINATOR_LENGTH_IN_BYTES: usize = 32;
const ORIGINATOR_REFERENCE_LENGTH_IN_BYTES: usize = 32;
const ORIGINATOR_DATA_LENGTH_IN_BYTES: usize = 10;
const ORIGINATOR_TIME_LENGTH_IN_BYTES: usize = 8;
const RESERVED_FIELD_LENGTH_IN_BYTES: usize = 180;
const UMID_UNIVERSAL_LABEL_LENGTH_IN_BYTES: usize = 8;
const UMID_LENGTH_LENGTH_IN_BYTES: usize = 1;
const UMID_INSTANCE_NUMBER_LENGTH_IN_BYTES: usize = 3;
const UMID_MATERIAL_NUMBER_LEMGTH_IN_BYTES: usize = 16;
const UMID_TIME_AND_DATE_LENGTH_IN_BYTES: usize = 8;
const UMID_SPATIAL_COORDINATES_LENGTH_IN_BYTES: usize = 12;
const UMID_COUNTRY_LENGTH_IN_BYTES: usize = 4;
const UMID_ORGANIZATION_LENGTH_IN_BYTES: usize = 4;
const UMID_USER_LENGTH_IN_BYTES: usize = 4;

#[derive(Debug, Clone, Default, Serialize)]
pub struct UmidComponent {
    universal_label: Vec<u8>,
    length: Vec<u8>,
    instance_number: Vec<u8>,
    material_number: Vec<u8>,
    time_and_date: Vec<u8>,
    spatial_coordinates: Vec<u8>,
    country: Vec<u8>,
    organization: Vec<u8>,
    user: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct BextFields {
    pub template_name: &'static str,
    pub template_path: &'static str,
    pub description: String,
    pub originator: String,
    pub originator_reference: String,
    pub originator_date: String,
    pub originator_time: String,
    pub time_reference: u64,
    pub version: u16,
    pub umid: UmidComponent,
    pub loudness_value: i16,
    pub loudness_range: i16,
    pub max_true_peak_level: i16,
    pub max_momentary_loudness: i16,
    pub max_short_term_loudness: i16,
    pub reserved: String,
    pub coding_history: String,
}

impl BextFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            template_name: TEMPLATE_NAME,
            template_path: TEMPLATE_PATH,
            description: take_first_number_of_bytes_as_string(&mut chunk_data, DESCRIPTION_LENGTH_IN_BYTES)?,
            originator: take_first_number_of_bytes_as_string(&mut chunk_data, ORIGINATOR_LENGTH_IN_BYTES)?,
            originator_reference: take_first_number_of_bytes_as_string(
                &mut chunk_data,
                ORIGINATOR_REFERENCE_LENGTH_IN_BYTES,
            )?,
            originator_date: take_first_number_of_bytes_as_string(&mut chunk_data, ORIGINATOR_DATA_LENGTH_IN_BYTES)?,
            originator_time: take_first_number_of_bytes_as_string(&mut chunk_data, ORIGINATOR_TIME_LENGTH_IN_BYTES)?,
            time_reference: take_first_eight_bytes_as_unsigned_integer(&mut chunk_data)?,
            version: take_first_two_bytes_as_unsigned_integer(&mut chunk_data)?,
            umid: get_umid_from_bytes(&mut chunk_data)?,
            loudness_value: take_first_two_bytes_as_signed_integer(&mut chunk_data)?,
            loudness_range: take_first_two_bytes_as_signed_integer(&mut chunk_data)?,
            max_true_peak_level: take_first_two_bytes_as_signed_integer(&mut chunk_data)?,
            max_momentary_loudness: take_first_two_bytes_as_signed_integer(&mut chunk_data)?,
            max_short_term_loudness: take_first_two_bytes_as_signed_integer(&mut chunk_data)?,
            reserved: take_first_number_of_bytes_as_string(&mut chunk_data, RESERVED_FIELD_LENGTH_IN_BYTES)?,
            coding_history: get_coding_history_from_bytes(chunk_data)?,
        })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, Box<dyn Error>> {
        let wave_output_values: Value = upon::value! {
            description: &self.description,
            originator: &self.originator,
            originator_reference: &self.originator_reference,
            originator_date: &self.originator_date,
            originator_time: &self.originator_time,
            time_reference: self.time_reference,
            version: self.version,
            loudness_value: self.loudness_value / 100,
            loudness_range: self.loudness_range / 100,
            max_true_peak_level: self.max_true_peak_level / 100,
            max_momentary_loudness: self.max_momentary_loudness / 100,
            max_short_term_loudness: self.max_short_term_loudness / 100,
            reserved: &self.reserved,
            coding_history: &self.coding_history,
            universal_label: format_bytes_as_sting(&self.umid.universal_label),
            instance_number: format_bytes_as_sting(&self.umid.instance_number),
            material_number: format_bytes_as_sting(&self.umid.material_number),
            time_and_date: format_bytes_as_sting(&self.umid.time_and_date),
            spatial_coordinates: format_bytes_as_sting(&self.umid.spatial_coordinates),
            country: format_bytes_as_sting(&self.umid.country),
            organization: format_bytes_as_sting(&self.umid.organization),
            user: format_bytes_as_sting(&self.umid.user),
        };

        let formated_output = template.get_wave_chunk_output(self.template_name, self.template_path, wave_output_values)?;
        Ok(formated_output)
    }
}

fn get_coding_history_from_bytes(mut bext_data: Vec<u8>) -> Result<String, Box<dyn Error>> {
    let mut coding_history = "".to_string();

    if !bext_data.is_empty() {
        let bext_data_remaining_bytes = bext_data.len();
        coding_history = take_first_number_of_bytes_as_string(&mut bext_data, bext_data_remaining_bytes)?;
    }

    Ok(coding_history)
}

fn get_umid_from_bytes(umid_data: &mut Vec<u8>) -> Result<UmidComponent, Box<dyn Error>> {
    Ok(UmidComponent {
        universal_label: take_first_number_of_bytes(umid_data, UMID_UNIVERSAL_LABEL_LENGTH_IN_BYTES)?,
        length: take_first_number_of_bytes(umid_data, UMID_LENGTH_LENGTH_IN_BYTES)?,
        instance_number: take_first_number_of_bytes(umid_data, UMID_INSTANCE_NUMBER_LENGTH_IN_BYTES)?,
        material_number: take_first_number_of_bytes(umid_data, UMID_MATERIAL_NUMBER_LEMGTH_IN_BYTES)?,
        time_and_date: take_first_number_of_bytes(umid_data, UMID_TIME_AND_DATE_LENGTH_IN_BYTES)?,
        spatial_coordinates: take_first_number_of_bytes(umid_data, UMID_SPATIAL_COORDINATES_LENGTH_IN_BYTES)?,
        country: take_first_number_of_bytes(umid_data, UMID_COUNTRY_LENGTH_IN_BYTES)?,
        organization: take_first_number_of_bytes(umid_data, UMID_ORGANIZATION_LENGTH_IN_BYTES)?,
        user: take_first_number_of_bytes(umid_data, UMID_USER_LENGTH_IN_BYTES)?,
    })
}

fn format_bytes_as_sting(bytes: &[u8]) -> String {
    bytes
        .iter()
        .fold("".to_string(), |umid: String, byte| format!("{} 0x{:02X?}", umid, byte))
}
