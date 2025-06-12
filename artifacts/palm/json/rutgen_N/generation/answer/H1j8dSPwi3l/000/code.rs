// Answer 0

#[test]
fn test_parse_escape_double_quote() {
    use std::io::Cursor;
    let mut scratch = Vec::new();
    let input = b"\"";
    let mut cursor = Cursor::new(input);

    let result = parse_escape(&mut cursor, false, &mut scratch);

    assert!(result.is_ok());
    assert_eq!(scratch, b"\"");
}

#[test]
fn test_parse_escape_backslash() {
    use std::io::Cursor;
    let mut scratch = Vec::new();
    let input = b"\\";
    let mut cursor = Cursor::new(input);

    let result = parse_escape(&mut cursor, false, &mut scratch);

    assert!(result.is_ok());
    assert_eq!(scratch, b"\\");
}

#[test]
fn test_parse_escape_forward_slash() {
    use std::io::Cursor;
    let mut scratch = Vec::new();
    let input = b"/";
    let mut cursor = Cursor::new(input);

    let result = parse_escape(&mut cursor, false, &mut scratch);

    assert!(result.is_ok());
    assert_eq!(scratch, b"/");
}

#[test]
fn test_parse_escape_backspace() {
    use std::io::Cursor;
    let mut scratch = Vec::new();
    let input = b"b";
    let mut cursor = Cursor::new(input);

    let result = parse_escape(&mut cursor, false, &mut scratch);

    assert!(result.is_ok());
    assert_eq!(scratch, b"\x08");
}

#[test]
fn test_parse_escape_form_feed() {
    use std::io::Cursor;
    let mut scratch = Vec::new();
    let input = b"f";
    let mut cursor = Cursor::new(input);

    let result = parse_escape(&mut cursor, false, &mut scratch);

    assert!(result.is_ok());
    assert_eq!(scratch, b"\x0c");
}

#[test]
fn test_parse_escape_newline() {
    use std::io::Cursor;
    let mut scratch = Vec::new();
    let input = b"n";
    let mut cursor = Cursor::new(input);

    let result = parse_escape(&mut cursor, false, &mut scratch);

    assert!(result.is_ok());
    assert_eq!(scratch, b"\n");
}

#[test]
fn test_parse_escape_carriage_return() {
    use std::io::Cursor;
    let mut scratch = Vec::new();
    let input = b"r";
    let mut cursor = Cursor::new(input);

    let result = parse_escape(&mut cursor, false, &mut scratch);

    assert!(result.is_ok());
    assert_eq!(scratch, b"\r");
}

#[test]
fn test_parse_escape_tab() {
    use std::io::Cursor;
    let mut scratch = Vec::new();
    let input = b"t";
    let mut cursor = Cursor::new(input);

    let result = parse_escape(&mut cursor, false, &mut scratch);

    assert!(result.is_ok());
    assert_eq!(scratch, b"\t");
}

#[test]
fn test_parse_escape_invalid_character() {
    use std::io::Cursor;
    let mut scratch = Vec::new();
    let input = b"x";
    let mut cursor = Cursor::new(input);

    let result = parse_escape(&mut cursor, false, &mut scratch);

    assert!(result.is_err());
}

