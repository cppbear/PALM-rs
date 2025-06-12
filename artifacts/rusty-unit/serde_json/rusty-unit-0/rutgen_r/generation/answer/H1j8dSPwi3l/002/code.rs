// Answer 0

#[test]
fn test_parse_escape_double_quote() {
    use std::io::Cursor;

    let mut scratch = Vec::new();
    let input = b"\"";
    let mut reader = Cursor::new(input);
    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert_eq!(result.is_ok(), true);
    assert_eq!(scratch, vec![b'"']);
}

#[test]
fn test_parse_escape_backslash() {
    use std::io::Cursor;

    let mut scratch = Vec::new();
    let input = b"\\";
    let mut reader = Cursor::new(input);
    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert_eq!(result.is_ok(), true);
    assert_eq!(scratch, vec![b'\\']);
}

#[test]
fn test_parse_escape_b() {
    use std::io::Cursor;

    let mut scratch = Vec::new();
    let input = b"b";
    let mut reader = Cursor::new(input);
    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert_eq!(result.is_ok(), true);
    assert_eq!(scratch, vec![b'\x08']);
}

#[test]
fn test_parse_escape_f() {
    use std::io::Cursor;

    let mut scratch = Vec::new();
    let input = b"f";
    let mut reader = Cursor::new(input);
    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert_eq!(result.is_ok(), true);
    assert_eq!(scratch, vec![b'\x0c']);
}

#[test]
fn test_parse_escape_n() {
    use std::io::Cursor;

    let mut scratch = Vec::new();
    let input = b"n";
    let mut reader = Cursor::new(input);
    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert_eq!(result.is_ok(), true);
    assert_eq!(scratch, vec![b'\n']);
}

#[test]
fn test_parse_escape_r() {
    use std::io::Cursor;

    let mut scratch = Vec::new();
    let input = b"r";
    let mut reader = Cursor::new(input);
    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert_eq!(result.is_ok(), true);
    assert_eq!(scratch, vec![b'\r']);
}

#[test]
fn test_parse_escape_t() {
    use std::io::Cursor;

    let mut scratch = Vec::new();
    let input = b"t";
    let mut reader = Cursor::new(input);
    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert_eq!(result.is_ok(), true);
    assert_eq!(scratch, vec![b'\t']);
}

#[test]
fn test_parse_escape_forward_slash() {
    use std::io::Cursor;

    let mut scratch = Vec::new();
    let input = b"/";
    let mut reader = Cursor::new(input);
    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert_eq!(result.is_ok(), true);
    assert_eq!(scratch, vec![b'/']);
}

#[test]
#[should_panic]
fn test_parse_escape_invalid_escape() {
    use std::io::Cursor;

    let mut scratch = Vec::new();
    let input = b"a"; // Invalid escape sequence
    let mut reader = Cursor::new(input);
    let result = parse_escape(&mut reader, false, &mut scratch);
    
    assert!(result.is_err());
}

