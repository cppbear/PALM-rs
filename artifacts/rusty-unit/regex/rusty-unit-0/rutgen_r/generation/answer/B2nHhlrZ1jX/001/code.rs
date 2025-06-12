// Answer 0

#[test]
fn test_replace_append() {
    use regex::Regex;
    use regex::Captures;

    struct TestStruct(Regex);

    let re = Regex::new(r"(?P<word>\w+)").unwrap();
    let mut test_struct = TestStruct(re);
    
    let caps = test_struct.0.captures("Hello world").unwrap();
    let mut output = String::new();
    
    // Test with a simple input that should trigger the replacement
    test_struct.replace_append(&caps, &mut output);
    assert_eq!(output, "Hello");

    output.clear();
    // Test with another input
    let caps2 = test_struct.0.captures("Rust programming").unwrap();
    test_struct.replace_append(&caps2, &mut output);
    assert_eq!(output, "Rust");

    output.clear();
    // Edge case: no match should not panic
    let caps3 = test_struct.0.captures("12345").unwrap_or_else(|_| Captures::new());
    test_struct.replace_append(&caps3, &mut output);
    assert_eq!(output, "");

    output.clear();
    // Edge case: empty string
    let caps4 = test_struct.0.captures("").unwrap_or_else(|_| Captures::new());
    test_struct.replace_append(&caps4, &mut output);
    assert_eq!(output, "");
}

