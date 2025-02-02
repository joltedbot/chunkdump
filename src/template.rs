use crate::errors::LocalError;
use std::collections::HashMap;
use std::error::Error;
use upon::{Engine, Value};

pub struct Template {
    pub engine: Engine<'static>,
    pub wave_store: HashMap<&'static str, upon::Template<'static>>,
}

impl Template {
    pub fn new() -> Result<Self, upon::Error> {
        let engine = Engine::new();
        let wave_store = create_wave_template_store(&engine)?;
        Ok(Template { engine, wave_store })
    }

    pub fn get_wave_chunk_output(&self, template_name: &str, values: Value) -> Result<String, Box<dyn Error>> {
        let template = match self.wave_store.get(template_name) {
            Some(template) => template,
            None => return Err(Box::new(LocalError::InvalidOutputTemplate)),
        };

        match template.render(&self.engine, &values).to_string() {
            Ok(template) => Ok(template),
            Err(e) => Err(Box::new(e)),
        }
    }
}

pub fn create_wave_template_store(engine: &Engine) -> upon::Result<HashMap<&'static str, upon::Template<'static>>> {
    let mut store: HashMap<&str, upon::Template> = HashMap::<&'static str, upon::Template<'static>>::new();

    store.insert(
        crate::wave::ACID_TEMPLATE_NAME,
        engine.compile(include_str!("templates/wave/acid.tmpl"))?,
    );
    store.insert(
        crate::wave::BEXT_TEMPLATE_NAME,
        engine.compile(include_str!("templates/wave/bext.tmpl"))?,
    );
    store.insert(
        crate::wave::CART_TEMPLATE_NAME,
        engine.compile(include_str!("templates/wave/cart.tmpl"))?,
    );
    store.insert(
        crate::wave::CUE_TEMPLATE_NAME,
        engine.compile(include_str!("templates/wave/cue.tmpl"))?,
    );
    store.insert(
        crate::wave::EXTRA_TEMPLATE_NAME,
        engine.compile(include_str!("templates/wave/extra.tmpl"))?,
    );
    store.insert(
        crate::wave::FACT_TEMPLATE_NAME,
        engine.compile(include_str!("templates/wave/fact.tmpl"))?,
    );
    store.insert(
        crate::wave::FMT_TEMPLATE_NAME,
        engine.compile(include_str!("templates/wave/fmt.tmpl"))?,
    );
    store.insert(
        crate::wave::ID3_TEMPLATE_NAME,
        engine.compile(include_str!("templates/wave/id3.tmpl"))?,
    );
    store.insert(
        crate::wave::IXML_TEMPLATE_NAME,
        engine.compile(include_str!("templates/wave/ixml.tmpl"))?,
    );
    store.insert(
        crate::wave::JUNK_TEMPLATE_NAME,
        engine.compile(include_str!("templates/wave/junk.tmpl"))?,
    );
    store.insert(
        crate::wave::LIST_TEMPLATE_INFO_NAME,
        engine.compile(include_str!("templates/wave/list_info.tmpl"))?,
    );
    store.insert(
        crate::wave::LIST_TEMPLATE_ADTL_NAME,
        engine.compile(include_str!("templates/wave/list_adtl.tmpl"))?,
    );
    store.insert(
        crate::wave::RESU_TEMPLATE_NAME,
        engine.compile(include_str!("templates/wave/resu.tmpl"))?,
    );
    store.insert(
        crate::wave::WAVE_TEMPLATE_NAME,
        engine.compile(include_str!("templates/wave/wave.tmpl"))?,
    );
    store.insert(
        crate::wave::XMP_TEMPLATE_NAME,
        engine.compile(include_str!("templates/wave/xmp.tmpl"))?,
    );

    Ok(store)
}
