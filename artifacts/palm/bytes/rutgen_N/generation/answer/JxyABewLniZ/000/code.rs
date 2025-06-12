// Answer 0

#[test]
fn test_chunk_empty_vec_deque() {
    struct VecDeque {
        data: Vec<u8>,
    }

    impl VecDeque {
        fn new() -> Self {
            VecDeque { data: Vec::new() }
        }

        fn as_slices(&self) -> (&[u8], &[u8]) {
            if self.data.is_empty() {
                (&[], &[])
            } else {
                (&self.data, &[])
            }
        }

        fn chunk(&self) -> &[u8] {
            let (s1, s2) = self.as_slices();
            if s1.is_empty() {
                s2
            } else {
                s1
            }
        }
    }

    let vec_deque = VecDeque::new();
    assert_eq!(vec_deque.chunk(), &[]);
}

#[test]
fn test_chunk_non_empty_vec_deque() {
    struct VecDeque {
        data: Vec<u8>,
    }

    impl VecDeque {
        fn new(data: Vec<u8>) -> Self {
            VecDeque { data }
        }

        fn as_slices(&self) -> (&[u8], &[u8]) {
            if self.data.is_empty() {
                (&[], &[])
            } else {
                (&self.data, &[])
            }
        }

        fn chunk(&self) -> &[u8] {
            let (s1, s2) = self.as_slices();
            if s1.is_empty() {
                s2
            } else {
                s1
            }
        }
    }

    let vec_deque = VecDeque::new(vec![1, 2, 3]);
    assert_eq!(vec_deque.chunk(), [1, 2, 3]);
}

