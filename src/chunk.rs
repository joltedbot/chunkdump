mod acid;
mod bext;
mod cart;
mod cue;
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

use crate::chunk::acid::AcidFields;
use crate::chunk::bext::BextFields;
use crate::chunk::cart::CartFields;
use crate::chunk::cue::CueFields;
use crate::chunk::extra::ExtraChunk;
use crate::chunk::fact::FactFields;
use crate::chunk::fmt::FmtFields;
use crate::chunk::id3::ID3Fields;
use crate::chunk::ixml::IXMLFields;
use crate::chunk::junk::JunkFields;
use crate::chunk::list::ListFields;
use crate::chunk::resu::ResuFields;
use crate::chunk::smpl::SmplFields;
use crate::chunk::xmp::XMPFields;
use crate::template::Template;
use std::error::Error;

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
const DISP_CHUNKID: &str = "disp";
const LOGIC_PRO_CHUNKID: &str = "lgwv";

const CHUNKS_TO_SKIP: [&str; 4] = [DATA_CHUNKID, ID3_CHUNKID, DISP_CHUNKID, LOGIC_PRO_CHUNKID];

#[derive(Default)]
pub struct Chunk {
    path_to_file: String,
    pub ignore_data_for_chunks: [&'static str; 4],
    pub found_chunk_ids: Vec<String>,
    pub extra_chunks: ExtraChunk,
    pub fact_data: FactFields,
    pub format_data: FmtFields,
    pub resu_data: ResuFields,
    pub cue_data: CueFields,
    pub junk_data: JunkFields,
    pub list_data: Vec<ListFields>,
    pub ixml_data: IXMLFields,
    pub xmp_data: XMPFields,
    pub id3_data: ID3Fields,
    pub bext_data: BextFields,
    pub cart_data: CartFields,
    pub acid_data: AcidFields,
    pub smpl_data: SmplFields,
}

impl Chunk {
    pub fn new(path_to_file: String) -> Self {
        Chunk {
            path_to_file,
            ignore_data_for_chunks: CHUNKS_TO_SKIP,
            extra_chunks: ExtraChunk::new(),
            ..Default::default()
        }
    }

    pub fn add_chunk(&mut self, chunk_id: String, chunk_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let found_chunk_id = chunk_id.clone();

        match chunk_id.as_str() {
            FMT_CHUNKID => self.format_data = FmtFields::new(chunk_data)?,
            FACT_CHUNKID => self.fact_data = FactFields::new(chunk_data)?,
            DATA_CHUNKID => {}
            JUNK_CHUNKID => self.junk_data = JunkFields::new(chunk_data)?,
            CUE_CHUNKID => self.cue_data = CueFields::new(chunk_data)?,
            RESU_CHUNKID => self.resu_data = ResuFields::new(chunk_data)?,
            LIST_CHUNKID => self.list_data.push(ListFields::new(chunk_data)?),
            IXML_CHUNKID => self.ixml_data = IXMLFields::new(chunk_data)?,
            XMP_CHUNKID => self.xmp_data = XMPFields::new(chunk_data)?,
            ID3_CHUNKID => self.id3_data = ID3Fields::new(self.path_to_file.clone())?,
            BEXT_CHUNKID => self.bext_data = BextFields::new(chunk_data)?,
            CART_CHUNKID => self.cart_data = CartFields::new(chunk_data)?,
            ACID_CHUNKID => self.acid_data = AcidFields::new(chunk_data)?,
            SMPL_CHUNKID => self.smpl_data = SmplFields::new(chunk_data)?,
            DISP_CHUNKID => {}
            LOGIC_PRO_CHUNKID => {}
            _ => self.extra_chunks.add_chunk(chunk_id, chunk_data)?,
        }

        if !self.found_chunk_ids.contains(&found_chunk_id) {
            self.found_chunk_ids.push(found_chunk_id);
        }

        Ok(())
    }

    pub fn format_data_for_output(&self, template: &mut Template) -> Result<Vec<String>, Box<dyn Error>> {
        let mut data_output_lines: Vec<String> = vec![];

        for chunk in self.found_chunk_ids.iter() {
            let chunk_fields = match chunk.as_str() {
                FACT_CHUNKID => self.fact_data.format_data_for_output(template)?,
                FMT_CHUNKID => self.format_data.format_data_for_output(template)?,
                BEXT_CHUNKID => self.bext_data.format_data_for_output(template)?,
                ID3_CHUNKID => self.id3_data.format_data_for_output(template)?,
                CUE_CHUNKID => self.cue_data.format_data_for_output(template)?,
                JUNK_CHUNKID => self.junk_data.format_data_for_output(template)?,
                ACID_CHUNKID => self.acid_data.format_data_for_output(template)?,
                XMP_CHUNKID => self.xmp_data.format_data_for_output(template)?,
                IXML_CHUNKID => self.ixml_data.format_data_for_output(template)?,
                RESU_CHUNKID => self.resu_data.format_data_for_output(template)?,
                CART_CHUNKID => self.cart_data.format_data_for_output(template)?,
                SMPL_CHUNKID => self.smpl_data.format_data_for_output(template)?,
                LIST_CHUNKID => {
                    let mut list_metadata_output = String::new();
                    for list_field in self.list_data.iter() {
                        list_metadata_output += list_field.format_data_for_output(template)?.as_str();
                    }
                    list_metadata_output
                }
                _ => continue,
            };

            if !chunk_fields.is_empty() {
                data_output_lines.push(chunk_fields);
            }
        }

        let extra_chunks_output = self.extra_chunks.format_data_for_output(template)?;

        if !extra_chunks_output.is_empty() {
            data_output_lines.push(extra_chunks_output);
        }

        Ok(data_output_lines)
    }
}
