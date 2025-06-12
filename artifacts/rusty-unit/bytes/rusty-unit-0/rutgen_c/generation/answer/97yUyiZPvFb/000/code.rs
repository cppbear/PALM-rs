// Answer 0

#[test]
fn test_last_mut() {
    struct DummyBuf {
        data: Vec<u8>,
        position: usize,
    }
    
    impl DummyBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
        
        fn advance(&mut self, n: usize) {
            self.position += n;
        }
        
        fn copy_to_bytes(&self, size: usize) -> Vec<u8> {
            self.data[self.position..self.position + size].to_vec()
        }
    }

    let mut buf1 = DummyBuf::new(b"hello ".to_vec());
    let mut buf2 = DummyBuf::new(b"world".to_vec());
    let mut chain = Chain::new(buf1, buf2);
    
    chain.last_mut().advance(1);
    let full = chain.last_mut().copy_to_bytes(5);
    assert_eq!(full, b"orld"[..]);
}

