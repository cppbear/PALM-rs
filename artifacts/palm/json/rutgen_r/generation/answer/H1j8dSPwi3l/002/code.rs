// Answer 0

#[test]
fn test_parse_escape_double_quote() {
    let mut scratch = Vec::new();
    let input = b"\\\""; // Represents the escape sequence for "
    let mut reader = std::io::Cursor::new(input);

    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'"']);
}

#[test]
fn test_parse_escape_backslash() {
    let mut scratch = Vec::new();
    let input = b"\\\\"; // Represents the escape sequence for \
    let mut reader = std::io::Cursor::new(input);

    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\\']);
}

#[test]
fn test_parse_escape_b() {
    let mut scratch = Vec::new();
    let input = b"\\b"; // Represents the escape sequence for backspace
    let mut reader = std::io::Cursor::new(input);

    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\x08']);
}

#[test]
fn test_parse_escape_f() {
    let mut scratch = Vec::new();
    let input = b"\\f"; // Represents the escape sequence for form feed
    let mut reader = std::io::Cursor::new(input);

    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\x0c']);
}

#[test]
fn test_parse_escape_n() {
    let mut scratch = Vec::new();
    let input = b"\\n"; // Represents the escape sequence for newline
    let mut reader = std::io::Cursor::new(input);

    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\n']);
}

#[test]
fn test_parse_escape_r() {
    let mut scratch = Vec::new();
    let input = b"\\r"; // Represents the escape sequence for carriage return
    let mut reader = std::io::Cursor::new(input);

    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\r']);
}

#[test]
fn test_parse_escape_t() {
    let mut scratch = Vec::new();
    let input = b"\\t"; // Represents the escape sequence for tab
    let mut reader = std::io::Cursor::new(input);

    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\t']);
}

#[test]
fn test_parse_escape_slash() {
    let mut scratch = Vec::new();
    let input = b"\\\\"; // Represents the escape sequence for /
    let mut reader = std::io::Cursor::new(input);

    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'/']);
}

#[test]
fn test_parse_escape_invalid() {
    let mut scratch = Vec::new();
    let input = b"\\x"; // Invalid escape sequence
    let mut reader = std::io::Cursor::new(input);

    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert!(result.is_err());
}

