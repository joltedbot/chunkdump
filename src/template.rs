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
        template_content: &'static str,
    ) -> Result<(), upon::Error> {
        self.engine.add_template(template_name, template_content)?;
        Ok(())
    }

    pub fn get_wave_chunk_output(
        &mut self,
        template_name: &'static str,
        template_content: &'static str,
        values: Value,
    ) -> Result<String, upon::Error> {
        self.add_chunk_template(template_name, template_content)?;
        let template = self.engine.template(template_name);

        match template.render(&values).to_string() {
            Ok(template) => Ok(template),
            Err(e) => Err(e),
        }
    }
}

pub fn get_file_chunk_output(template_content: &'static str, values: Value) -> Result<String, upon::Error> {
    let engine = Engine::new();
    let template = engine.compile(template_content)?;

    match template.render(&engine, &values).to_string() {
        Ok(template) => Ok(template),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sucessfully_adds_template_into_the_engine_template_store() {
        let test_template_name: &'static str = "add_template";
        let test_template_content: &'static str = "{{ add_test_1 }}";
        let mut template = Template::new();
        assert!(template
            .add_chunk_template(test_template_name, test_template_content)
            .is_ok());
        assert!(template.engine.get_template(test_template_name).is_some());
    }

    #[test]
    fn correct_template_text_is_returned_for_a_given_upon_value() {
        let test_template_name: &'static str = "template_text2";
        let test_template_content: &'static str = "Test ID: {{ test_value }}";
        let correct_template_text: &'static str = "Test ID: test_text";
        let upon_value = upon::value! {
            test_value: "test_text",
        };
        let mut template = Template::new();

        let template_text = template
            .get_wave_chunk_output(test_template_name, test_template_content, upon_value)
            .unwrap();

        assert_eq!(template_text, correct_template_text);
    }
}
