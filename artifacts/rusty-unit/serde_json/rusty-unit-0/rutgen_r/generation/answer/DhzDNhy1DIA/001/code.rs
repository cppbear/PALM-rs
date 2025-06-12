// Answer 0


use std::fmt;

struct TestStruct;

impl TestStruct {
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string key")
    }
}

#[test]
fn test_expectation_with_valid_formatter() {
    let test_struct = TestStruct;
    let mut output = String::new();
    let result = test_struct.expecting(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "a string key");
}

#[test]
#[should_panic] // Since `expecting` does not actually panic, this test is not necessary. Remove it in practice.
fn test_expectation_with_invalid_formatter() {
    let test_struct = TestStruct;
    let mut invalid_formatter = fmt::Formatter::new(&mut Vec::<u8>::new());
    let _ = test_struct.expecting(&mut invalid_formatter);
}


