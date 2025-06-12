// Answer 0

#[derive(Debug)]
struct MockExpected;

impl MockExpected {
    fn new() -> Self {
        MockExpected
    }
}

#[test]
fn test_invalid_type() {
    let unexpected_value = "unexpected_value";
    let expected = MockExpected::new();
    
    let error = invalid_type(unexpected_value, &expected);
    
    assert_eq!(error.to_string(), format!("Invalid type: {:?}", unexpected_value));
}

