// Answer 0

#[derive(Debug)]
struct BytesMut {
    data: Vec<u8>,
}

impl BytesMut {
    fn new() -> Self {
        BytesMut { data: Vec::new() }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn remaining(&self) -> usize {
        self.len()
    }
}

#[test]
fn test_remaining_empty() {
    let bytes = BytesMut::new();
    assert_eq!(bytes.remaining(), 0);
}

#[test]
fn test_remaining_non_empty() {
    let mut bytes = BytesMut::new();
    bytes.data.extend_from_slice(&[1, 2, 3]);
    assert_eq!(bytes.remaining(), 3);
}

#[test]
fn test_remaining_large() {
    let mut bytes = BytesMut::new();
    bytes.data.extend_from_slice(&vec![0; 1000]);
    assert_eq!(bytes.remaining(), 1000);
}

#[test]
fn test_remaining_after_clear() {
    let mut bytes = BytesMut::new();
    bytes.data.extend_from_slice(&[1, 2, 3]);
    bytes.data.clear();
    assert_eq!(bytes.remaining(), 0);
}

