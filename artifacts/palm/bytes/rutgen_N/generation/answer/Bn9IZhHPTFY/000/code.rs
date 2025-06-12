// Answer 0

#[test]
fn test_put_slice_empty() {
    let mut buffer: Vec<u8> = Vec::new();
    let input: &[u8] = b"";
    buffer.put_slice(input);
    assert_eq!(buffer, b"");
}

#[test]
fn test_put_slice_non_empty() {
    let mut buffer: Vec<u8> = Vec::new();
    let input: &[u8] = b"Hello, world!";
    buffer.put_slice(input);
    assert_eq!(buffer, b"Hello, world!");
}

#[test]
fn test_put_slice_boundary() {
    let mut buffer: Vec<u8> = Vec::new();
    let input: &[u8] = b"Rust";
    buffer.put_slice(input);
    assert_eq!(buffer.len(), 4);
    assert_eq!(buffer, b"Rust");
}

