use crate::template::Template;
use crate::wave::acid::AcidFields;
use crate::wave::bext::BextFields;
use crate::wave::cart::CartFields;
use crate::wave::cue::CueFields;
use crate::wave::extra::ExtraChunk;
use crate::wave::fact::FactFields;
use crate::wave::fmt::FmtFields;
use crate::wave::id3::ID3Fields;
use crate::wave::ixml::IXMLFields;
use crate::wave::junk::JunkFields;
use crate::wave::list::ListFields;
use crate::wave::resu::ResuFields;
use crate::wave::smpl::SmplFields;
use crate::wave::umid::UMIDFields;
use crate::wave::xmp::XMPFields;
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
const PRO_TOOLS_UMID_CHUNKID: &str = "umid";
const PRO_TOOLS_DGDA_CHUNKID: &str = "dgda";
const PRO_TOOLS_MINF_CHUNKID: &str = "minf";
const PRO_TOOLS_ELM1_CHUNKID: &str = "elm1";
const PRO_TOOLS_REGN_CHUNKID: &str = "regn";

const NUMBER_OF_CHUNKS_TO_SKIP: usize = 8;
const CHUNKS_TO_SKIP: [&str; NUMBER_OF_CHUNKS_TO_SKIP] = [
    DATA_CHUNKID,
    ID3_CHUNKID,
    DISP_CHUNKID,
    LOGIC_PRO_CHUNKID,
    PRO_TOOLS_DGDA_CHUNKID,
    PRO_TOOLS_MINF_CHUNKID,
    PRO_TOOLS_ELM1_CHUNKID,
    PRO_TOOLS_REGN_CHUNKID,
];

#[derive(Default)]
pub struct Chunk {
    path_to_file: String,
    pub ignore_data_for_chunks: [&'static str; NUMBER_OF_CHUNKS_TO_SKIP],
    pub found_chunk_ids: Vec<String>,
    pub skipped_chunk_ids: Vec<String>,
    extra_chunks: ExtraChunk,
    fact_data: FactFields,
    format_data: FmtFields,
    resu_data: ResuFields,
    cue_data: CueFields,
    junk_data: JunkFields,
    list_data: Vec<ListFields>,
    ixml_data: IXMLFields,
    xmp_data: XMPFields,
    id3_data: ID3Fields,
    bext_data: BextFields,
    cart_data: CartFields,
    acid_data: AcidFields,
    smpl_data: SmplFields,
    umid_data: UMIDFields,
}

impl Chunk {
    pub fn new(path_to_file: String) -> Self {
        Self {
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
            PRO_TOOLS_UMID_CHUNKID => self.umid_data = UMIDFields::new(chunk_data)?,
            PRO_TOOLS_ELM1_CHUNKID => {}
            PRO_TOOLS_MINF_CHUNKID => {}
            PRO_TOOLS_DGDA_CHUNKID => {}
            PRO_TOOLS_REGN_CHUNKID => {}
            DISP_CHUNKID => {}
            LOGIC_PRO_CHUNKID => {}
            _ => self.extra_chunks.add_chunk(chunk_id, chunk_data)?,
        }

        if CHUNKS_TO_SKIP.contains(&found_chunk_id.as_str()) {
            self.skipped_chunk_ids.push(found_chunk_id);
        } else if !self.found_chunk_ids.contains(&found_chunk_id) {
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
                PRO_TOOLS_UMID_CHUNKID => self.umid_data.format_data_for_output(template)?,
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
