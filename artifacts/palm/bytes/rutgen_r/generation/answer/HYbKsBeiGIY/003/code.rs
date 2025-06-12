// Answer 0

#[test]
fn test_chunks_vectored_single_slice() {
    use std::io;

    struct VecDeque {
        data: Vec<u8>,
    }

    impl VecDeque {
        fn new(data: Vec<u8>) -> Self {
            VecDeque { data }
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn as_slices(&self) -> (&[u8], &[u8]) {
            let (s1, s2) = self.data.split_at(self.data.len()); // This ensures s2 is empty
            (s1, s2)
        }
    }

    let vec_deque = VecDeque::new(vec![1, 2, 3]);
    let mut io_slices: [io::IoSlice; 2] = Default::default(); // Pre-allocated buffer

    let result = vec_deque.chunks_vectored(&mut io_slices);

    assert_eq!(result, 1);
    assert_eq!(io_slices[0].len(), 3);
    assert_eq!(io_slices[1].len(), 0); // s2 is empty
}

