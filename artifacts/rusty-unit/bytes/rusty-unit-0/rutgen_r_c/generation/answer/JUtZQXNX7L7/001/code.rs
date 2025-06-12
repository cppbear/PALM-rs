// Answer 0

#[derive(Debug)]
struct TestBuf {
    data: Vec<u8>,
    position: usize,
}

impl TestBuf {
    fn new(data: Vec<u8>) -> Self {
        TestBuf { data, position: 0 }
    }
}

impl Buf for TestBuf {
    fn remaining(&self) -> usize {
        self.data.len() - self.position
    }

    fn chunk(&self) -> &[u8] {
        &self.data[self.position..]
    }

    fn advance(&mut self, cnt: usize) {
        self.position = cmp::min(self.position + cnt, self.data.len());
    }

    fn has_remaining(&self) -> bool {
        self.position < self.data.len()
    }

    fn copy_to_slice(&mut self, dst: &mut [u8]) {
        let len = cmp::min(dst.len(), self.remaining());
        dst[..len].copy_from_slice(&self.data[self.position..self.position + len]);
        self.advance(len);
    }

    fn get_u8(&mut self) -> u8 {
        self.data[self.position] 
    }

    // Other trait methods can be left unimplemented for this test
}

#[test]
fn test_fill_buf() {
    let buf_data = vec![1, 2, 3, 4, 5];
    let mut test_buf = TestBuf::new(buf_data);
    let mut reader = Reader { buf: test_buf };

    let result = reader.fill_buf();
    assert!(result.is_ok());
    let chunk = result.unwrap();
    assert_eq!(chunk, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_fill_buf_empty() {
    let buf_data: Vec<u8> = Vec::new();
    let mut test_buf = TestBuf::new(buf_data);
    let mut reader = Reader { buf: test_buf };

    let result = reader.fill_buf();
    assert!(result.is_ok());
    let chunk = result.unwrap();
    assert!(chunk.is_empty());
}

