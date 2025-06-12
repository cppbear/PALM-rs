// Answer 0

#[derive(Default)]
struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    fn chunk(&self) -> &[u8] {
        &self.data
    }
}

#[test]
fn test_chunk_non_empty() {
    let buffer = Buffer { data: vec![1, 2, 3, 4, 5] };
    let chunk = buffer.chunk();
    assert_eq!(chunk, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_chunk_empty() {
    let buffer = Buffer::default();
    let chunk = buffer.chunk();
    assert_eq!(chunk, &[]);
}

