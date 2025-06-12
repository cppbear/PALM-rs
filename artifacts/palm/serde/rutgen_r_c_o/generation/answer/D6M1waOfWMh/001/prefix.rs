// Answer 0

#[test]
fn test_visit_borrowed_bytes_valid_utf8() {
    let visitor = StrVisitor;
    let input: &[u8] = b"hello";
    let result = visitor.visit_borrowed_bytes(input);
}

#[test]
fn test_visit_borrowed_bytes_empty() {
    let visitor = StrVisitor;
    let input: &[u8] = b"";
    let result = visitor.visit_borrowed_bytes(input);
}

#[test]
fn test_visit_borrowed_bytes_single_byte_valid() {
    let visitor = StrVisitor;
    let input: &[u8] = b"a";
    let result = visitor.visit_borrowed_bytes(input);
}

#[test]
fn test_visit_borrowed_bytes_multi_byte_valid() {
    let visitor = StrVisitor;
    let input: &[u8] = b"hello world";
    let result = visitor.visit_borrowed_bytes(input);
}

#[test]
#[should_panic]
fn test_visit_borrowed_bytes_invalid_utf8_single_byte() {
    let visitor = StrVisitor;
    let input: &[u8] = &[255]; // invalid UTF-8
    let result = visitor.visit_borrowed_bytes(input);
}

#[test]
#[should_panic]
fn test_visit_borrowed_bytes_invalid_utf8_sequence() {
    let visitor = StrVisitor;
    let input: &[u8] = &[0, 159, 146, 150]; // invalid UTF-8 sequence
    let result = visitor.visit_borrowed_bytes(input);
}

#[test]
#[should_panic]
fn test_visit_borrowed_bytes_invalid_first_byte() {
    let visitor = StrVisitor;
    let input: &[u8] = &[128]; // invalid UTF-8
    let result = visitor.visit_borrowed_bytes(input);
}

