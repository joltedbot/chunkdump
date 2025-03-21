mod acid;
mod bext;
mod cart;
pub mod comm;
pub mod comt;
mod cue;
pub mod extra;
mod fact;
mod fmt;
pub mod fver;
pub mod id3;
mod list;
pub mod mark;
mod resu;
mod skipped;
mod smpl;
mod sndm;
mod text;
mod umid;

use crate::byte_arrays::Endian;
use crate::fileio::{
    read_bytes_from_file, read_chunk_id_from_file, read_chunk_size_from_file,
    skip_over_bytes_in_file,
};
use crate::output::OutputEntry;
use std::error::Error;
use std::fs::File;

pub const CHUNK_ID_FIELD_LENGTH_IN_BYTES: usize = 4;
pub const CHUNK_SIZE_FIELD_LENGTH_IN_BYTES: usize = 4;

const ACID_CHUNK_ID: &str = "acid";
const AXML_CHUNK_ID: &str = "axml";
const AXML_TEMPLATE_TITLE: &str = "AXML (XML)";
const BEXT_CHUNK_ID: &str = "bext";
const CART_CHUNK_ID: &str = "cart";
const CUE_CHUNK_ID: &str = "cue ";
const DATA_CHUNK_ID: &str = "data";
const DISP_CHUNK_ID: &str = "disp";
const FACT_CHUNK_ID: &str = "fact";
const FMT_CHUNK_ID: &str = "fmt ";
pub const ID3_CHUNK_ID: &str = "id3 ";
const IXML_CHUNK_ID: &str = "ixml";
const IXML_TEMPLATE_TITLE: &str = "iXML";
const JUNK_CHUNK_ID: &str = "junk";
const JUNK_TEMPLATE_TITLE: &str = "Junk";
const LIST_CHUNK_ID: &str = "list";
const LOGIC_PRO_CHUNK_ID: &str = "lgwv";
const PAD_CHUNK_ID: &str = "pad ";
const PAD_TEMPLATE_TITLE: &str = "PAD";
const PRO_TOOLS_DGDA_CHUNK_ID: &str = "dgda";
const PRO_TOOLS_ELM1_CHUNK_ID: &str = "elm1";
const PRO_TOOLS_MINF_CHUNK_ID: &str = "minf";
const PRO_TOOLS_REGN_CHUNK_ID: &str = "regn";
const PRO_TOOLS_UMID_CHUNK_ID: &str = "umid";
const RESU_CHUNK_ID: &str = "resu";
const SMPL_CHUNK_ID: &str = "smpl";
const SNDM_CHUNK_ID: &str = "sndm";
const XMP_CHUNK_ID: &str = "_pmx";
const XMP_TEMPLATE_TITLE: &str = "XMP (XML)";
const ANNOTATION_CHUNK_ID: &str = "anno";
const ANNOTATION_TEMPLATE_TITLE: &str = "Annotation (ANNO)";
const APPLICATION_CHUNK_ID: &str = "appl";
const APPLICATION_TEMPLATE_TITLE: &str = "Application (APPL)";
const AUTHOR_CHUNK_ID: &str = "auth";
const AUTHOR_TEMPLATE_TITLE: &str = "Author (auth)";
const COPYRIGHT_CHUNK_ID: &str = "(c) ";
const COPYRIGHT_TEMPLATE_TITLE: &str = "Copyright ((c) )";
const CHAN_CHUNK_ID: &str = "chan";
const COMMON_CHUNK_ID: &str = "comm";
const COMMENT_CHUNK_ID: &str = "comt";
const FORMAT_VERSION_CHUNK_ID: &str = "fver";
const MARKER_CHUNK_ID: &str = "mark";
pub const AUDIO_SAMPLES_CHUNK_ID: &str = "ssnd";
pub const NAME_CHUNK_ID: &str = "name";
const NAME_TEMPLATE_TITLE: &str = "Name";
pub const CHUNKS_NOT_TO_EXTRACT_DATA_FROM: [&str; 10] = [
    DATA_CHUNK_ID,
    AUDIO_SAMPLES_CHUNK_ID,
    CHAN_CHUNK_ID,
    PRO_TOOLS_ELM1_CHUNK_ID,
    PRO_TOOLS_MINF_CHUNK_ID,
    PRO_TOOLS_DGDA_CHUNK_ID,
    PRO_TOOLS_REGN_CHUNK_ID,
    DISP_CHUNK_ID,
    LOGIC_PRO_CHUNK_ID,
    ID3_CHUNK_ID,
];
pub const ERROR_TO_MATCH_IF_NOT_ENOUGH_BYTES_LEFT_IN_FILE: &str = "failed to fill whole buffer";

pub fn get_metadata_from_chunks(
    input_file: &mut File,
    file_path: &str,
    endianness: Endian,
) -> Result<Vec<OutputEntry>, Box<dyn Error>> {
    let mut output: Vec<OutputEntry> = vec![];

    loop {
        let chunk_id: String = match read_chunk_id_from_file(input_file) {
            Ok(chunk_id) => chunk_id.to_lowercase(),
            Err(error) if error.to_string() == ERROR_TO_MATCH_IF_NOT_ENOUGH_BYTES_LEFT_IN_FILE => {
                break
            }
            Err(error) => return Err(error),
        };

        let chunk_size = match chunk_id.as_str() {
            ID3_CHUNK_ID => read_chunk_size_from_file(input_file, Endian::Little)?,
            _ => read_chunk_size_from_file(input_file, endianness.to_owned())?,
        };

        let mut chunk_data: Vec<u8> = Vec::new();
        if CHUNKS_NOT_TO_EXTRACT_DATA_FROM.contains(&chunk_id.as_str()) {
            skip_over_bytes_in_file(input_file, chunk_size)?;
        } else {
            chunk_data = read_bytes_from_file(input_file, chunk_size).unwrap_or_default();
        }

        output.push(get_chunk_metadata(chunk_id, chunk_data, file_path)?);
    }

    Ok(output)
}

pub fn get_chunk_metadata(
    chunk_id: String,
    chunk_data: Vec<u8>,
    file_path: &str,
) -> Result<OutputEntry, Box<dyn Error>> {
    let result = match chunk_id.as_str() {
        FMT_CHUNK_ID => fmt::get_metadata(chunk_data)?,
        FACT_CHUNK_ID => fact::get_metadata(chunk_data)?,
        COMMON_CHUNK_ID => comm::get_metadata(chunk_data)?,
        BEXT_CHUNK_ID => bext::get_metadata(chunk_data)?,
        CART_CHUNK_ID => cart::get_metadata(chunk_data)?,
        CUE_CHUNK_ID => cue::get_metadata(chunk_data)?,
        COMMENT_CHUNK_ID => comt::get_metadata(chunk_data)?,
        FORMAT_VERSION_CHUNK_ID => fver::get_metadata(chunk_data)?,
        MARKER_CHUNK_ID => mark::get_metadata(chunk_data)?,
        ACID_CHUNK_ID => acid::get_metadata(chunk_data)?,
        JUNK_CHUNK_ID => text::get_metadata(JUNK_TEMPLATE_TITLE, chunk_data)?,
        PAD_CHUNK_ID => text::get_metadata(PAD_TEMPLATE_TITLE, chunk_data)?,
        LIST_CHUNK_ID => list::get_metadata(chunk_data)?,
        ID3_CHUNK_ID => id3::get_metadata(file_path)?,
        IXML_CHUNK_ID => text::get_metadata(IXML_TEMPLATE_TITLE, chunk_data)?,
        XMP_CHUNK_ID => text::get_metadata(XMP_TEMPLATE_TITLE, chunk_data)?,
        AXML_CHUNK_ID => text::get_metadata(AXML_TEMPLATE_TITLE, chunk_data)?,
        RESU_CHUNK_ID => resu::get_metadata(chunk_data)?,
        SMPL_CHUNK_ID => smpl::get_metadata(chunk_data)?,
        SNDM_CHUNK_ID => sndm::get_metadata(chunk_data)?,
        APPLICATION_CHUNK_ID => text::get_metadata(APPLICATION_TEMPLATE_TITLE, chunk_data)?,
        ANNOTATION_CHUNK_ID => text::get_metadata(ANNOTATION_TEMPLATE_TITLE, chunk_data)?,
        AUTHOR_CHUNK_ID => text::get_metadata(AUTHOR_TEMPLATE_TITLE, chunk_data)?,
        NAME_CHUNK_ID => text::get_metadata(NAME_TEMPLATE_TITLE, chunk_data)?,
        COPYRIGHT_CHUNK_ID => text::get_metadata(COPYRIGHT_TEMPLATE_TITLE, chunk_data)?,
        PRO_TOOLS_UMID_CHUNK_ID => umid::get_metadata(chunk_data)?,
        DATA_CHUNK_ID => skipped::get_metadata(chunk_id)?,
        AUDIO_SAMPLES_CHUNK_ID => skipped::get_metadata(chunk_id)?,
        CHAN_CHUNK_ID => skipped::get_metadata(chunk_id)?,
        PRO_TOOLS_ELM1_CHUNK_ID => skipped::get_metadata(chunk_id)?,
        PRO_TOOLS_MINF_CHUNK_ID => skipped::get_metadata(chunk_id)?,
        PRO_TOOLS_DGDA_CHUNK_ID => skipped::get_metadata(chunk_id)?,
        PRO_TOOLS_REGN_CHUNK_ID => skipped::get_metadata(chunk_id)?,
        DISP_CHUNK_ID => skipped::get_metadata(chunk_id)?,
        LOGIC_PRO_CHUNK_ID => skipped::get_metadata(chunk_id)?,
        _ => extra::get_metadata(chunk_id, chunk_data)?,
    };

    Ok(result)
}
