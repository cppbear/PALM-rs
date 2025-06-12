// Answer 0

#[test]
fn test_parse_escape_double_quote() {
    use std::io::Cursor;

    let mut scratch = Vec::new();
    let input = b"\\\""; // Testing for double quote escape
    let mut reader = Cursor::new(input);

    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'"']);
}

#[test]
fn test_parse_escape_backslash() {
    use std::io::Cursor;

    let mut scratch = Vec::new();
    let input = b"\\\\"; // Testing for backslash escape
    let mut reader = Cursor::new(input);

    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\\']);
}

#[test]
fn test_parse_escape_forward_slash() {
    use std::io::Cursor;

    let mut scratch = Vec::new();
    let input = b"\\/"; // Testing for forward slash escape
    let mut reader = Cursor::new(input);

    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'/']);
}

#[test]
fn test_parse_escape_backspace() {
    use std::io::Cursor;

    let mut scratch = Vec::new();
    let input = b"\\b"; // Testing for backspace escape
    let mut reader = Cursor::new(input);

    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\x08']);
}

#[test]
fn test_parse_escape_form_feed() {
    use std::io::Cursor;

    let mut scratch = Vec::new();
    let input = b"\\f"; // Testing for form feed escape
    let mut reader = Cursor::new(input);

    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\x0c']);
}

#[test]
fn test_parse_escape_newline() {
    use std::io::Cursor;

    let mut scratch = Vec::new();
    let input = b"\\n"; // Testing for newline escape
    let mut reader = Cursor::new(input);

    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\n']);
}

#[test]
fn test_parse_escape_carriage_return() {
    use std::io::Cursor;

    let mut scratch = Vec::new();
    let input = b"\\r"; // Testing for carriage return escape
    let mut reader = Cursor::new(input);

    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\r']);
}

#[test]
fn test_parse_escape_tab() {
    use std::io::Cursor;

    let mut scratch = Vec::new();
    let input = b"\\t"; // Testing for tab escape
    let mut reader = Cursor::new(input);

    let result = parse_escape(&mut reader, false, &mut scratch);
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\t']);
}

