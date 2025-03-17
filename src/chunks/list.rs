use crate::byte_arrays::{
    take_first_four_bytes_as_unsigned_integer, take_first_number_of_bytes_as_string, Endian,
};
use crate::errors::LocalError;
use crate::formating::add_one_if_byte_size_is_odd;
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use serde::Serialize;
use std::error::Error;

const INFO_TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/list_info.tmpl");
const ADTL_TEMPLATE_CONTENT: &str = include_str!("../templates/chunks/list_adtl.tmpl");
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

#[derive(Serialize)]
struct InfoData {
    id: String,
    data: String,
}

#[derive(Debug, Clone, Serialize)]
struct LabelData {
    cue_point_id: u32,
    data: String,
}

#[derive(Debug, Clone, Serialize)]
struct NoteData {
    cue_point_id: u32,
    data: String,
}

#[derive(Debug, Clone, Serialize)]
struct LabeledText {
    cue_point_id: u32,
    sample_length: u32,
    purpose_id: String,
    country: String,
    language: String,
    dialect: String,
    code_page: String,
    data: String,
}

#[derive(Default)]
struct AssociatedData {
    labels: Vec<LabelData>,
    notes: Vec<NoteData>,
    labeled_texts: Vec<LabeledText>,
}

pub fn get_metadata(mut chunk_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let list_type =
        take_first_number_of_bytes_as_string(&mut chunk_data, LIST_TYPE_LENGTH_IN_BYTES)?;

    let mut info_data: Vec<InfoData> = Vec::new();
    let mut adtl_data: Vec<AssociatedData> = Vec::new();

    match list_type.as_str() {
        INFO_TYPE_ID => info_data = parse_info_data(&mut chunk_data)?,
        ADTL_TYPE_ID => adtl_data = parse_adtl_data(&mut chunk_data)?,
        _ => {}
    }

    let mut info_output = String::new();
    if !info_data.is_empty() {
        let info_output_values = upon::value! {
                info_items: &info_data,
        };

        info_output = get_file_chunk_output(INFO_TEMPLATE_CONTENT, info_output_values)?;
    }

    let mut adtl_output: String = String::new();
    adtl_data.iter().for_each(|field| {
        adtl_output += &get_file_chunk_output(
            ADTL_TEMPLATE_CONTENT,
            upon::value! {
                labels: &field.labels,
                notes: &field.notes,
                labeled_texts: &field.labeled_texts,
            },
        )
        .unwrap_or_default();
    });

    let formated_output = info_output + &adtl_output;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}

fn parse_info_data(list_data: &mut Vec<u8>) -> Result<Vec<InfoData>, LocalError> {
    let mut list_fields: Vec<InfoData> = Default::default();
    loop {
        if list_data.is_empty() {
            break;
        }

        let id = take_first_number_of_bytes_as_string(list_data, INFO_ITEM_ID_LENGTH_IN_BYTES)?;
        let mut size = take_first_four_bytes_as_unsigned_integer(list_data, Endian::Little)?;
        size = add_one_if_byte_size_is_odd(size);
        let data = take_first_number_of_bytes_as_string(list_data, size as usize)?;

        list_fields.push(InfoData { id, data });
    }

    Ok(list_fields)
}

fn parse_adtl_data(list_data: &mut Vec<u8>) -> Result<Vec<AssociatedData>, LocalError> {
    let mut adtl_list_data: Vec<AssociatedData> = Default::default();

    loop {
        if list_data.is_empty() {
            break;
        }

        let mut associated_data: AssociatedData = Default::default();

        match take_first_number_of_bytes_as_string(list_data, ADTL_SUB_CHUNK_ID_LENGTH_IN_BYTES)?
            .as_str()
        {
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
            other => return Err(LocalError::InvalidADTLTypeID(other.to_string())),
        }

        adtl_list_data.push(associated_data);
    }

    Ok(adtl_list_data)
}

fn parse_label_data(list_data: &mut Vec<u8>) -> Result<LabelData, LocalError> {
    let label_size: usize =
        take_first_four_bytes_as_unsigned_integer(list_data, Endian::Little)? as usize;
    let data_size: usize =
        add_one_if_byte_size_is_odd((label_size - ADTL_CUE_POINT_ID_LENGTH_IN_BYTES) as u32)
            as usize;
    let cue_point_id: u32 = take_first_four_bytes_as_unsigned_integer(list_data, Endian::Little)?;
    let label_data: String = take_first_number_of_bytes_as_string(list_data, data_size)?;
    Ok(LabelData {
        cue_point_id,
        data: label_data,
    })
}

fn parse_note_data(list_data: &mut Vec<u8>) -> Result<NoteData, LocalError> {
    let note_size: usize =
        take_first_four_bytes_as_unsigned_integer(list_data, Endian::Little)? as usize;
    let data_size: usize = note_size - ADTL_CUE_POINT_ID_LENGTH_IN_BYTES;
    let cue_point_id: u32 = take_first_four_bytes_as_unsigned_integer(list_data, Endian::Little)?;
    let note_data: String = take_first_number_of_bytes_as_string(list_data, data_size)?;
    Ok(NoteData {
        cue_point_id,
        data: note_data,
    })
}

fn data_labeled_text_data(list_data: &mut Vec<u8>) -> Result<LabeledText, LocalError> {
    let cue_point_id: u32 = take_first_four_bytes_as_unsigned_integer(list_data, Endian::Little)?;
    let sample_length: u32 = take_first_four_bytes_as_unsigned_integer(list_data, Endian::Little)?;
    let purpose_id: String =
        take_first_number_of_bytes_as_string(list_data, ADTL_PURPOSE_ID_LENGTH_IN_BYTES)?;
    let country: String =
        take_first_number_of_bytes_as_string(list_data, ADTL_COUNTRY_LENGTH_IN_BYTES)?;
    let language: String =
        take_first_number_of_bytes_as_string(list_data, ADTL_LANGUAGE_LENGTH_IN_BYTES)?;
    let dialect: String =
        take_first_number_of_bytes_as_string(list_data, ADTL_DIALECT_LENGTH_IN_BYTES)?;
    let code_page: String =
        take_first_number_of_bytes_as_string(list_data, ADTL_CODE_PAGE_LENGTH_IN_BYTES)?;
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
