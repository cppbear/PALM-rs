// Answer 0

#[test]
fn test_parse_escape_double_quote() {
    use std::io::Cursor;
    use std::io::Read;
    
    let input = b"\"";
    let mut scratch = Vec::new();
    let mut cursor = Cursor::new(input);

    let result = parse_escape(&mut cursor, true, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'"']);
}

#[test]
fn test_parse_escape_backslash() {
    use std::io::Cursor;
    use std::io::Read;
    
    let input = b"\\";
    let mut scratch = Vec::new();
    let mut cursor = Cursor::new(input);

    let result = parse_escape(&mut cursor, true, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\\']);
}

#[test]
fn test_parse_escape_forward_slash() {
    use std::io::Cursor;
    use std::io::Read;
    
    let input = b"/";
    let mut scratch = Vec::new();
    let mut cursor = Cursor::new(input);

    let result = parse_escape(&mut cursor, true, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'/']);
}

#[test]
fn test_parse_escape_backspace() {
    use std::io::Cursor;
    use std::io::Read;
    
    let input = b"b";
    let mut scratch = Vec::new();
    let mut cursor = Cursor::new(input);

    let result = parse_escape(&mut cursor, true, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\x08']);
}

#[test]
fn test_parse_escape_form_feed() {
    use std::io::Cursor;
    use std::io::Read;
    
    let input = b"f";
    let mut scratch = Vec::new();
    let mut cursor = Cursor::new(input);

    let result = parse_escape(&mut cursor, true, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\x0c']);
}

#[test]
fn test_parse_escape_newline() {
    use std::io::Cursor;
    use std::io::Read;
    
    let input = b"n";
    let mut scratch = Vec::new();
    let mut cursor = Cursor::new(input);

    let result = parse_escape(&mut cursor, true, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\n']);
}

#[test]
fn test_parse_escape_carriage_return() {
    use std::io::Cursor;
    use std::io::Read;
    
    let input = b"r";
    let mut scratch = Vec::new();
    let mut cursor = Cursor::new(input);

    let result = parse_escape(&mut cursor, true, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\r']);
}

#[test]
fn test_parse_escape_tab() {
    use std::io::Cursor;
    use std::io::Read;
    
    let input = b"t";
    let mut scratch = Vec::new();
    let mut cursor = Cursor::new(input);

    let result = parse_escape(&mut cursor, true, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![b'\t']);
}

