// Answer 0

#[derive(Debug)]
struct SampleStruct {
    tag: String,
    content: String,
}

impl SampleStruct {
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "{:?}, {:?}, or other ignored fields",
            self.tag, self.content
        )
    }
}

#[test]
fn test_expectation_with_valid_data() {
    let sample = SampleStruct {
        tag: "example_tag".to_string(),
        content: "example_content".to_string(),
    };
    let mut output = String::new();
    let result = sample.expecting(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, r#" "example_tag", "example_content", or other ignored fields"#);
}

#[test]
fn test_expectation_with_empty_fields() {
    let sample = SampleStruct {
        tag: "".to_string(),
        content: "".to_string(),
    };
    let mut output = String::new();
    let result = sample.expecting(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, r#" "", "", or other ignored fields"#);
}

