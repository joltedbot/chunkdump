mod acid;
mod bext;
mod cart;
mod cue;
mod data;
mod extra;
mod fact;
mod fmt;
mod id3;
mod ixml;
mod junk;
mod list;
mod resu;
mod xmp;

use crate::errors::LocalError;
use crate::fileio::{
    canonicalize_file_path, get_file_name_from_file_path, read_bytes_from_file,
    read_bytes_from_file_as_string, skip_over_bytes_in_file,
};
use crate::wave::cue::{read_cue_chunk, CueFields};
use crate::wave::fmt::{read_fmt_chunk, FmtFields};

use crate::wave::bext::{read_bext_chunk, BextData};
use crate::wave::data::skip_data_chunk;
use crate::wave::extra::read_extra_chunk_fields;
use crate::wave::fact::read_fact_chunk;
use crate::wave::id3::read_id3_chunk;
use crate::wave::ixml::read_ixml_chunk;
use crate::wave::junk::read_junk_chunk;
use crate::wave::list::{read_list_chunk_fields, ListData};
use crate::wave::resu::read_resu_chunk;
use crate::wave::xmp::read_xmp_chunk;

use crate::wave::acid::{read_acid_chunk, AcidData};
use crate::wave::cart::{read_cart_chunk, CartData};
use std::error::Error;
use std::fs::File;

const FMT_CHUNKID: &str = "fmt ";
const FACT_CHUNKID: &str = "fact";
const DATA_CHUNKID: &str = "data";
const CUE_CHUNKID: &str = "cue ";
const RESU_CHUNKID: &str = "ResU";
const JUNK_UPPER_CHUNKID: &str = "JUNK";
const JUNK_LOWER_CHUNKID: &str = "junk";
const LIST_CHUNKID: &str = "LIST";
const IXML_CHUNKID: &str = "iXML";
const XMP_CHUNKID: &str = "_PMX";
const ID3_CHUNKID: &str = "id3 ";
const BEXT_CHUNKID: &str = "bext";
const CART_CHUNKID: &str = "cart";
const ACID_CHUNKID: &str = "acid";

const WAVEID_FIELD_LENGTH_IN_BYTES: usize = 4;
const CHUNKID_FIELD_LENGTH_IN_BYTES: usize = 4;
const RIFF_CKSIZE_FIELD_LENGTH_IN_BYTES: i64 = 4;
const WAVEID_IN_DECIMAL_LITTLE_ENDIAN_BYTES: [u8; 4] = [87, 65, 86, 69];

#[derive(Debug, Clone, Default)]
pub struct Wave {
    pub name: String,
    pub chunk_ids: Vec<String>,
    pub canonical_path: String,
    pub size_in_bytes: u64,
    pub samples_per_channel: u32,
    pub format_data: FmtFields,
    pub resu_data: String,
    pub cue_data: CueFields,
    pub junk_data: String,
    pub list_data: ListData,
    pub ixml_data: String,
    pub xmp_data: String,
    pub id3_data: Vec<(String, String)>,
    pub bext_data: BextData,
    pub cart_data: CartData,
    pub acid_data: AcidData,
    pub extra_data: Vec<(String, String)>,
}

impl Wave {
    pub fn new(file_path: String, mut wave_file: File) -> Result<Self, Box<dyn Error>> {
        skip_over_bytes_in_file(&mut wave_file, RIFF_CKSIZE_FIELD_LENGTH_IN_BYTES)?;

        let wave_id_bytes = read_bytes_from_file(&mut wave_file, WAVEID_FIELD_LENGTH_IN_BYTES)?;
        if wave_id_bytes != WAVEID_IN_DECIMAL_LITTLE_ENDIAN_BYTES {
            return Err(Box::new(LocalError::InvalidWaveID));
        }

        let new_wave: Self = extract_metadata(file_path, wave_file)?;
        Ok(new_wave)
    }
}

fn extract_metadata(file_path: String, mut wave_file: File) -> Result<Wave, Box<dyn Error>> {
    let mut new_wave: Wave = Default::default();

    loop {
        let next_chunkid: String =
            match read_bytes_from_file_as_string(&mut wave_file, CHUNKID_FIELD_LENGTH_IN_BYTES) {
                Ok(chunkid) => chunkid,
                Err(_) => break,
            };

        new_wave.chunk_ids.push(next_chunkid.clone());

        match next_chunkid.as_str() {
            JUNK_UPPER_CHUNKID => new_wave.junk_data = read_junk_chunk(&mut wave_file)?,
            JUNK_LOWER_CHUNKID => new_wave.junk_data = read_junk_chunk(&mut wave_file)?,
            FMT_CHUNKID => new_wave.format_data = read_fmt_chunk(&mut wave_file)?,
            FACT_CHUNKID => new_wave.samples_per_channel = read_fact_chunk(&mut wave_file)?,
            DATA_CHUNKID => skip_data_chunk(&mut wave_file)?,
            CUE_CHUNKID => new_wave.cue_data = read_cue_chunk(&mut wave_file)?,
            RESU_CHUNKID => new_wave.resu_data = read_resu_chunk(&mut wave_file)?,
            LIST_CHUNKID => {
                let list_result = read_list_chunk_fields(&mut wave_file)?;
                if list_result.info_data.is_empty() {
                    new_wave.list_data.info_data = list_result.info_data;
                }

                if list_result.adtl_data.is_empty() {
                    new_wave.list_data.adtl_data = list_result.adtl_data;
                }
            }
            IXML_CHUNKID => new_wave.ixml_data = read_ixml_chunk(&mut wave_file)?,
            XMP_CHUNKID => new_wave.xmp_data = read_xmp_chunk(&mut wave_file)?,
            ID3_CHUNKID => new_wave.id3_data = read_id3_chunk(&mut wave_file, file_path.clone())?,
            BEXT_CHUNKID => new_wave.bext_data = read_bext_chunk(&mut wave_file)?,
            CART_CHUNKID => new_wave.cart_data = read_cart_chunk(&mut wave_file)?,
            ACID_CHUNKID => new_wave.acid_data = read_acid_chunk(&mut wave_file)?,
            _ => new_wave
                .extra_data
                .push(read_extra_chunk_fields(&mut wave_file, next_chunkid)?),
        }
    }

    new_wave.name = get_file_name_from_file_path(&file_path)?;
    new_wave.canonical_path = canonicalize_file_path(&file_path)?;
    new_wave.size_in_bytes = wave_file.metadata()?.len();

    Ok(new_wave)
}
