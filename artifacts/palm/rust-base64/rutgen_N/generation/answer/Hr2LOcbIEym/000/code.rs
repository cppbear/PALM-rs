// Answer 0

#[derive(Debug)]
struct MockReader {
    data: Vec<u8>,
    position: usize,
}

impl MockReader {
    fn new(data: Vec<u8>) -> Self {
        MockReader { data, position: 0 }
    }
}

impl std::io::Read for MockReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let remaining = &self.data[self.position..];
        let len = remaining.len().min(buf.len());
        buf[..len].copy_from_slice(&remaining[..len]);
        self.position += len;
        Ok(len)
    }
}

struct MockEngine;

#[test]
fn test_decoder_creation_with_empty_reader() {
    let reader = MockReader::new(vec![]);
    let engine = &MockEngine;
    
    let decoder = new(reader, engine);
    
    assert!(decoder.inner.position == 0);
    assert!(decoder.b64_len == 0);
}

#[test]
fn test_decoder_creation_with_non_empty_reader() {
    let reader = MockReader::new(b"Hello, World!".to_vec());
    let engine = &MockEngine;
    
    let decoder = new(reader, engine);
    
    assert!(decoder.inner.position == 0);
    assert!(decoder.b64_len == 0);
}

