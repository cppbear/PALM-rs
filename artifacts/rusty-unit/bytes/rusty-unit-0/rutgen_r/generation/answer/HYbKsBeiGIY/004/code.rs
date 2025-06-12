// Answer 0

#[test]
fn test_chunks_vectored_non_empty_slices() {
    struct TestVecDeque {
        data: Vec<u8>,
    }

    impl TestVecDeque {
        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn as_slices(&self) -> (&[u8], &[u8]) {
            let mid = self.data.len() / 2;
            (&self.data[..mid], &self.data[mid..])
        }
    }

    let vec_deque = TestVecDeque { data: vec![1, 2, 3, 4] };
    let mut io_slices = vec![std::io::IoSlice::new(&[]); 2];
    
    let result = vec_deque.chunks_vectored(&mut io_slices);
    
    assert_eq!(result, 2);
    assert_eq!(io_slices[0].len(), 2);
    assert_eq!(io_slices[1].len(), 2);
}

#[test]
fn test_chunks_vectored_single_slice_case() {
    struct TestVecDeque {
        data: Vec<u8>,
    }

    impl TestVecDeque {
        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn as_slices(&self) -> (&[u8], &[u8]) {
            (&self.data, &[])
        }
    }

    let vec_deque = TestVecDeque { data: vec![1, 2, 3, 4] };
    let mut io_slices = vec![std::io::IoSlice::new(&[]); 1];
    
    let result = vec_deque.chunks_vectored(&mut io_slices);
    
    assert_eq!(result, 1);
    assert_eq!(io_slices[0].len(), 4);
}

