// Answer 0

#[derive(Debug)]
struct MyStruct {
    tag: String,
    content: String,
}

impl MyStruct {
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{:?} or {:?}", self.tag, self.content)
    }
}

#[test]
fn test_expectation_valid() {
    let my_struct = MyStruct {
        tag: String::from("test_tag"),
        content: String::from("test_content"),
    };
    let mut output = String::new();
    let result = my_struct.expecting(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "\"test_tag\" or \"test_content\"");
}

#[test]
fn test_expectation_empty_tag() {
    let my_struct = MyStruct {
        tag: String::new(),
        content: String::from("non_empty_content"),
    };
    let mut output = String::new();
    let result = my_struct.expecting(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "\"\" or \"non_empty_content\"");
}

#[test]
fn test_expectation_empty_content() {
    let my_struct = MyStruct {
        tag: String::from("non_empty_tag"),
        content: String::new(),
    };
    let mut output = String::new();
    let result = my_struct.expecting(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "\"non_empty_tag\" or \"\"");
}

#[test]
fn test_expectation_both_empty() {
    let my_struct = MyStruct {
        tag: String::new(),
        content: String::new(),
    };
    let mut output = String::new();
    let result = my_struct.expecting(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "\"\" or \"\"");
}

