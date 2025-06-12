// Answer 0

#[test]
fn test_literal_debug_complete() {
    // Create a complete Literal without cut
    let bytes = vec![72, 101, 108, 108, 111]; // "Hello"
    let literal = Literal { v: bytes.clone(), cut: false };

    let mut output = String::new();
    {
        let result = write!(&mut output, "{:?}", literal);
        assert!(result.is_ok());
    }
    assert_eq!(output, "Complete(Hello)");
}

#[test]
fn test_literal_debug_complete_with_unicode() {
    // Create a complete Literal with unicode characters
    let bytes = vec![240, 159, 152, 128]; // U+1F600 (Smiling Face Emoji)
    let literal = Literal { v: bytes.clone(), cut: false };

    let mut output = String::new();
    {
        let result = write!(&mut output, "{:?}", literal);
        assert!(result.is_ok());
    }
    assert_eq!(output, "Complete(\u{1f600})");
}

#[test]
fn test_literal_debug_complete_with_whitespace() {
    // Create a complete Literal with whitespace characters
    let bytes = vec![32, 9, 10]; // Space, Tab, Newline
    let literal = Literal { v: bytes.clone(), cut: false };

    let mut output = String::new();
    {
        let result = write!(&mut output, "{:?}", literal);
        assert!(result.is_ok());
    }
    assert_eq!(output, "Complete(\\ \\t\\n)");
}

