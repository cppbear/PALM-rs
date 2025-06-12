// Answer 0

#[test]
fn test_first_mut() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            TestBuf { data }
        }

        fn advance(&mut self, n: usize) {
            self.data.drain(..n);
        }
        
        fn copy_to_bytes(&self, _len: usize) -> &[u8] {
            &self.data
        }
    }

    let mut a = TestBuf::new(b"hello".to_vec());
    let b = TestBuf::new(b"world".to_vec());
    let mut chain = Chain::new(a, b);

    chain.first_mut().advance(1);

    let full = chain.first_ref().copy_to_bytes(9);
    assert_eq!(full, b"elloworld"[..]);
}

