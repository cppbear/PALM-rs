// Answer 0

#[test]
fn test_chunks_vectored() {
    struct TestBuffer {
        data: Vec<u8>,
    }

    impl TestBuffer {
        fn new(data: Vec<u8>) -> Self {
            Self { data }
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn as_slices(&self) -> (&[u8], &[u8]) {
            let mid = self.data.len() / 2;
            self.data.split_at(mid)
        }
    }

    let buffer = TestBuffer::new(vec![1, 2, 3, 4]);
    
    let mut dst: [std::io::IoSlice; 2] = Default::default();
    let result = buffer.chunks_vectored(&mut dst);

    assert_eq!(result, 2);
    assert_eq!(dst[0].as_ref(), &[1, 2]);
    assert_eq!(dst[1].as_ref(), &[3, 4]);
}

