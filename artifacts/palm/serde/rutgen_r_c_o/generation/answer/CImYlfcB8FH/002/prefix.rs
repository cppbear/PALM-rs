// Answer 0

#[test]
fn test_visit_byte_buf_valid_ascii() {
    let mut output = String::new();
    let visitor = StringInPlaceVisitor(&mut output);
    let input: Vec<u8> = vec![0x61]; // 'a'
    visitor.visit_byte_buf(input).unwrap(); // Should succeed
}

#[test]
fn test_visit_byte_buf_valid_ascii_multiple() {
    let mut output = String::new();
    let visitor = StringInPlaceVisitor(&mut output);
    let input: Vec<u8> = vec![0x68, 0x65, 0x6C, 0x6C, 0x6F]; // 'hello'
    visitor.visit_byte_buf(input).unwrap(); // Should succeed
}

#[test]
fn test_visit_byte_buf_valid_unicode() {
    let mut output = String::new();
    let visitor = StringInPlaceVisitor(&mut output);
    let input: Vec<u8> = vec![0xE2, 0x9C, 0x94]; // '✓' (check mark)
    visitor.visit_byte_buf(input).unwrap(); // Should succeed
}

#[test]
fn test_visit_byte_buf_valid_unicode_multiple() {
    let mut output = String::new();
    let visitor = StringInPlaceVisitor(&mut output);
    let input: Vec<u8> = vec![0xF0, 0x9F, 0x98, 0x81]; // '😀' (grinning face)
    visitor.visit_byte_buf(input).unwrap(); // Should succeed
}

#[test]
fn test_visit_byte_buf_valid_length_boundary() {
    let mut output = String::new();
    let visitor = StringInPlaceVisitor(&mut output);
    let input: Vec<u8> = vec![0x41; 32]; // 'A' repeated 32 times
    visitor.visit_byte_buf(input).unwrap(); // Should succeed
}

#[test]
fn test_visit_byte_buf_valid_empty() {
    let mut output = String::new();
    let visitor = StringInPlaceVisitor(&mut output);
    let input: Vec<u8> = vec![]; // Empty byte vector
    visitor.visit_byte_buf(input).unwrap(); // Should succeed
}

