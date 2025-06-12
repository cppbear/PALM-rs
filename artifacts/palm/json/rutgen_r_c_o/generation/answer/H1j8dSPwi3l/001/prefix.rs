// Answer 0

#[test]
fn test_parse_escape_invalid_byte_0() {
    let input = vec![b'\\', b'\x00'];
    let mut scratch = Vec::new();
    let mut cursor = std::io::Cursor::new(input);
    let result = parse_escape(&mut cursor, false, &mut scratch);
}

#[test]
fn test_parse_escape_invalid_byte_255() {
    let input = vec![b'\\', b'\xFF'];
    let mut scratch = Vec::new();
    let mut cursor = std::io::Cursor::new(input);
    let result = parse_escape(&mut cursor, false, &mut scratch);
}

