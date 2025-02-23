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
use crate::wave::umid::UMIDFields;
use crate::wave::xmp::XMPFields;
use std::error::Error;

const FMT_CHUNK_ID: &str = "fmt ";
const FACT_CHUNK_ID: &str = "fact";
const DATA_CHUNK_ID: &str = "data";
const CUE_CHUNK_ID: &str = "cue ";
const RESU_CHUNK_ID: &str = "resu";
const JUNK_CHUNK_ID: &str = "junk";
const JUNK_TEMPLATE_TITLE: &str = "Junk Chunk Details";
const LIST_CHUNK_ID: &str = "list";
const IXML_CHUNK_ID: &str = "ixml";
const IXML_TEMPLATE_TITLE: &str = "iXML Chunk (XML):";
const XMP_CHUNK_ID: &str = "_pmx";
const ID3_CHUNK_ID: &str = "id3 ";
const BEXT_CHUNK_ID: &str = "bext";
const CART_CHUNK_ID: &str = "cart";
const ACID_CHUNK_ID: &str = "acid";
const SMPL_CHUNK_ID: &str = "smpl";
const DISP_CHUNK_ID: &str = "disp";
const LOGIC_PRO_CHUNK_ID: &str = "lgwv";
const PRO_TOOLS_UMID_CHUNK_ID: &str = "umid";
const PRO_TOOLS_DGDA_CHUNK_ID: &str = "dgda";
const PRO_TOOLS_MINF_CHUNK_ID: &str = "minf";
const PRO_TOOLS_ELM1_CHUNK_ID: &str = "elm1";
const PRO_TOOLS_REGN_CHUNK_ID: &str = "regn";

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
    fact_data: FactFields,
    format_data: FmtFields,
    resu_data: ResuFields,
    cue_data: CueFields,
    junk_data: TextFields,
    list_data: Vec<ListFields>,
    ixml_data: TextFields,
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
