use crate::bytes::{
    take_first_number_of_bytes, take_first_number_of_bytes_as_string, take_first_two_bytes_as_unsigned_integer, Endian,
};
use crate::errors::LocalError;
use crate::template::Template;
use upon::Value;

const TEMPLATE_NAME: &str = "sndm"; // A short name to identify the template
const TEMPLATE_CONTENT: &str = include_str!("../templates/aiff/sndm.tmpl"); // The file path where you placed the template
const TAG_HEADER_LENGTH_IN_BYTES: usize = 5;
const TAG_ID_LENGTH_IN_BYTES: usize = 3;
const TAG_DATA_SPACER_LENGTH_IN_BYTES: usize = 2;
const GENRE_TAG_ID: &str = "gen";
const AUTHOR_TAG_ID: &str = "aut";
const ALBUM_TAG_ID: &str = "alb";

#[derive(Debug, Clone, Default)]
pub struct SndmFields {
    genre: String,
    author: String,
    album: String,
}

impl SndmFields {
    pub fn new(mut chunk_data: Vec<u8>) -> Result<Self, LocalError> {
        let mut genre = String::new();
        let mut author = String::new();
        let mut album = String::new();

        loop {
            if chunk_data.len() < TAG_HEADER_LENGTH_IN_BYTES {
                break;
            }

            let _throw_away_header_bytes = take_first_number_of_bytes(&mut chunk_data, TAG_HEADER_LENGTH_IN_BYTES)?;
            let tag_id = take_first_number_of_bytes_as_string(&mut chunk_data, TAG_ID_LENGTH_IN_BYTES)?;
            let tag_data_length = take_first_two_bytes_as_unsigned_integer(&mut chunk_data, Endian::Big)?;
            let _throw_away_spacer = take_first_number_of_bytes(&mut chunk_data, TAG_DATA_SPACER_LENGTH_IN_BYTES)?;
            match tag_id.as_str() {
                GENRE_TAG_ID => {
                    genre = take_first_number_of_bytes_as_string(&mut chunk_data, tag_data_length as usize)?;
                }
                AUTHOR_TAG_ID => {
                    author = take_first_number_of_bytes_as_string(&mut chunk_data, tag_data_length as usize)?;
                }
                ALBUM_TAG_ID => {
                    album = take_first_number_of_bytes_as_string(&mut chunk_data, tag_data_length as usize)?;
                }
                _ => break,
            }
        }

        Ok(Self { genre, author, album })
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<String, upon::Error> {
        let aiff_output_values: Value = upon::value! {
            genre: &self.genre,
            author: &self.author,
            album: &self.album,
        };

        let formated_output = template.get_wave_chunk_output(TEMPLATE_NAME, TEMPLATE_CONTENT, aiff_output_values)?;
        Ok(formated_output)
    }
}
