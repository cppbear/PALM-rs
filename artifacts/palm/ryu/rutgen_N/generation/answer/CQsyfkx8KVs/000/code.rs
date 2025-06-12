// Answer 0

#[derive(Debug)]
struct Buffer {
    bytes: [std::mem::MaybeUninit<u8>; 24],
}

impl Buffer {
    pub fn new() -> Self {
        let bytes = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        Buffer { bytes }
    }
}

#[test]
fn test_buffer_new() {
    let buffer = Buffer::new();
    assert_eq!(buffer.bytes.len(), 24);
}

