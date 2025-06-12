// Answer 0

#[test]
fn test_parse_escape_newline() {
    use std::io::{Cursor, Read};
    use std::result::Result;

    let input = b"\\n";
    let mut scratch = Vec::new();
    let mut reader = Cursor::new(input);

    let result: Result<(), _> = parse_escape(&mut reader, true, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\n']);
}

#[test]
fn test_parse_escape_double_quote() {
    use std::io::{Cursor, Read};
    use std::result::Result;

    let input = b"\\\"";
    let mut scratch = Vec::new();
    let mut reader = Cursor::new(input);

    let result: Result<(), _> = parse_escape(&mut reader, true, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'"']);
}

#[test]
fn test_parse_escape_backslash() {
    use std::io::{Cursor, Read};
    use std::result::Result;

    let input = b"\\\\"; // Input to test backslash escape
    let mut scratch = Vec::new();
    let mut reader = Cursor::new(input);

    let result: Result<(), _> = parse_escape(&mut reader, true, &mut scratch);

    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\\']);
} 

#[test]
fn test_parse_escape_forward_slash() {
    use std::io::{Cursor, Read};
    use std::result::Result;

    let input = b"\\/";
    let mut scratch = Vec::new();
    let mut reader = Cursor::new(input);

    let result: Result<(), _> = parse_escape(&mut reader, true, &mut scratch);

    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'/']);
}

#[test]
fn test_parse_escape_backspace() {
    use std::io::{Cursor, Read};
    use std::result::Result;

    let input = b"\\b";
    let mut scratch = Vec::new();
    let mut reader = Cursor::new(input);

    let result: Result<(), _> = parse_escape(&mut reader, true, &mut scratch);

    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\x08']);
}

#[test]
fn test_parse_escape_formfeed() {
    use std::io::{Cursor, Read};
    use std::result::Result;

    let input = b"\\f";
    let mut scratch = Vec::new();
    let mut reader = Cursor::new(input);

    let result: Result<(), _> = parse_escape(&mut reader, true, &mut scratch);

    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\x0c']);
}

#[test]
fn test_parse_escape_carriage_return() {
    use std::io::{Cursor, Read};
    use std::result::Result;

    let input = b"\\r";
    let mut scratch = Vec::new();
    let mut reader = Cursor::new(input);

    let result: Result<(), _> = parse_escape(&mut reader, true, &mut scratch);

    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\r']);
}

#[test]
fn test_parse_escape_tab() {
    use std::io::{Cursor, Read};
    use std::result::Result;

    let input = b"\\t";
    let mut scratch = Vec::new();
    let mut reader = Cursor::new(input);

    let result: Result<(), _> = parse_escape(&mut reader, true, &mut scratch);

    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\t']);
}

#[test]
#[should_panic]
fn test_parse_escape_invalid_escape() {
    use std::io::{Cursor, Read};

    let input = b"\\x"; // Invalid escape sequence
    let mut scratch = Vec::new();
    let mut reader = Cursor::new(input);

    let _ = parse_escape(&mut reader, true, &mut scratch);
}

