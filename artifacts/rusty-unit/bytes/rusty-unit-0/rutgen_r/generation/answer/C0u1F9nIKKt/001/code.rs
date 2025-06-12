// Answer 0

#[test]
fn test_reader_new_with_vector() {
    let buffer = vec![1, 2, 3, 4, 5];
    let reader = new(buffer);
    assert_eq!(reader.buf.len(), 5);
}

#[test]
fn test_reader_new_with_slice() {
    let buffer: &[u8] = &[1, 2, 3, 4, 5];
    let reader = new(buffer);
    assert_eq!(reader.buf.len(), 5);
}

#[test]
fn test_reader_new_with_empty_vector() {
    let buffer: Vec<u8> = Vec::new();
    let reader = new(buffer);
    assert_eq!(reader.buf.len(), 0);
}

#[test]
fn test_reader_new_with_empty_slice() {
    let buffer: &[u8] = &[];
    let reader = new(buffer);
    assert_eq!(reader.buf.len(), 0);
}

