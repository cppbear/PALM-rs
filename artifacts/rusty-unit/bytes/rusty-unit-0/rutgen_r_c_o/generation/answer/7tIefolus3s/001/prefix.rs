// Answer 0

#[test]
fn test_put_with_remaining() {
    let mut buffer: Vec<u8> = Vec::with_capacity(10);
    let src: Vec<u8> = vec![1, 2, 3];
    buffer.put(src);
}

#[test]
fn test_put_with_empty_source() {
    let mut buffer: Vec<u8> = Vec::with_capacity(10);
    let src: Vec<u8> = Vec::new();
    buffer.put(src);
}

#[test]
fn test_put_with_boundary_size() {
    let mut buffer: Vec<u8> = Vec::with_capacity(10);
    let src: Vec<u8> = vec![4, 5, 6, 7, 8, 9, 10];
    buffer.put(src);
}

#[test]
#[should_panic]
fn test_put_exceeding_capacity() {
    let mut buffer: Vec<u8> = Vec::with_capacity(3);
    let src: Vec<u8> = vec![1, 2, 3, 4, 5];
    buffer.put(src);
}

