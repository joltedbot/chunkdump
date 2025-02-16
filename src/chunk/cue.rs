use crate::byteio::{take_first_four_bytes_as_unsigned_integer, take_first_number_of_bytes_as_string};
use crate::errors::LocalError;
use crate::template::Template;
use serde::Serialize;
use upon::Value;

const TEMPLATE_NAME: &str = "cue";
const TEMPLATE_CONTENT: &str = include_str!("../templates/wave/cue.tmpl");
const DATA_CHUNK_ID_LENGTH_IN_BYTES: usize = 4;

#[derive(Debug, Clone, Default)]
pub struct CueFields {
    pub template_name: &'static str,
    pub template_content: &'static str,
    pub number_of_cue_points: u32,
    pub cue_points: Vec<CuePoint>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CuePoint {
    pub id: u32,
    pub position: u32,
    pub data_chunk_id: String,
    pub chunk_start: u32,
    pub block_start: u32,
    pub sample_start: u32,
}

impl CueFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, LocalError> {
        let mut cue_points: Vec<CuePoint> = vec![];
        let number_of_cue_points: u32 = take_first_four_bytes_as_unsigned_integer(&mut chunk_data)?;

        for _ in 0..number_of_cue_points {
            cue_points.push(CuePoint {
                id: take_first_four_bytes_as_unsigned_integer(&mut chunk_data)?,
                position: take_first_four_bytes_as_unsigned_integer(&mut chunk_data)?,
                data_chunk_id: take_first_number_of_bytes_as_string(&mut chunk_data, DATA_CHUNK_ID_LENGTH_IN_BYTES)?,
                chunk_start: take_first_four_bytes_as_unsigned_integer(&mut chunk_data)?,
                block_start: take_first_four_bytes_as_unsigned_integer(&mut chunk_data)?,
                sample_start: take_first_four_bytes_as_unsigned_integer(&mut chunk_data)?,
            })
        }

        Ok(Self {
            template_name: TEMPLATE_NAME,
            template_content: TEMPLATE_CONTENT,
            number_of_cue_points,
            cue_points,
        })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        let wave_output_values: Value = upon::value! {
                number_of_cue_points: &self.number_of_cue_points,
                cue_points: &self.cue_points
        };

        let formated_output =
            template.get_wave_chunk_output(self.template_name, self.template_content, wave_output_values)?;

        Ok(formated_output)
    }
}
