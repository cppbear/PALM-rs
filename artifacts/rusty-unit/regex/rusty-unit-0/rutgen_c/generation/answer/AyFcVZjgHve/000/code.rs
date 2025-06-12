// Answer 0

#[test]
fn test_input_at_len() {
    // Create an instance of InputAt directly.
    let input_at = InputAt {
        pos: 0,
        c: Char(97), // Character 'a'
        byte: Some(97),
        len: 1,
    };

    // Test that the `len` method returns the correct length.
    assert_eq!(input_at.len(), 1);

    // Test with another instance having different length
    let input_at_multi_byte = InputAt {
        pos: 1,
        c: Char(200), // Using a multi-byte character (ex. U+00C8, 'È')
        byte: Some(200),
        len: 2,
    };

    // Expect the length to be 2 for a multi-byte character
    assert_eq!(input_at_multi_byte.len(), 2);
}

