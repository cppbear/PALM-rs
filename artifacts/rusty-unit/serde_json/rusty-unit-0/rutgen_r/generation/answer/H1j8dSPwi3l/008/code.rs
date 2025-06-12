// Answer 0

#[test]
fn test_parse_escape_backspace() {
    use std::io::Cursor;

    // Initialize the input data as a sequence of bytes representing the escape character for backspace
    let input: &[u8] = b"\\b";
    let mut reader = Cursor::new(input);
    let mut scratch = Vec::new();

    // Call the function with the reader and check the result
    let result = parse_escape(&mut reader, false, &mut scratch);

    // Assert that the result is Ok and the scratch contains the correct byte
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\x08']);
}

#[test]
fn test_parse_escape_invalid_character() {
    use std::io::Cursor;

    // Initialize the input data as a sequence of bytes representing an invalid escape sequence
    let input: &[u8] = b"\\x"; // 'x' is not a valid escape character
    let mut reader = Cursor::new(input);
    let mut scratch = Vec::new();

    // Call the function and expect it to return an error
    let result = parse_escape(&mut reader, false, &mut scratch);

    // Assert that the result is an error
    assert!(result.is_err());
}

#[test]
fn test_parse_escape_valid_characters() {
    use std::io::Cursor;

    // Prepare different escape sequences to test multiple valid characters
    let test_cases = vec![b'"', b'\\', b'/', b'b', b'f', b'n', b'r', b't'];

    for &ch in &test_cases {
        let input = format!("\\{}", ch as char);
        let mut reader = Cursor::new(input.as_bytes());
        let mut scratch = Vec::new();

        // Call the function with the reader and check the result
        let result = parse_escape(&mut reader, false, &mut scratch);

        // Assert that the result is Ok and the scratch contains the correct byte for valid characters
        assert!(result.is_ok());
        let expected = match ch {
            b'"' => b'"',
            b'\\' => b'\\',
            b'/' => b'/',
            b'b' => b'\x08',
            b'f' => b'\x0c',
            b'n' => b'\n',
            b'r' => b'\r',
            b't' => b'\t',
            _ => unreachable!(),
        };
        assert_eq!(scratch, vec![expected]);
    }
}

