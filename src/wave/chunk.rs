use crate::shared::extra::ExtraChunks;
use crate::shared::id3::ID3Fields;
use crate::shared::text::TextFields;
use crate::template::Template;
use crate::wave::acid::AcidFields;
use crate::wave::bext::BextFields;
use crate::wave::cart::CartFields;
use crate::wave::cue::CueFields;
use crate::wave::fact::FactFields;
use crate::wave::fmt::FmtFields;
use crate::wave::list::ListFields;
use crate::wave::resu::ResuFields;
use crate::wave::smpl::SmplFields;
use crate::wave::sndm::SndmFields;
use crate::wave::umid::UMIDFields;
use crate::wave::xmp::XMPFields;
use std::error::Error;

const ACID_CHUNK_ID: &str = "acid";
const AXML_CHUNK_ID: &str = "axml";
const AXML_TEMPLATE_TITLE: &str = "AXML Chunk (XML):";
const BEXT_CHUNK_ID: &str = "bext";
const CART_CHUNK_ID: &str = "cart";
const CUE_CHUNK_ID: &str = "cue ";
const DATA_CHUNK_ID: &str = "data";
const DISP_CHUNK_ID: &str = "disp";
const FACT_CHUNK_ID: &str = "fact";
const FMT_CHUNK_ID: &str = "fmt ";
const ID3_CHUNK_ID: &str = "id3 ";
const IXML_CHUNK_ID: &str = "ixml";
const IXML_TEMPLATE_TITLE: &str = "iXML Chunk (XML):";
const JUNK_CHUNK_ID: &str = "junk";
const JUNK_TEMPLATE_TITLE: &str = "Junk Chunk Details";
const LIST_CHUNK_ID: &str = "list";
const LOGIC_PRO_CHUNK_ID: &str = "lgwv";
const PAD_CHUNK_ID: &str = "pad ";
const PAD_TEMPLATE_TITLE: &str = "PAD Chunk Details";
const PRO_TOOLS_DGDA_CHUNK_ID: &str = "dgda";
const PRO_TOOLS_ELM1_CHUNK_ID: &str = "elm1";
const PRO_TOOLS_MINF_CHUNK_ID: &str = "minf";
const PRO_TOOLS_REGN_CHUNK_ID: &str = "regn";
const PRO_TOOLS_UMID_CHUNK_ID: &str = "umid";
const RESU_CHUNK_ID: &str = "resu";
const SMPL_CHUNK_ID: &str = "smpl";
const SNDM_CHUNK_ID: &str = "sndm";
const XMP_CHUNK_ID: &str = "_pmx";

const NUMBER_OF_CHUNKS_TO_SKIP: usize = 7;
const CHUNKS_TO_SKIP: [&str; NUMBER_OF_CHUNKS_TO_SKIP] = [
    DATA_CHUNK_ID,
    DISP_CHUNK_ID,
    LOGIC_PRO_CHUNK_ID,
    PRO_TOOLS_DGDA_CHUNK_ID,
    PRO_TOOLS_MINF_CHUNK_ID,
    PRO_TOOLS_ELM1_CHUNK_ID,
    PRO_TOOLS_REGN_CHUNK_ID,
];

#[derive(Default)]
pub struct Chunk {
    path_to_file: String,
    pub ignore_data_for_chunks: [&'static str; NUMBER_OF_CHUNKS_TO_SKIP],
    pub found_chunk_ids: Vec<String>,
    pub skipped_chunk_ids: Vec<String>,
    extra_chunks: ExtraChunks,
    acid_data: AcidFields,
    axml_data: TextFields,
    bext_data: BextFields,
    cart_data: CartFields,
    cue_data: CueFields,
    fact_data: FactFields,
    format_data: FmtFields,
    id3_data: ID3Fields,
    ixml_data: TextFields,
    junk_data: TextFields,
    list_data: Vec<ListFields>,
    pad_data: TextFields,
    resu_data: ResuFields,
    smpl_data: SmplFields,
    sndm_data: SndmFields,
    umid_data: UMIDFields,
    xmp_data: XMPFields,
}

impl Chunk {
    pub fn new(path_to_file: String) -> Self {
        Self {
            path_to_file,
            ignore_data_for_chunks: CHUNKS_TO_SKIP,
            extra_chunks: ExtraChunks::new(),
            ..Default::default()
        }
    }

    pub fn add_chunk(&mut self, chunk_id: String, chunk_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let found_chunk_id = chunk_id.clone();

        match chunk_id.as_str() {
            FMT_CHUNK_ID => self.format_data = FmtFields::new(chunk_data)?,
            FACT_CHUNK_ID => self.fact_data = FactFields::new(chunk_data)?,
            DATA_CHUNK_ID => {}
            JUNK_CHUNK_ID => self.junk_data = TextFields::new(chunk_data)?,
            CUE_CHUNK_ID => self.cue_data = CueFields::new(chunk_data)?,
            RESU_CHUNK_ID => self.resu_data = ResuFields::new(chunk_data)?,
            LIST_CHUNK_ID => self.list_data.push(ListFields::new(chunk_data)?),
            IXML_CHUNK_ID => self.ixml_data = TextFields::new(chunk_data)?,
            XMP_CHUNK_ID => self.xmp_data = XMPFields::new(chunk_data)?,
            ID3_CHUNK_ID => self.id3_data = ID3Fields::new(self.path_to_file.clone())?,
            BEXT_CHUNK_ID => self.bext_data = BextFields::new(chunk_data)?,
            CART_CHUNK_ID => self.cart_data = CartFields::new(chunk_data)?,
            ACID_CHUNK_ID => self.acid_data = AcidFields::new(chunk_data)?,
            SMPL_CHUNK_ID => self.smpl_data = SmplFields::new(chunk_data)?,
            SNDM_CHUNK_ID => self.sndm_data = SndmFields::new(chunk_data)?,
            AXML_CHUNK_ID => self.axml_data = TextFields::new(chunk_data)?,
            PAD_CHUNK_ID => self.pad_data = TextFields::new(chunk_data)?,
            PRO_TOOLS_UMID_CHUNK_ID => self.umid_data = UMIDFields::new(chunk_data)?,
            PRO_TOOLS_ELM1_CHUNK_ID => {}
            PRO_TOOLS_MINF_CHUNK_ID => {}
            PRO_TOOLS_DGDA_CHUNK_ID => {}
            PRO_TOOLS_REGN_CHUNK_ID => {}
            DISP_CHUNK_ID => {}
            LOGIC_PRO_CHUNK_ID => {}
            _ => self.extra_chunks.add_chunk(&chunk_id, chunk_data)?,
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
                FACT_CHUNK_ID => self.fact_data.format_data_for_output(template)?,
                FMT_CHUNK_ID => self.format_data.format_data_for_output(template)?,
                BEXT_CHUNK_ID => self.bext_data.format_data_for_output(template)?,
                ID3_CHUNK_ID => self.id3_data.format_data_for_output(template)?,
                CUE_CHUNK_ID => self.cue_data.format_data_for_output(template)?,
                JUNK_CHUNK_ID => self.junk_data.format_data_for_output(template, JUNK_TEMPLATE_TITLE)?,
                ACID_CHUNK_ID => self.acid_data.format_data_for_output(template)?,
                XMP_CHUNK_ID => self.xmp_data.format_data_for_output(template)?,
                IXML_CHUNK_ID => self.ixml_data.format_data_for_output(template, IXML_TEMPLATE_TITLE)?,
                RESU_CHUNK_ID => self.resu_data.format_data_for_output(template)?,
                CART_CHUNK_ID => self.cart_data.format_data_for_output(template)?,
                SMPL_CHUNK_ID => self.smpl_data.format_data_for_output(template)?,
                PRO_TOOLS_UMID_CHUNK_ID => self.umid_data.format_data_for_output(template)?,
                SNDM_CHUNK_ID => self.sndm_data.format_data_for_output(template)?,
                AXML_CHUNK_ID => self.axml_data.format_data_for_output(template, AXML_TEMPLATE_TITLE)?,
                PAD_CHUNK_ID => self.pad_data.format_data_for_output(template, PAD_TEMPLATE_TITLE)?,
                LIST_CHUNK_ID => {
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
