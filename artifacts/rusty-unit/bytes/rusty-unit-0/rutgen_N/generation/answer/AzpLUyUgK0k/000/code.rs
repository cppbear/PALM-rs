// Answer 0

#[test]
fn test_chunk_mut() {
    struct TestBufMut {
        data: Vec<u8>,
    }

    impl TestBufMut {
        fn new(data: Vec<u8>) -> Self {
            TestBufMut { data }
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.data
        }
    }

    let mut buf = TestBufMut::new(vec![1, 2, 3, 4, 5]);
    let chunk = buf.chunk_mut();

    assert_eq!(chunk.len(), 5);
    chunk[0] = 10;
    assert_eq!(chunk[0], 10);
}

