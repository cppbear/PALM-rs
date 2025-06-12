// Answer 0

#[test]
fn test_parse_escape_double_quote() {
    use std::io::Cursor;
    use std::io::Read;

    // Create a cursor for a byte slice containing a backslash followed by a double quote
    let input = b"\""; // represents an escape sequence of \" after a backslash
    let mut reader = Cursor::new(input);
    let mut scratch = Vec::new();

    // Call the parse_escape function
    let result = parse_escape(&mut reader, true, &mut scratch);

    // Check that the result is Ok and the scratch contains the expected byte for "
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![b'"']);
}

#[test]
fn test_parse_escape_backslash() {
    use std::io::Cursor;
    use std::io::Read;

    // Create a cursor for a byte slice containing a backslash followed by another backslash
    let input = b"\\";
    let mut reader = Cursor::new(input);
    let mut scratch = Vec::new();

    // Call the parse_escape function
    let result = parse_escape(&mut reader, true, &mut scratch);

    // Check that the result is Ok and the scratch contains the expected byte for \
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![b'\\']);
}

#[test]
fn test_parse_escape_slash() {
    use std::io::Cursor;
    use std::io::Read;

    // Create a cursor for a byte slice containing a backslash followed by a slash
    let input = b"/";
    let mut reader = Cursor::new(input);
    let mut scratch = Vec::new();

    // Call the parse_escape function
    let result = parse_escape(&mut reader, true, &mut scratch);

    // Check that the result is Ok and the scratch contains the expected byte for /
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![b'/']);
} 

#[test]
fn test_parse_escape_b() {
    use std::io::Cursor;
    use std::io::Read;

    // Create a cursor for a byte slice containing a backslash followed by b
    let input = b"b";
    let mut reader = Cursor::new(input);
    let mut scratch = Vec::new();

    // Call the parse_escape function
    let result = parse_escape(&mut reader, true, &mut scratch);

    // Check that the result is Ok and the scratch contains the expected byte for \x08
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![b'\x08']);
} 

#[test]
fn test_parse_escape_f() {
    use std::io::Cursor;
    use std::io::Read;

    // Create a cursor for a byte slice containing a backslash followed by f
    let input = b"f";
    let mut reader = Cursor::new(input);
    let mut scratch = Vec::new();

    // Call the parse_escape function
    let result = parse_escape(&mut reader, true, &mut scratch);

    // Check that the result is Ok and the scratch contains the expected byte for \x0c
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![b'\x0c']);
} 

#[test]
fn test_parse_escape_n() {
    use std::io::Cursor;
    use std::io::Read;

    // Create a cursor for a byte slice containing a backslash followed by n
    let input = b"n";
    let mut reader = Cursor::new(input);
    let mut scratch = Vec::new();

    // Call the parse_escape function
    let result = parse_escape(&mut reader, true, &mut scratch);

    // Check that the result is Ok and the scratch contains the expected byte for \n
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![b'\n']);
} 

#[test]
fn test_parse_escape_r() {
    use std::io::Cursor;
    use std::io::Read;

    // Create a cursor for a byte slice containing a backslash followed by r
    let input = b"r";
    let mut reader = Cursor::new(input);
    let mut scratch = Vec::new();

    // Call the parse_escape function
    let result = parse_escape(&mut reader, true, &mut scratch);

    // Check that the result is Ok and the scratch contains the expected byte for \r
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![b'\r']);
} 

#[test]
fn test_parse_escape_t() {
    use std::io::Cursor;
    use std::io::Read;

    // Create a cursor for a byte slice containing a backslash followed by t
    let input = b"t";
    let mut reader = Cursor::new(input);
    let mut scratch = Vec::new();

    // Call the parse_escape function
    let result = parse_escape(&mut reader, true, &mut scratch);

    // Check that the result is Ok and the scratch contains the expected byte for \t
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![b'\t']);
} 

#[test]
fn test_parse_escape_invalid_character() {
    use std::io::Cursor;
    use std::io::Read;

    // Create a cursor for a byte slice containing a backslash followed by an invalid character
    let input = b"x";  // 'x' is not a valid escape
    let mut reader = Cursor::new(input);
    let mut scratch = Vec::new();

    // Call the parse_escape function and expect it to panic due to invalid escape
    let result = parse_escape(&mut reader, true, &mut scratch);

    // Check that the result is an error
    assert!(result.is_err());
}

