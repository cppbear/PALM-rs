// Answer 0

#[test]
fn test_writer_new_with_vector() {
    let buffer = Vec::new();
    let writer = new(buffer);
    assert_eq!(writer.buf.len(), 0);
}

#[test]
fn test_writer_new_with_array() {
    let buffer = [0u8; 10];
    let writer = new(buffer);
    assert_eq!(writer.buf.len(), 10);
}

#[test]
fn test_writer_new_with_string() {
    let buffer = String::new();
    let writer = new(buffer);
    assert_eq!(writer.buf.len(), 0);
}

