use upon::{Engine, Value};

pub fn get_file_chunk_output(template_content: &str, values: Value) -> Result<String, upon::Error> {
    let engine = Engine::new();
    let template = engine.compile(template_content)?;
    template.render(&engine, &values).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_correct_string_from_valid_template_content_and_values() {
        let test_content = "one {{ two }} three";
        let correct_result = "one 2 three";
        let test_value = upon::value! {two: "2"};
        let result = get_file_chunk_output(test_content, test_value).unwrap();

        assert_eq!(correct_result, result);
    }
}
