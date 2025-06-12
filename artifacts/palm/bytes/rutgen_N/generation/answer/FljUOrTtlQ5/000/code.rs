// Answer 0

#[derive(Debug)]
struct MockBuf {
    data: Vec<u8>,
    position: usize,
}

impl MockBuf {
    fn new(data: Vec<u8>) -> Self {
        Self { data, position: 0 }
    }

    fn has_remaining(&self) -> bool {
        self.position < self.data.len()
    }

    fn chunk(&mut self) -> &[u8] {
        let chunk = &self.data[self.position..];
        if let Some(end) = chunk.iter().position(|&x| x == 0) {
            &chunk[..end]
        } else {
            self.position = self.data.len(); // Advance to the end
            chunk
        }
    }

    fn advance(&mut self, count: usize) {
        self.position += count;
    }
}

trait Buf {
    fn has_remaining(&self) -> bool;
    fn chunk(&mut self) -> &[u8];
    fn advance(&mut self, count: usize);
}

impl Buf for MockBuf {
    fn has_remaining(&self) -> bool {
        MockBuf::has_remaining(self)
    }

    fn chunk(&mut self) -> &[u8] {
        MockBuf::chunk(self)
    }

    fn advance(&mut self, count: usize) {
        MockBuf::advance(self, count)
    }
}

struct ByteBuffer {
    buffer: Vec<u8>,
}

impl ByteBuffer {
    fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    fn extend_from_slice(&mut self, slice: &[u8]) {
        self.buffer.extend_from_slice(slice);
    }

    fn put<T: Buf>(&mut self, mut src: T)
    where
        Self: Sized,
    {
        while src.has_remaining() {
            let s = src.chunk();
            let l = s.len();
            self.extend_from_slice(s);
            src.advance(l);
        }
    }

    fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }
}

#[test]
fn test_put_with_empty_buf() {
    let mut byte_buffer = ByteBuffer::new();
    let mock_buf = MockBuf::new(vec![]);
    byte_buffer.put(mock_buf);
    assert_eq!(byte_buffer.get_buffer(), &[]);
}

#[test]
fn test_put_with_single_chunk() {
    let mut byte_buffer = ByteBuffer::new();
    let mock_buf = MockBuf::new(vec![1, 2, 3]);
    byte_buffer.put(mock_buf);
    assert_eq!(byte_buffer.get_buffer(), &[1, 2, 3]);
}

#[test]
fn test_put_with_multiple_chunks() {
    let mut byte_buffer = ByteBuffer::new();
    let mock_buf = MockBuf::new(vec![1, 2, 3, 4, 5, 6]);
    byte_buffer.put(mock_buf);
    assert_eq!(byte_buffer.get_buffer(), &[1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_put_with_boundary_condition() {
    let mut byte_buffer = ByteBuffer::new();
    let mock_buf = MockBuf::new(vec![0, 1, 2, 3, 4]); // zero-byte case
    byte_buffer.put(mock_buf);
    assert_eq!(byte_buffer.get_buffer(), &[0, 1, 2, 3, 4]);
}

