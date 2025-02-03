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
mod smpl;
mod xmp;

use crate::errors::LocalError;
use crate::fileio::{
    canonicalize_file_path, get_file_name_from_file_path, read_bytes_from_file, read_bytes_from_file_as_string,
    skip_over_bytes_in_file,
};
use crate::template::Template;
use crate::wave::acid::AcidData;
use crate::wave::bext::BextData;
use crate::wave::cart::CartData;
use crate::wave::cue::CueFields;
use crate::wave::data::skip_data_chunk;
use crate::wave::extra::ExtraChunks;
use crate::wave::fact::FactFields;
use crate::wave::fmt::FmtFields;
use crate::wave::id3::ID3Fields;
use crate::wave::ixml::IXMLFields;
use crate::wave::junk::JunkFields;
use crate::wave::list::ListFields;
use crate::wave::resu::ResuFields;
use crate::wave::xmp::XMPFields;

use crate::wave::smpl::SmplFields;
use byte_unit::{Byte, UnitType};
use std::error::Error;
use std::fs::File;
use upon::Value;

const FMT_CHUNKID: &str = "fmt ";
const FACT_CHUNKID: &str = "fact";
const DATA_CHUNKID: &str = "data";
const CUE_CHUNKID: &str = "cue ";
const RESU_CHUNKID: &str = "resu";
const JUNK_CHUNKID: &str = "junk";
const LIST_CHUNKID: &str = "list";
const IXML_CHUNKID: &str = "ixml";
const XMP_CHUNKID: &str = "_pmx";
const ID3_CHUNKID: &str = "id3 ";
const BEXT_CHUNKID: &str = "bext";
const CART_CHUNKID: &str = "cart";
const ACID_CHUNKID: &str = "acid";
const SMPL_CHUNKID: &str = "smpl";

pub const ACID_TEMPLATE_NAME: &str = "acid";
pub const BEXT_TEMPLATE_NAME: &str = "bext";
pub const CART_TEMPLATE_NAME: &str = "cart";
pub const CUE_TEMPLATE_NAME: &str = "cue";
pub const EXTRA_TEMPLATE_NAME: &str = "extra";
pub const FACT_TEMPLATE_NAME: &str = "fact";
pub const FMT_TEMPLATE_NAME: &str = "fmt";
pub const ID3_TEMPLATE_NAME: &str = "id3";
pub const IXML_TEMPLATE_NAME: &str = "ixml";
pub const JUNK_TEMPLATE_NAME: &str = "junk";
pub const LIST_TEMPLATE_INFO_NAME: &str = "list_info";
pub const LIST_TEMPLATE_ADTL_NAME: &str = "adtl_info";
pub const RESU_TEMPLATE_NAME: &str = "resu";
pub const WAVE_TEMPLATE_NAME: &str = "wave";
pub const XMP_TEMPLATE_NAME: &str = "xmp";
pub const SMPL_TEMPLATE_NAME: &str = "smpl";

const WAVEID_FIELD_LENGTH_IN_BYTES: usize = 4;
const CHUNKID_FIELD_LENGTH_IN_BYTES: usize = 4;
const RIFF_CKSIZE_FIELD_LENGTH_IN_BYTES: i64 = 4;
const WAVEID_IN_DECIMAL_LITTLE_ENDIAN_BYTES: [u8; 4] = [87, 65, 86, 69];

#[derive(Debug, Clone, Default)]
pub struct Wave {
    pub original_file_path: String,
    pub name: String,
    pub chunk_ids: Vec<String>,
    pub canonical_path: String,
    pub size_in_bytes: u64,
    pub fact_data: FactFields,
    pub format_data: FmtFields,
    pub resu_data: ResuFields,
    pub cue_data: CueFields,
    pub junk_data: JunkFields,
    pub list_data: Vec<ListFields>,
    pub ixml_data: IXMLFields,
    pub xmp_data: XMPFields,
    pub id3_data: ID3Fields,
    pub bext_data: BextData,
    pub cart_data: CartData,
    pub acid_data: AcidData,
    pub smpl_data: SmplFields,
    pub extra_data: ExtraChunks,
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

    pub fn display_wave_file_metadata(&self, template: Template) -> Result<(), Box<dyn Error>> {
        println!("{}", self.get_metadata_outputs(&template)?);

        for chunk in self.chunk_ids.iter() {
            let chunk_fields = match chunk.as_str() {
                FACT_CHUNKID => self.fact_data.get_metadata_output(&template, FACT_TEMPLATE_NAME)?,
                FMT_CHUNKID => self.format_data.get_metadata_output(&template, FMT_TEMPLATE_NAME)?,
                BEXT_CHUNKID => self.bext_data.get_metadata_output(&template, BEXT_TEMPLATE_NAME)?,
                ID3_CHUNKID => self.id3_data.get_metadata_output(&template, ID3_TEMPLATE_NAME)?,
                CUE_CHUNKID => self.cue_data.get_metadata_output(&template, CUE_TEMPLATE_NAME)?,
                JUNK_CHUNKID => self.junk_data.get_metadata_output(&template, JUNK_TEMPLATE_NAME)?,
                ACID_CHUNKID => self.acid_data.get_metadata_output(&template, ACID_TEMPLATE_NAME)?,
                XMP_CHUNKID => self.xmp_data.get_metadata_output(&template, XMP_TEMPLATE_NAME)?,
                IXML_CHUNKID => self.ixml_data.get_metadata_output(&template, IXML_TEMPLATE_NAME)?,
                RESU_CHUNKID => self.resu_data.get_metadata_output(&template, RESU_TEMPLATE_NAME)?,
                CART_CHUNKID => self.cart_data.get_metadata_output(&template, CART_TEMPLATE_NAME)?,
                SMPL_CHUNKID => self.smpl_data.get_metadata_output(&template, SMPL_TEMPLATE_NAME)?,
                LIST_CHUNKID => {
                    let mut list_metadata_output = Default::default();
                    for list_field in self.list_data.iter() {
                        list_metadata_output += list_field
                            .get_metadata_output(&template, LIST_TEMPLATE_INFO_NAME, LIST_TEMPLATE_ADTL_NAME)?
                            .as_str();
                    }
                    list_metadata_output
                }
                _ => continue,
            };

            if !chunk_fields.is_empty() {
                println!("{}", chunk_fields);
            }
        }

        println!(
            "{}",
            self.extra_data.get_metadata_outputs(&template, EXTRA_TEMPLATE_NAME)?
        );

        Ok(())
    }

    fn get_metadata_outputs(&self, template: &Template) -> Result<String, Box<dyn Error>> {
        let wave_output_values: Value = upon::value! {
            file_name: self.name.clone(),
            file_path: self.original_file_path.clone(),
            file_size: format_file_size_as_string(self.size_in_bytes),
            chunk_ids_found: self.chunk_ids.clone().join(", "),
        };

        let wave_metadata_output = template.get_wave_chunk_output(WAVE_TEMPLATE_NAME, wave_output_values)?;
        Ok(wave_metadata_output)
    }
}

fn extract_metadata(file_path: String, mut wave_file: File) -> Result<Wave, Box<dyn Error>> {
    let mut new_wave: Wave = Default::default();
    new_wave.extra_data = ExtraChunks::new();
    new_wave.original_file_path = file_path.clone();

    loop {
        let next_chunkid: String = match read_bytes_from_file_as_string(&mut wave_file, CHUNKID_FIELD_LENGTH_IN_BYTES) {
            Ok(chunkid) => chunkid.to_lowercase(),
            Err(_) => break,
        };

        if !new_wave.chunk_ids.contains(&next_chunkid) {
            new_wave.chunk_ids.push(next_chunkid.clone());
        }

        parse_chunk_ids(&mut wave_file, &mut new_wave, next_chunkid)?;
    }

    new_wave.name = get_file_name_from_file_path(&file_path)?;
    new_wave.canonical_path = canonicalize_file_path(&file_path)?;
    new_wave.size_in_bytes = wave_file.metadata()?.len();

    Ok(new_wave)
}

fn parse_chunk_ids(wave_file: &mut File, new_wave: &mut Wave, next_chunkid: String) -> Result<(), Box<dyn Error>> {
    match next_chunkid.as_str() {
        JUNK_CHUNKID => new_wave.junk_data = JunkFields::new(wave_file)?,
        FMT_CHUNKID => new_wave.format_data = FmtFields::new(wave_file)?,
        FACT_CHUNKID => new_wave.fact_data = FactFields::new(wave_file)?,
        DATA_CHUNKID => skip_data_chunk(wave_file)?,
        CUE_CHUNKID => new_wave.cue_data = CueFields::new(wave_file)?,
        RESU_CHUNKID => new_wave.resu_data = ResuFields::new(wave_file)?,
        LIST_CHUNKID => new_wave.list_data.push(ListFields::new(wave_file)?),
        IXML_CHUNKID => new_wave.ixml_data = IXMLFields::new(wave_file)?,
        XMP_CHUNKID => new_wave.xmp_data = XMPFields::new(wave_file)?,
        ID3_CHUNKID => new_wave.id3_data = ID3Fields::new(wave_file, new_wave.original_file_path.clone())?,
        BEXT_CHUNKID => new_wave.bext_data = BextData::new(wave_file)?,
        CART_CHUNKID => new_wave.cart_data = CartData::new(wave_file)?,
        ACID_CHUNKID => new_wave.acid_data = AcidData::new(wave_file)?,
        SMPL_CHUNKID => new_wave.smpl_data = SmplFields::new(wave_file)?,
        _ => new_wave.extra_data.add_chunk(wave_file, next_chunkid)?,
    }
    Ok(())
}

pub fn add_one_if_byte_size_is_odd(mut byte_size: u32) -> u32 {
    if byte_size % 2 > 0 {
        byte_size += 1;
    }

    byte_size
}

fn format_file_size_as_string(file_size_in_bytes: u64) -> String {
    Byte::from_u64(file_size_in_bytes)
        .get_appropriate_unit(UnitType::Binary)
        .to_string()
}
