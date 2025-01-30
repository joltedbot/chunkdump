use crate::byteio::{take_first_four_bytes_as_integer, take_first_number_of_bytes_as_string};
use crate::errors::LocalError;
use crate::fileio::{
    read_bytes_from_file, read_bytes_from_file_as_string, read_chunk_size_from_file,
};
use std::error::Error;
use std::fs::File;

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
    pub info_data: Vec<(String, String)>,
    pub adtl_data: Vec<AssociatedData>,
}

#[derive(Debug, Clone, Default)]
pub struct AssociatedData {
    pub labels: Vec<(u32, String)>,
    pub notes: Vec<(u32, String)>,
    pub labeled_texts: Vec<LabeledText>,
}

#[derive(Debug, Clone, Default)]
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

impl ListFields {
    pub fn new(mut wave_file: &mut File) -> Result<Self, Box<dyn Error>> {
        let chunk_size = read_chunk_size_from_file(wave_file)?;
        let list_type = read_bytes_from_file_as_string(wave_file, LIST_TYPE_LENGTH_IN_BYTES)?;
        let data_size: usize = chunk_size as usize - LIST_TYPE_LENGTH_IN_BYTES;
        let mut data: Vec<u8> = read_bytes_from_file(&mut wave_file, data_size)?;

        let mut list_data: Self = Default::default();

        match list_type.as_str() {
            INFO_TYPE_ID => list_data.info_data = read_info_list(&mut data)?,
            ADTL_TYPE_ID => list_data.adtl_data = read_adtl_list(&mut data)?,
            other => return Err(Box::new(LocalError::InvalidInfoTypeID(other.to_string()))),
        }

        Ok(list_data)
    }

    pub fn get_metadata_output(&self) -> Vec<String> {
        let mut list_data: Vec<String> = vec![];

        list_data.push("\n-------------\nList Chunk Details:\n-------------\n".to_string());
        if !self.info_data.is_empty() {
            list_data.push("Type: Info\n-------------".to_string());
            for info in &self.info_data {
                list_data.push(format!("{:#}:  {}", info.0, info.1));
            }
            list_data.push("".to_string());
        }

        if !self.adtl_data.is_empty() {
            list_data.push("Type: Associated Data List (adtl)\n-------------".to_string());

            for adtl in &self.adtl_data {
                if !adtl.labels.is_empty() {
                    for label in &adtl.labels {
                        list_data.push(format!("Label {:#}:  {}\n", label.0, label.1));
                    }
                }
                if !adtl.notes.is_empty() {
                    for note in &adtl.notes {
                        list_data.push(format!("Note {:#}:  {}\n", note.0, note.1));
                    }
                }
                if !adtl.labeled_texts.is_empty() {
                    list_data.push("Labeled Text:\n-------------".to_string());
                    for text in &adtl.labeled_texts {
                        list_data.push(format!("Cue Point ID: {}", text.cue_point_id));
                        list_data.push(format!("Sample Length: {}", text.sample_length));
                        list_data.push(format!("Purpose ID: {}", text.purpose_id));
                        list_data.push(format!("Country: {}", text.country));
                        list_data.push(format!("Language: {}", text.language));
                        list_data.push(format!("Dialect: {}", text.dialect));
                        list_data.push(format!("Code Page: {}", text.code_page));
                        list_data.push(format!("Data: {}", text.data));
                        list_data.push("".to_string());
                    }
                }
            }
        }

        list_data
    }
}

fn read_info_list(list_data: &mut Vec<u8>) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut list_fields: Vec<(String, String)> = Default::default();
    loop {
        if list_data.is_empty() {
            break;
        }

        let id = take_first_number_of_bytes_as_string(list_data, INFO_ITEM_ID_LENGTH_IN_BYTES)?;
        let mut size = take_first_four_bytes_as_integer(list_data)?;

        if size % 2 > 0 {
            size += 1;
        }

        let data = take_first_number_of_bytes_as_string(list_data, size as usize)?;

        list_fields.push((id, data));
    }

    Ok(list_fields)
}

fn read_adtl_list(list_data: &mut Vec<u8>) -> Result<Vec<AssociatedData>, Box<dyn Error>> {
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
                let label: (u32, String) = read_label_data(list_data)?;
                associated_data.labels.push(label);
            }
            ADTL_SUB_CHUNK_ID_NOTE => {
                let note: (u32, String) = read_note_data(list_data)?;
                associated_data.notes.push(note);
            }
            ADTL_SUB_CHUNK_ID_LABELED_TEXT => {
                let labeled_text: LabeledText = read_labeled_text_data(list_data)?;
                associated_data.labeled_texts.push(labeled_text);
            }
            other => return Err(Box::new(LocalError::InvalidADTLTypeID(other.to_string()))),
        }

        adtl_list_data.push(associated_data);
    }

    Ok(adtl_list_data)
}

fn read_label_data(list_data: &mut Vec<u8>) -> Result<(u32, String), Box<dyn Error>> {
    let label_size: usize = take_first_four_bytes_as_integer(list_data)? as usize;
    let data_size: usize = label_size - ADTL_CUE_POINT_ID_LENGTH_IN_BYTES;
    let cue_point_id: u32 = take_first_four_bytes_as_integer(list_data)?;
    let label_data: String = take_first_number_of_bytes_as_string(list_data, data_size)?;
    Ok((cue_point_id, label_data))
}

fn read_note_data(list_data: &mut Vec<u8>) -> Result<(u32, String), Box<dyn Error>> {
    let note_size: usize = take_first_four_bytes_as_integer(list_data)? as usize;
    let data_size: usize = note_size - ADTL_CUE_POINT_ID_LENGTH_IN_BYTES;
    let cue_point_id: u32 = take_first_four_bytes_as_integer(list_data)?;
    let note_data: String = take_first_number_of_bytes_as_string(list_data, data_size)?;
    Ok((cue_point_id, note_data))
}

fn read_labeled_text_data(list_data: &mut Vec<u8>) -> Result<LabeledText, Box<dyn Error>> {
    let cue_point_id: u32 = take_first_four_bytes_as_integer(list_data)?;
    let sample_length: u32 = take_first_four_bytes_as_integer(list_data)?;
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
