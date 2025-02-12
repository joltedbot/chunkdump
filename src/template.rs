use std::error::Error;
use upon::{Engine, Value};

pub struct Template {
    pub engine: Engine<'static>,
}

impl Template {
    pub fn new() -> Self {
        let engine = Engine::new();
        Template { engine }
    }

    pub fn add_chunk_template(
        &mut self,
        template_name: &'static str,
        template_path: &'static str,
    ) -> Result<(), upon::Error> {
        self.engine.add_template(template_name, template_path)?;
        Ok(())
    }

    pub fn get_wave_chunk_output(
        &mut self,
        template_name: &'static str,
        template_path: &'static str,
        values: Value,
    ) -> Result<String, Box<dyn Error>> {
        self.add_chunk_template(template_name, template_path)?;
        let template = self.engine.template(template_name);

        match template.render(&values).to_string() {
            Ok(template) => Ok(template),
            Err(e) => Err(Box::new(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sucessfully_adds_template_into_the_engine_template_store() {
        let test_template_name: &'static str = "test_template";
        let test_template_location: &'static str = "tests/testw.tmpl";
        let mut template = Template::new();
        template
            .add_chunk_template(test_template_name, test_template_location)
            .unwrap();

        assert!(template.engine.get_template(test_template_name).is_some());
    }
}
