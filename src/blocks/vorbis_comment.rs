use crate::byte_arrays::{take_first_four_bytes_as_unsigned_integer, take_first_number_of_bytes_as_string, Endian};
use crate::output::{OutputEntry, Section};
use crate::template::get_file_chunk_output;
use serde::Serialize;
use std::error::Error;
use upon::Value;

const TEMPLATE_CONTENT: &str = include_str!("../templates/blocks/vorbis_comments.tmpl");

#[derive(Debug, PartialEq, Serialize)]
struct VorbisTag {
    key: String,
    spacer: String,
    value: String,
}

pub fn get_metadata(mut block_data: Vec<u8>) -> Result<OutputEntry, Box<dyn Error>> {
    let vorbis_vendor_length = take_first_four_bytes_as_unsigned_integer(&mut block_data, Endian::Little)?;
    let vorbis_vendor = take_first_number_of_bytes_as_string(&mut block_data, vorbis_vendor_length as usize)?;
    let number_of_tags = take_first_four_bytes_as_unsigned_integer(&mut block_data, Endian::Little)?;

    let vorbis_tags = get_vorbis_comment_tags(&mut block_data, number_of_tags)?;

    let output_values: Value = upon::value! {
        vorbis_vendor: vorbis_vendor,
        vorbis_tags: vorbis_tags,
    };

    let formated_output = get_file_chunk_output(TEMPLATE_CONTENT, output_values)?;

    Ok(OutputEntry {
        section: Section::Optional,
        text: formated_output,
    })
}

fn get_vorbis_comment_tags(block_data: &mut Vec<u8>, number_of_tags: u32) -> Result<Vec<VorbisTag>, Box<dyn Error>> {
    let mut vorbis_tags: Vec<VorbisTag> = vec![];

    for _ in 0..number_of_tags {
        let tag_length = take_first_four_bytes_as_unsigned_integer(block_data, Endian::Little)? as usize;
        let raw_tag = take_first_number_of_bytes_as_string(block_data, tag_length)?;

        let tag_key_and_value = match raw_tag.split_once('=') {
            Some((key, value)) => (key.trim(), value.trim()),
            None => continue,
        };

        vorbis_tags.push(VorbisTag {
            key: tag_key_and_value.0.to_string(),
            spacer: " ".to_string(),
            value: tag_key_and_value.1.to_string(),
        });
    }

    set_tag_spacers(&mut vorbis_tags);

    Ok(vorbis_tags)
}

fn set_tag_spacers(tags: &mut Vec<VorbisTag>) {
    let longest_key = match tags.iter().max_by_key(|tag| tag.key.len()) {
        Some(tag) => tag.key.len(),
        None => return,
    };

    for tag in tags {
        if longest_key > tag.key.len() {
            tag.spacer = " ".repeat(longest_key - tag.key.len());
        } else {
            tag.spacer = String::new();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_set_tag_spacers() {
        let mut test_tags: Vec<VorbisTag> = vec![
            VorbisTag {
                key: "k".to_string(),
                spacer: "".to_string(),
                value: "none".to_string(),
            },
            VorbisTag {
                key: "ke".to_string(),
                spacer: "".to_string(),
                value: "none".to_string(),
            },
            VorbisTag {
                key: "key".to_string(),
                spacer: "".to_string(),
                value: "none".to_string(),
            },
            VorbisTag {
                key: "keyvaluepair".to_string(),
                spacer: "".to_string(),
                value: "none".to_string(),
            },
        ];
        let correct_tags: Vec<VorbisTag> = vec![
            VorbisTag {
                key: "k".to_string(),
                spacer: "           ".to_string(),
                value: "none".to_string(),
            },
            VorbisTag {
                key: "ke".to_string(),
                spacer: "          ".to_string(),
                value: "none".to_string(),
            },
            VorbisTag {
                key: "key".to_string(),
                spacer: "         ".to_string(),
                value: "none".to_string(),
            },
            VorbisTag {
                key: "keyvaluepair".to_string(),
                spacer: "".to_string(),
                value: "none".to_string(),
            },
        ];
        set_tag_spacers(&mut test_tags);
        assert_eq!(test_tags, correct_tags);
    }
}
