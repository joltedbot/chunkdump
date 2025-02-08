use crate::byteio::{take_first_four_bytes_as_unsigned_integer, take_first_number_of_bytes_as_string};
use crate::errors::LocalError;
use crate::template::Template;
use crate::wave::add_one_if_byte_size_is_odd;
use serde::Serialize;
use std::error::Error;

pub const INFO_TEMPLATE_NAME: &str = "list_info";
pub const ADTL_TEMPLATE_NAME: &str = "adtl_info";
const INFO_TEMPLATE_PATH: &str = include_str!("../templates/wave/list_info.tmpl");
const ADTL_TEMPLATE_PATH: &str = include_str!("../templates/wave/list_adtl.tmpl");

const LIST_TYPE_LENGTH_IN_BYTES: usize = 4;
const INFO_TYPE_ID: &str = "INFO";
const ADTL_TYPE_ID: &str = "adtl";

const INFO_ITEM_ID_LENGTH_IN_BYTES: usize = 4;
const ADTL_SUB_CHUNK_ID_LENGTH_IN_BYTES: usize = 4;
const ADTL_SUB_CHUNK_ID_LABEL: &str = "labl";
const ADTL_SUB_CHUNK_ID_NOTE: &str = "note";
const ADTL_SUB_CHUNK_ID_LABELED_TEXT: &str = "ltxt";
const ADTL_CUE_POINT_ID_LENGTH_IN_BYTES: usize = 4;
const ADTL_PURPOSE_ID_LENGTH_IN_BYTES: usize = 4;
const ADTL_COUNTRY_LENGTH_IN_BYTES: usize = 2;
const ADTL_LANGUAGE_LENGTH_IN_BYTES: usize = 2;
const ADTL_DIALECT_LENGTH_IN_BYTES: usize = 2;
const ADTL_CODE_PAGE_LENGTH_IN_BYTES: usize = 2;

#[derive(Debug, Clone, Default)]
pub struct ListFields {
    pub info_template_name: &'static str,
    pub adtl_template_name: &'static str,
    pub info_template_path: &'static str,
    pub adtl_template_path: &'static str,
    pub info_data: Vec<InfoData>,
    pub adtl_data: Vec<AssociatedData>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct InfoData {
    pub id: String,
    pub data: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LabelData {
    pub cue_point_id: u32,
    pub data: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct NoteData {
    pub cue_point_id: u32,
    pub data: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LabeledText {
    cue_point_id: u32,
    sample_length: u32,
    purpose_id: String,
    country: String,
    language: String,
    dialect: String,
    code_page: String,
    data: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AssociatedData {
    pub labels: Vec<LabelData>,
    pub notes: Vec<NoteData>,
    pub labeled_texts: Vec<LabeledText>,
}

impl ListFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let list_type = take_first_number_of_bytes_as_string(&mut chunk_data, LIST_TYPE_LENGTH_IN_BYTES)?;

        let mut list_data: Self = Default::default();
        match list_type.as_str() {
            INFO_TYPE_ID => list_data.info_data = parse_info_data(&mut chunk_data)?,
            ADTL_TYPE_ID => list_data.adtl_data = parse_adtl_data(&mut chunk_data)?,
            other => return Err(Box::new(LocalError::InvalidInfoTypeID(other.to_string()))),
        }

        list_data.info_template_name = INFO_TEMPLATE_NAME;
        list_data.adtl_template_name = ADTL_TEMPLATE_NAME;
        list_data.info_template_path = INFO_TEMPLATE_PATH;
        list_data.adtl_template_path = ADTL_TEMPLATE_PATH;

        Ok(list_data)
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, Box<dyn Error>> {
        let info_output_values = upon::value! {
                info_items: self.info_data.clone(),
        };

        let info_output =
            template.get_wave_chunk_output(self.info_template_name, self.info_template_path, info_output_values)?;

        let mut adtl_output: String = String::new();
        for field in &self.adtl_data {
            adtl_output += &template.get_wave_chunk_output(
                self.adtl_template_name,
                self.adtl_template_path,
                upon::value! {
                    labels: &field.labels,
                    notes: &field.notes,
                    labeled_texts: &field.labeled_texts,
                },
            )?;
        }

        Ok(info_output + &adtl_output)
    }
}

fn parse_info_data(list_data: &mut Vec<u8>) -> Result<Vec<InfoData>, Box<dyn Error>> {
    let mut list_fields: Vec<InfoData> = Default::default();
    loop {
        if list_data.is_empty() {
            break;
        }

        let id = take_first_number_of_bytes_as_string(list_data, INFO_ITEM_ID_LENGTH_IN_BYTES)?;
        let mut size = take_first_four_bytes_as_unsigned_integer(list_data)?;
        size = add_one_if_byte_size_is_odd(size);
        let data = take_first_number_of_bytes_as_string(list_data, size as usize)?;

        list_fields.push(InfoData { id, data });
    }

    Ok(list_fields)
}

fn parse_adtl_data(list_data: &mut Vec<u8>) -> Result<Vec<AssociatedData>, Box<dyn Error>> {
    let mut adtl_list_data: Vec<AssociatedData> = Default::default();

    loop {
        if list_data.is_empty() {
            break;
        }

        let mut associated_data: AssociatedData = Default::default();

        match take_first_number_of_bytes_as_string(list_data, ADTL_SUB_CHUNK_ID_LENGTH_IN_BYTES)?.as_str() {
            ADTL_SUB_CHUNK_ID_LABEL => {
                let label: LabelData = parse_label_data(list_data)?;
                associated_data.labels.push(label);
            }
            ADTL_SUB_CHUNK_ID_NOTE => {
                let note: NoteData = parse_note_data(list_data)?;
                associated_data.notes.push(note);
            }
            ADTL_SUB_CHUNK_ID_LABELED_TEXT => {
                let labeled_text: LabeledText = data_labeled_text_data(list_data)?;
                associated_data.labeled_texts.push(labeled_text);
            }
            other => return Err(Box::new(LocalError::InvalidADTLTypeID(other.to_string()))),
        }

        adtl_list_data.push(associated_data);
    }

    Ok(adtl_list_data)
}

fn parse_label_data(list_data: &mut Vec<u8>) -> Result<LabelData, Box<dyn Error>> {
    let label_size: usize = take_first_four_bytes_as_unsigned_integer(list_data)? as usize;
    let data_size: usize = label_size - ADTL_CUE_POINT_ID_LENGTH_IN_BYTES;
    let cue_point_id: u32 = take_first_four_bytes_as_unsigned_integer(list_data)?;
    let label_data: String = take_first_number_of_bytes_as_string(list_data, data_size)?;
    Ok(LabelData {
        cue_point_id,
        data: label_data,
    })
}

fn parse_note_data(list_data: &mut Vec<u8>) -> Result<NoteData, Box<dyn Error>> {
    let note_size: usize = take_first_four_bytes_as_unsigned_integer(list_data)? as usize;
    let data_size: usize = note_size - ADTL_CUE_POINT_ID_LENGTH_IN_BYTES;
    let cue_point_id: u32 = take_first_four_bytes_as_unsigned_integer(list_data)?;
    let note_data: String = take_first_number_of_bytes_as_string(list_data, data_size)?;
    Ok(NoteData {
        cue_point_id,
        data: note_data,
    })
}

fn data_labeled_text_data(list_data: &mut Vec<u8>) -> Result<LabeledText, Box<dyn Error>> {
    let cue_point_id: u32 = take_first_four_bytes_as_unsigned_integer(list_data)?;
    let sample_length: u32 = take_first_four_bytes_as_unsigned_integer(list_data)?;
    let purpose_id: String = take_first_number_of_bytes_as_string(list_data, ADTL_PURPOSE_ID_LENGTH_IN_BYTES)?;
    let country: String = take_first_number_of_bytes_as_string(list_data, ADTL_COUNTRY_LENGTH_IN_BYTES)?;
    let language: String = take_first_number_of_bytes_as_string(list_data, ADTL_LANGUAGE_LENGTH_IN_BYTES)?;
    let dialect: String = take_first_number_of_bytes_as_string(list_data, ADTL_DIALECT_LENGTH_IN_BYTES)?;
    let code_page: String = take_first_number_of_bytes_as_string(list_data, ADTL_CODE_PAGE_LENGTH_IN_BYTES)?;
    let data: String = take_first_number_of_bytes_as_string(list_data, list_data.len())?;

    Ok(LabeledText {
        cue_point_id,
        sample_length,
        purpose_id,
        country,
        language,
        dialect,
        code_page,
        data,
    })
}
