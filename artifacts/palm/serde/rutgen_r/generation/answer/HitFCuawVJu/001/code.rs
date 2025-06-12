// Answer 0

#[derive(Debug)]
struct TestStruct {
    expecting: &'static str,
}

impl TestStruct {
    fn new(expectation: &'static str) -> Self {
        TestStruct { expecting: expectation }
    }
}

#[test]
fn test_expecting_with_simple_string() {
    let test_instance = TestStruct::new("A simple expectation.");
    let mut output = std::fmt::Formatter::default();
    assert!(test_instance.expecting(&mut output).is_ok());
}

#[test]
fn test_expecting_with_empty_string() {
    let test_instance = TestStruct::new("");
    let mut output = std::fmt::Formatter::default();
    assert!(test_instance.expecting(&mut output).is_ok());
}

#[test]
fn test_expecting_with_long_string() {
    let long_expectation = "This is a very long expectation to test the bounds of the formatter. ".repeat(10);
    let test_instance = TestStruct::new(&long_expectation);
    let mut output = std::fmt::Formatter::default();
    assert!(test_instance.expecting(&mut output).is_ok());
}

#[should_panic]
#[test]
fn test_expecting_with_null_pointer() {
    let test_instance = TestStruct::new(std::ptr::null());
    let mut output = std::fmt::Formatter::default();
    let _ = test_instance.expecting(&mut output);
}

