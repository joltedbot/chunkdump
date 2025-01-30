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
use crate::wave::cue::CueFields;
use crate::wave::fmt::FmtFields;

use crate::wave::bext::BextData;
use crate::wave::data::skip_data_chunk;
use crate::wave::extra::{get_extra_chunks_output, read_extra_chunk_fields};
use crate::wave::fact::FactFields;
use crate::wave::id3::ID3Fields;
use crate::wave::ixml::IXMLFields;
use crate::wave::junk::JunkFields;
use crate::wave::list::ListFields;
use crate::wave::resu::ResuFields;
use crate::wave::xmp::XMPFields;

use crate::wave::acid::AcidData;
use crate::wave::cart::CartData;
use byte_unit::{AdjustedByte, Byte, UnitType};
use std::error::Error;
use std::fs::File;

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
    pub fact_data: FactFields,
    pub format_data: FmtFields,
    pub resu_data: ResuFields,
    pub cue_data: CueFields,
    pub junk_data: JunkFields,
    pub list_data: ListFields,
    pub ixml_data: IXMLFields,
    pub xmp_data: XMPFields,
    pub id3_data: ID3Fields,
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

        let new_wave: Self = extract_metadata(wave_file, file_path)?;
        Ok(new_wave)
    }

    pub fn display_wave_file_metadata(&self) -> Result<(), Box<dyn Error>> {
        println!("\n > Wave File Metadata \n");

        let wave_file_header_fields = self.get_metadata_output();
        for header_field in wave_file_header_fields {
            println!("{}", header_field);
        }

        for chunk in self.chunk_ids.iter() {
            let chunk_fields = match chunk.as_str() {
                FACT_CHUNKID => self.fact_data.get_metadata_output(),
                FMT_CHUNKID => self.format_data.get_metadata_output(),
                BEXT_CHUNKID => self.bext_data.get_metadata_output(),
                ID3_CHUNKID => self.id3_data.get_metadata_output(),
                CUE_CHUNKID => self.cue_data.get_metadata_output(),
                JUNK_CHUNKID => self.junk_data.get_metadata_output(),
                ACID_CHUNKID => self.acid_data.get_metadata_output(),
                XMP_CHUNKID => self.xmp_data.get_metadata_output(),
                IXML_CHUNKID => self.ixml_data.get_metadata_output(),
                RESU_CHUNKID => self.resu_data.get_metadata_output(),
                CART_CHUNKID => self.cart_data.get_metadata_output(),
                LIST_CHUNKID => self.list_data.get_metadata_output(),
                _ => vec![],
            };

            for field in chunk_fields {
                println!("{}", field);
            }
        }

        for extra_chunk in get_extra_chunks_output(&self.extra_data) {
            println!("{}", extra_chunk);
        }

        println!("\n");

        Ok(())
    }

    fn get_metadata_output(&self) -> Vec<String> {
        let mut header_data: Vec<String> = vec![];

        header_data.push("-------------\nFile Details:\n-------------".to_string());
        header_data.push(format!("File Name: {}", self.name.clone()));
        header_data.push(format!("File Path: {}", self.canonical_path.clone()));
        header_data.push(format!(
            "File Size: {:.2}",
            format_file_size(self.size_in_bytes.clone())
        ));
        header_data.push(format!(
            "Chunk IDs Found: [{}]",
            self.chunk_ids.clone().join(", ")
        ));

        header_data
    }
}

fn extract_metadata(mut wave_file: File, file_path: String) -> Result<Wave, Box<dyn Error>> {
    let mut new_wave: Wave = Default::default();

    loop {
        let next_chunkid: String =
            match read_bytes_from_file_as_string(&mut wave_file, CHUNKID_FIELD_LENGTH_IN_BYTES) {
                Ok(chunkid) => chunkid.to_lowercase(),
                Err(_) => break,
            };

        new_wave.chunk_ids.push(next_chunkid.clone());

        match next_chunkid.as_str() {
            JUNK_CHUNKID => new_wave.junk_data = JunkFields::new(&mut wave_file)?,
            FMT_CHUNKID => new_wave.format_data = FmtFields::new(&mut wave_file)?,
            FACT_CHUNKID => new_wave.fact_data = FactFields::new(&mut wave_file)?,
            DATA_CHUNKID => skip_data_chunk(&mut wave_file)?,
            CUE_CHUNKID => new_wave.cue_data = CueFields::new(&mut wave_file)?,
            RESU_CHUNKID => new_wave.resu_data = ResuFields::new(&mut wave_file)?,
            LIST_CHUNKID => {
                let list_result = ListFields::new(&mut wave_file)?;
                if !list_result.info_data.is_empty() {
                    new_wave.list_data.info_data = list_result.info_data;
                }

                if !list_result.adtl_data.is_empty() {
                    new_wave.list_data.adtl_data = list_result.adtl_data;
                }
            }
            IXML_CHUNKID => new_wave.ixml_data = IXMLFields::new(&mut wave_file)?,
            XMP_CHUNKID => new_wave.xmp_data = XMPFields::new(&mut wave_file)?,
            ID3_CHUNKID => new_wave.id3_data = ID3Fields::new(&mut wave_file, file_path.clone())?,
            BEXT_CHUNKID => new_wave.bext_data = BextData::new(&mut wave_file)?,
            CART_CHUNKID => new_wave.cart_data = CartData::new(&mut wave_file)?,
            ACID_CHUNKID => new_wave.acid_data = AcidData::new(&mut wave_file)?,
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

fn format_file_size(file_size_in_bytes: u64) -> AdjustedByte {
    Byte::from_u64(file_size_in_bytes).get_appropriate_unit(UnitType::Binary)
}
