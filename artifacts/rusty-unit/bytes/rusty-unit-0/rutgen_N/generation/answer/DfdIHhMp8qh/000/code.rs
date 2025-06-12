// Answer 0

#[derive(Debug)]
struct MockBuf {
    data: Vec<u8>,
}

impl MockBuf {
    fn new(data: Vec<u8>) -> Self {
        MockBuf { data }
    }
}

impl bytes::Buf for MockBuf {
    fn remaining(&self) -> usize {
        self.data.len()
    }

    fn chunk(&self) -> &[u8] {
        &self.data
    }

    fn advance(&mut self, cnt: usize) {
        self.data.drain(0..cnt);
    }
}

#[test]
fn test_first_mut() {
    let mut buf_a = MockBuf::new(b"hello".to_vec());
    let mut buf_b = MockBuf::new(b"world".to_vec());
    
    let mut chain_buf = buf_a.chain(&mut buf_b);
    
    let first = chain_buf.first_mut();
    first.advance(1);
    
    let full = chain_buf.copy_to_bytes(9);
    assert_eq!(full, b"elloworld"[..]);
}

